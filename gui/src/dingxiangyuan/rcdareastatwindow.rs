//! #   rCovid
//!                         rcdareastatwindow.rs
//!                         -------------------------------------
//!     begin               2022/05/17
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use egui::{self, Direction, Layout, RichText, Window};
use egui_extras::{Size, TableBuilder};
use json::JsonValue;
use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;

use rcovid_core;
use crate::dingxiangyuan::{CityStat, DangerArea, ProvinceStat, rcdprovincedetailwindow};

#[derive(Debug)]
struct NationalStatistics {
    // 现存确诊
    pub current_confirmed_count: i64,
    pub current_confirmed_incr: i64,
    // 累计确诊
    pub confirmed_count: i64,
    pub confirmed_incr: i64,
    // 死亡
    pub dead_count: i64,
    pub dead_incr: i64,
    // 治愈
    pub cured_count: i64,
    pub cured_incr: i64,
    // 无症状
    pub serious_count: i64,
    pub serious_incr: i64,
    // 境外输入
    pub suspected_count: i64,
    pub suspected_incr: i64,
    pub statistic_datetime: chrono::DateTime<chrono::Utc>,
}

#[derive(Default)]
pub struct RcdAreaStatWindow {
    province_detail_map: HashMap<i32, bool>,
    provinces_stat: LinkedHashMap<i32, ProvinceStat>,
    province_detail_open: bool,
    province_detail_id: Option<i32>,
    privince_detail_window: rcdprovincedetailwindow::RcdProvinceDetailWindow,
    high_danger_areas: LinkedHashMap<String, Vec<DangerArea>>,
    high_danger_area_count: u32,
    mid_danger_areas: LinkedHashMap<String, Vec<DangerArea>>,
    mid_danger_area_count: u32,
    danger_areas_open: bool,
    national_statistics: Option<NationalStatistics>,
}

impl super::Window for RcdAreaStatWindow {
    fn name(&self) -> &'static str {
        "☀ 国内疫情"
    }

    fn window_type(&self) -> rcovid_core::CovidDataType {
        rcovid_core::CovidDataType::AreaStat
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) {
        Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View as _;
            self.ui(ui, data, statistics_data);
        });
    }
}

