//! #   rCovid
//!                         rcdareastatwindow.rs
//!                         -------------------------------------
//!     begin               2022/05/17
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use egui::{self, Color32, Direction, RichText, WidgetText, Window};
use egui_extras::{Size, TableBuilder};
use json::JsonValue;
use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;

use rcovid_core;
use crate::{CityStat, DangerArea, ProvinceStat, rcdprovincedetailwindow};

#[derive(Default)]
pub struct RcdAreaStatWindow {
    province_detail_map: HashMap<i32, bool>,
    provinces_stat: LinkedHashMap<i32, ProvinceStat>,
    province_detail_open: bool,
    province_detail_id: Option<i32>,
    privince_detail_window: rcdprovincedetailwindow::RcdProvinceDetailWindow,
}

impl super::Window for RcdAreaStatWindow {
    fn name(&self) -> &'static str {
        "☀ 国内疫情"
    }

    fn window_type(&self) -> rcovid_core::CovidDataType {
        rcovid_core::CovidDataType::AreaStat
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, data: Option<&JsonValue>) {
        Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View as _;
            self.ui(ui, data);
        });
    }
}

impl super::View for RcdAreaStatWindow {
    fn ui(&mut self, ui: &mut egui::Ui, data: Option<&JsonValue>) {
        if self.provinces_stat.len() <= 0 {
            // let mut provinces_stat = Vec::new();
            if let Some(json_value) = data {
                if json_value.is_array() {
                    let members = json_value.members();
                    for member in members {
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
                                    danger_level: danger_area_val["dangerLevel"].as_u8().unwrap_or(0),
                                };

                                danger_areas.push(danger_area);
                            }
                        }

                        let province_stat = ProvinceStat {
                            name: member["provinceName"].as_str().unwrap_or("").to_string(),
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

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(48.0).at_least(32.0))
            .column(Size::initial(64.0).at_least(48.0))
            .column(Size::initial(64.0).at_least(48.0))
            .column(Size::initial(64.0).at_least(32.0))
            .column(Size::initial(64.0).at_least(32.0))
            .column(Size::initial(64.0).at_least(32.0))
            .column(Size::initial(64.0).at_least(32.0))
            .column(Size::initial(16.0).at_least(16.0))
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
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.heading(RichText::new("疑似")); // .background_color(Color32::from_rgb(227, 231, 243)));
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
                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                ui.label(province_stat.suspected_count.to_string());
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

        self.privince_detail_window.show(ui.ctx(), &mut self.province_detail_open,
                                         if self.province_detail_id.is_some() { self.provinces_stat.get(&self.province_detail_id.unwrap()) } else { None });
        if !self.province_detail_open && self.province_detail_id.is_some() {
            *self.province_detail_map.get_mut(&self.province_detail_id.unwrap()).unwrap() = false;
        }
    }
}