impl super::View for RcdAreaStatWindow {
    fn ui(&mut self, ui: &mut egui::Ui, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) {
        if self.provinces_stat.len() <= 0 {
            if let Some(json_value) = data {
                if json_value.is_array() {
                    let members = json_value.members();
                    for member in members {
                        let province_name = member["provinceName"].as_str().unwrap_or("").to_string();
                        let mut cities = Vec::new();
                        let cities_val = &member["cities"];
                        if !cities_val.is_null() && cities_val.is_array() && cities_val.len() > 0 {
                            for city_val in cities_val.members() {
                                let city = CityStat {
                                    name: city_val["cityName"].as_str().unwrap_or("").to_string(),
                                    current_confirmed_count: city_val["currentConfirmedCount"].as_i64().unwrap_or(0),
                                    confirmed_count: city_val["confirmedCount"].as_i64().unwrap_or(0),
                                    dead_count: city_val["deadCount"].as_i64().unwrap_or(0),
                                    cured_count: city_val["curedCount"].as_i64().unwrap_or(0),
                                    suspected_count: city_val["suspectedCount"].as_i64().unwrap_or(0),
                                    location_id: city_val["locationId"].as_i32().unwrap_or(0),
                                    high_danger_count: city_val["highDangerCount"].as_u32().unwrap_or(0),
                                    mid_danger_count: city_val["midDangerCount"].as_u32().unwrap_or(0),
                                };
                                cities.push(city);
                            }
                        }

                        let mut danger_areas = Vec::new();
                        let danger_area_val = &member["dangerAreas"];
                        if !danger_area_val.is_null() && danger_area_val.is_array() && danger_area_val.len() > 0 {
                            for danger_area_val in danger_area_val.members() {
                                let danger_area = DangerArea {
                                    city_name: danger_area_val["cityName"].as_str().unwrap_or("").to_string(),
                                    area_name: danger_area_val["areaName"].as_str().unwrap_or("").to_string(),
                                    danger_level: danger_area_val["dangerLevel"].as_u8().unwrap_or(3),
                                };

                                if danger_area.danger_level == 2 {
                                    self.mid_danger_area_count += 1;
                                    if let Some(areas) = self.mid_danger_areas.get_mut(province_name.as_str()) {
                                        areas.push(danger_area.clone());
                                    } else {
                                        self.mid_danger_areas.insert(province_name.clone(), vec![danger_area.clone()]);
                                    }
                                } else if danger_area.danger_level == 1 {
                                    self.high_danger_area_count += 1;
                                    if let Some(areas) = self.high_danger_areas.get_mut(province_name.as_str()) {
                                        areas.push(danger_area.clone());
                                    } else {
                                        self.high_danger_areas.insert(province_name.clone(), vec![danger_area.clone()]);
                                    }
                                }
                                danger_areas.push(danger_area);
                            }
                        }

                        let province_stat = ProvinceStat {
                            name: province_name,
                            short_name: member["provinceShortName"].as_str().unwrap_or("").to_string(),
                            current_confirmed_count: member["currentConfirmedCount"].as_i64().unwrap_or(0),
                            confirmed_count: member["confirmedCount"].as_i64().unwrap_or(0),
                            dead_count: member["deadCount"].as_i64().unwrap_or(0),
                            cured_count: member["curedCount"].as_i64().unwrap_or(0),
                            suspected_count: member["suspectedCount"].as_i64().unwrap_or(0),
                            location_id: member["locationId"].as_i32().unwrap_or(0),
                            comment: member["comment"].as_str().unwrap_or("").to_string(),
                            statistic_data_uri: member["statisticsData"].as_str().unwrap_or("").to_string(),
                            high_danger_count: member["highDangerCount"].as_u32().unwrap_or(0),
                            mid_danger_count: member["midDangerCount"].as_u32().unwrap_or(0),
                            detect_org_count: member["detectOrgCount"].as_u32().unwrap_or(0),
                            vaccination_org_count: member["vaccinationOrgCount"].as_u32().unwrap_or(0),
                            danger_areas,
                            cities,
                        };

                        self.provinces_stat.insert(province_stat.location_id, province_stat);
                    }
                }
            }
        }

        if self.national_statistics.is_none() {
            if let Some(json_value) = statistics_data {
                if json_value.is_object() {
                    let timestamp = json_value["modifyTime"].as_i64().unwrap_or(0);
                    let datetime = format!("{}",
                                           chrono::NaiveDateTime::from_timestamp(timestamp / 1000,
                                                                                 (timestamp % 1000) as u32).
                                               format("%Y-%m-%d %H:%M:%SZ")).as_str().parse::<chrono::DateTime<chrono::Utc>>().unwrap();

                    self.national_statistics = Some(NationalStatistics {
                        current_confirmed_count: json_value["currentConfirmedCount"].as_i64().unwrap_or(0),
                        confirmed_count: json_value["confirmedCount"].as_i64().unwrap_or(0),
                        dead_count: json_value["deadCount"].as_i64().unwrap_or(0),
                        cured_count: json_value["curedCount"].as_i64().unwrap_or(0),
                        current_confirmed_incr: json_value["currentConfirmedIncr"].as_i64().unwrap_or(0),
                        confirmed_incr: json_value["confirmedIncr"].as_i64().unwrap_or(0),
                        cured_incr: json_value["curedIncr"].as_i64().unwrap_or(0),
                        serious_count: json_value["seriousCount"].as_i64().unwrap_or(0),
                        serious_incr: json_value["seriousIncr"].as_i64().unwrap_or(0),
                        suspected_count: json_value["suspectedCount"].as_i64().unwrap_or(0),
                        dead_incr: json_value["deadIncr"].as_i64().unwrap_or(0),
                        statistic_datetime: datetime,
                        suspected_incr: json_value["suspectedIncr"].as_i64().unwrap_or(0),
                    });
                }
            }
        }

        ui.vertical(|ui| {
            if let Some(national_stat) = &self.national_statistics {
                let china_timezone = chrono::FixedOffset::east(8 * 3600);
                ui.code(format!("截止北京时间 {}", national_stat.statistic_datetime.with_timezone(&china_timezone).format("%Y-%m-%d %H:%M")));

                ui.separator();
            }

            if ui.selectable_label(self.danger_areas_open,
                                   RichText::new(format!("高风险地区 {} 个， 中风险地区 {} 个", self.high_danger_area_count, self.mid_danger_area_count).as_str())).clicked() {
                self.danger_areas_open = !self.danger_areas_open;
            }
            ui.separator();

            if let Some(national_stat) = &self.national_statistics {
                TableBuilder::new(ui)
                    .striped(true)
                    .column(Size::initial(80.).at_least(80.))
                    .column(Size::initial(80.).at_least(80.))
                    .column(Size::initial(80.).at_least(80.))
                    .body(|mut body| {
                        body.row(12., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("较昨日{}{}", if national_stat.current_confirmed_incr >= 0 { "+" } else { "-" },
                                                                   national_stat.current_confirmed_incr.abs()).as_str()).size(12.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("较昨日{}{}", if national_stat.suspected_incr >= 0 { "+" } else { "-" },
                                                                   national_stat.suspected_incr.abs()).as_str()).size(12.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("较昨日{}{}", if national_stat.serious_incr >= 0 { "+" } else { "-" },
                                                                   national_stat.serious_incr.abs()).as_str()).size(12.));
                                });
                            });
                        });
                        body.row(20., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", national_stat.current_confirmed_count).as_str()));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", national_stat.suspected_count).as_str()));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", national_stat.serious_count).as_str()));
                                });
                            });
                        });
                        body.row(14., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new("现存确诊").size(14.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new("境外输入").size(14.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new("现存无症状").size(14.));
                                });
                            });
                        });
                        body.row(10., |mut row| {
                            row.col(|ui| {});
                            row.col(|ui| {});
                            row.col(|ui| {});
                        });
                        body.row(12., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("较昨日{}{}", if national_stat.confirmed_incr >= 0 { "+" } else { "-" },
                                                                   national_stat.confirmed_incr.abs()).as_str()).size(12.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("较昨日{}{}", if national_stat.dead_incr >= 0 { "+" } else { "-" },
                                                                   national_stat.dead_incr.abs()).as_str()).size(12.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("较昨日{}{}", if national_stat.cured_incr >= 0 { "+" } else { "-" },
                                                                   national_stat.cured_incr.abs()).as_str()).size(12.));
                                });
                            });
                        });
                        body.row(20., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", national_stat.confirmed_count).as_str()));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", national_stat.dead_count).as_str()));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", national_stat.cured_count).as_str()));
                                });
                            });
                        });
                        body.row(14., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new("累计确诊").size(14.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new("累计死亡").size(14.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new("累计治愈").size(14.));
                                });
                            });
                        });
                    });

                ui.separator();
            }

            egui::CollapsingHeader::new("")
                .default_open(true)
                .show(ui, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                        .column(Size::initial(48.0).at_least(32.0))
                        .column(Size::initial(64.0).at_least(48.0))
                        .column(Size::initial(64.0).at_least(48.0))
                        .column(Size::initial(64.0).at_least(32.0))
                        .column(Size::initial(64.0).at_least(32.0))
                        .column(Size::initial(32.0).at_least(32.0))
                        .header(32., |mut header| {
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.heading(RichText::new("地区")); // .background_color(Color32::from_rgb(227, 231, 243))
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                    ui.heading(RichText::new("现存确诊")); //.background_color(Color32::from_rgb(243, 186, 176)));
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                    ui.heading(RichText::new("累计确诊")); //.background_color(Color32::from_rgb(230, 154, 141)));
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                    ui.heading(RichText::new("死亡")); //.background_color(Color32::from_rgb(180, 192, 213)));
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                    ui.heading(RichText::new("治愈")); //.background_color(Color32::from_rgb(149, 219, 154)));
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.heading(RichText::new("详细")); // .background_color(Color32::from_rgb(227, 231, 243)));
                                });
                            });
                        })
                        .body(|mut body| {
                            for (location_id, province_stat) in &self.provinces_stat {
                                body.row(30., |mut row| {
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                            ui.label(province_stat.short_name.as_str());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(province_stat.current_confirmed_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(province_stat.confirmed_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(province_stat.dead_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(province_stat.cured_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                            if ui.selectable_label(*self.province_detail_map.entry(*location_id).or_insert(false), "☞").clicked() {
                                                for (id, value) in self.province_detail_map.iter_mut() {
                                                    if *id == *location_id {
                                                        continue;
                                                    }
                                                    *value = false;
                                                }

                                                let selected = self.province_detail_map.entry(*location_id).or_insert(false);
                                                *selected = !(*selected);
                                                self.province_detail_open = *selected;

                                                if *selected {
                                                    self.province_detail_id = Some(*location_id);
                                                }
                                            }
                                        });
                                    });
                                });
                            }
                        });
                });
        });

        self.privince_detail_window.show(ui.ctx(), &mut self.province_detail_open,
                                         if self.province_detail_id.is_some() { self.provinces_stat.get(&self.province_detail_id.unwrap()) } else { None });
        if !self.province_detail_open && self.province_detail_id.is_some() {
            *self.province_detail_map.get_mut(&self.province_detail_id.unwrap()).unwrap() = false;
        }

        Window::new("风险地区详情").open(&mut self.danger_areas_open).scroll2([true; 2]).show(ui.ctx(), |ui| {
            egui::CollapsingHeader::new("高风险地区").default_open(true).show(ui, |ui| {
                egui::Grid::new("high_danger_areas").show(ui, |ui| {
                    for (province_name, danger_areas) in &self.high_danger_areas {
                        ui.label(province_name.as_str());
                        egui::Grid::new(format!("detail_high_danger_area_{}", province_name.as_str()).as_str()).show(ui, |ui| {
                            for danger_area in danger_areas {
                                ui.label(danger_area.city_name.as_str());
                                ui.label(danger_area.area_name.as_str());
                                ui.end_row();
                            }
                        });
                        ui.end_row();
                    }
                });
            });

            ui.separator();

            egui::CollapsingHeader::new("中风险地区").show(ui, |ui| {
                egui::Grid::new("mid_danger_areas").show(ui, |ui| {
                    for (province_name, danger_areas) in &self.mid_danger_areas {
                        ui.label(province_name.as_str());
                        egui::Grid::new(format!("detail_mid_danger_area_{}", province_name.as_str()).as_str()).show(ui, |ui| {
                            for danger_area in danger_areas {
                                ui.label(danger_area.city_name.as_str());
                                ui.label(danger_area.area_name.as_str());
                                ui.end_row();
                            }
                        });
                        ui.end_row();
                    }
                });
            });
        });
    }
}
