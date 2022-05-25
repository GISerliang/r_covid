//! #   rCovid
//!                         rcdrecentstatv2window.rs
//!                         -------------------------------------
//!     begin               2022/05/24
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;
use egui::{Context, Direction, Hyperlink, RichText, Ui, Window};
use json::JsonValue;
use linked_hash_map::LinkedHashMap;

use rcovid_core::CovidDataType;

struct RecentCityStat {
    // 名称
    pub name: String,
    // 现存确诊
    pub current_confirmed_count: i64,
    // 累计确诊
    pub confirmed_count: i64,
    // 本土新增
    pub yesterday_local_confirmed_count: i64,
    // 本土无症状
    pub yesterday_asymptomatic_count: i64,
    // 新增风险地区
    pub danger_count_incr: u32,
    // 地区代码
    pub location_id: i32,
    // 风险地区
    pub current_danger_count: u32,
}

struct RecentProvinceStat {
    // 名称
    pub name: String,
    // 简称
    pub short_name: String,
    // 现存确诊
    pub current_confirmed_count: i64,
    // 累计确诊
    pub confirmed_count: i64,
    // 本土新增
    pub yesterday_local_confirmed_count: i64,
    // 本土无症状
    pub yesterday_asymptomatic_count: i64,
    // 新增风险地区
    pub danger_count_incr: u32,
    // 地区代码
    pub location_id: i32,
    // JSON统计数据
    pub statistic_data_uri: String,
    // 风险地区
    pub current_danger_count: u32,
    // 市/区情况
    pub cities: Vec<RecentCityStat>,
}

#[derive(Default)]
pub struct RcdRecentStatV2Window {
    provinces_stat: LinkedHashMap<i32, RecentProvinceStat>,
    province_detail_map: HashMap<i32, bool>,
    province_detail_open: bool,
    province_detail_id: Option<i32>,
}

impl super::Window for RcdRecentStatV2Window {
    fn name(&self) -> &'static str {
        "💽 近期疫情"
    }

    fn window_type(&self) -> CovidDataType {
        CovidDataType::RecentStatV2
    }

    fn show(&mut self, ctx: &Context, open: &mut bool, data: Option<&JsonValue>) {
        Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View as _;
            self.ui(ui, data);
        });

        if let Some(id) = self.province_detail_id {
            if let Some(province_stat) = self.provinces_stat.get(&id) {
                Window::new(format!("{} 近期疫情详情", province_stat.short_name.as_str()).as_str())
                    .open(&mut self.province_detail_open)
                    .scroll2([true; 2])
                    .min_width(560.)
                    .show(ctx, |ui| {
                        use egui_extras::{TableBuilder, Size};

                        ui.vertical(|ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .resizable(true)
                                .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                                .column(Size::initial(56.0).at_least(32.0))
                                .column(Size::initial(64.0).at_least(48.0))
                                .column(Size::initial(72.0).at_least(56.0))
                                .column(Size::initial(64.0).at_least(32.0))
                                .column(Size::initial(64.0).at_least(32.0))
                                .column(Size::initial(88.0).at_least(80.0))
                                .column(Size::initial(64.0).at_least(56.0))
                                .header(32., |mut header| {
                                    header.col(|ui| {
                                        ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                            ui.heading(RichText::new("地区"));
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.heading(RichText::new("本土新增"));
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.heading(RichText::new("本土无症状"));
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.heading(RichText::new("现存确诊"));
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.heading(RichText::new("累计确诊"));
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.heading(RichText::new("新增风险地区"));
                                        });
                                    });
                                    header.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.heading(RichText::new("风险地区"));
                                        });
                                    });
                                })
                                .body(|mut body| {
                                    body.row(30., |mut row| {
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                                ui.strong(RichText::new(province_stat.short_name.as_str()).size(20.));
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.strong(RichText::new(province_stat.yesterday_local_confirmed_count.to_string()).size(20.));
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.strong(RichText::new(province_stat.yesterday_asymptomatic_count.to_string()).size(20.));
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.strong(RichText::new(province_stat.current_confirmed_count.to_string()).size(20.));
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.strong(RichText::new(province_stat.confirmed_count.to_string()).size(20.));
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.strong(RichText::new(province_stat.danger_count_incr.to_string()).size(20.));
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.strong(RichText::new(province_stat.current_danger_count.to_string()).size(20.));
                                            });
                                        });
                                    });

                                    for city_stat in &province_stat.cities {
                                        body.row(30., |mut row| {
                                            row.col(|ui| {
                                                ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                                    ui.label(city_stat.name.as_str());
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                    ui.label(city_stat.yesterday_local_confirmed_count.to_string());
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                    ui.label(city_stat.yesterday_asymptomatic_count.to_string());
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                    ui.label(city_stat.current_confirmed_count.to_string());
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                    ui.label(city_stat.confirmed_count.to_string());
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                    ui.label(city_stat.danger_count_incr.to_string());
                                                });
                                            });
                                            row.col(|ui| {
                                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                    ui.label(city_stat.current_danger_count.to_string());
                                                });
                                            });
                                        });
                                    }
                                });

                            if !province_stat.statistic_data_uri.is_empty() {
                                ui.separator();
                                ui.add(Hyperlink::from_label_and_url("JSON统计数据", province_stat.statistic_data_uri.as_str()));
                            }
                        });
                    });
            }
        }
        if !self.province_detail_open && self.province_detail_id.is_some() {
            *self.province_detail_map.get_mut(&self.province_detail_id.unwrap()).unwrap() = false;
        }
    }
}

impl super::View for RcdRecentStatV2Window {
    fn ui(&mut self, ui: &mut Ui, data: Option<&JsonValue>) {
        use egui_extras::{TableBuilder, Size};

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
                                let city = RecentCityStat {
                                    name: city_val["cityName"].as_str().unwrap_or("").to_string(),
                                    current_confirmed_count: city_val["currentConfirmedCount"].as_i64().unwrap_or(0),
                                    confirmed_count: city_val["confirmedCount"].as_i64().unwrap_or(0),
                                    yesterday_local_confirmed_count: city_val["yesterdayLocalConfirmedCount"].as_i64().unwrap_or(0),
                                    yesterday_asymptomatic_count: city_val["yesterdayAsymptomaticCount"].as_i64().unwrap_or(0),
                                    danger_count_incr: city_val["dangerCountIncr"].as_u32().unwrap_or(0),
                                    location_id: city_val["locationId"].as_i32().unwrap_or(0),
                                    current_danger_count: city_val["currentDangerCount"].as_u32().unwrap_or(0),
                                };
                                cities.push(city);
                            }
                        }

                        let province_stat = RecentProvinceStat {
                            name: province_name,
                            short_name: member["provinceShortName"].as_str().unwrap_or("").to_string(),
                            current_confirmed_count: member["currentConfirmedCount"].as_i64().unwrap_or(0),
                            confirmed_count: member["confirmedCount"].as_i64().unwrap_or(0),
                            yesterday_local_confirmed_count: member["yesterdayLocalConfirmedCount"].as_i64().unwrap_or(0),
                            yesterday_asymptomatic_count: member["yesterdayAsymptomaticCount"].as_i64().unwrap_or(0),
                            danger_count_incr: member["dangerCountIncr"].as_u32().unwrap_or(0),
                            location_id: member["locationId"].as_i32().unwrap_or(0),
                            statistic_data_uri: member["statisticsData"].as_str().unwrap_or("").to_string(),
                            current_danger_count: member["currentDangerCount"].as_u32().unwrap_or(0),
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
            .column(Size::initial(72.0).at_least(64.0))
            .column(Size::initial(64.0).at_least(32.0))
            // .column(Size::initial(72.0).at_least(64.0))
            .column(Size::initial(64.0).at_least(32.0))
            .column(Size::initial(32.0).at_least(32.0))
            .header(32., |mut header| {
                header.col(|ui| {
                    ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                        ui.heading(RichText::new("地区"));
                    });
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.heading(RichText::new("本土新增"));
                    });
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.heading(RichText::new("本土无症状"));
                    });
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.heading(RichText::new("现存确诊"));
                    });
                });
                // header.col(|ui| {
                //     ui.with_layout(egui::Layout::right_to_left(), |ui| {
                //         ui.heading(RichText::new("新增风险地区"));
                //     });
                // });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.heading(RichText::new("风险地区"));
                    });
                });
                header.col(|ui| {
                    ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                        ui.heading(RichText::new("详细"));
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
                                ui.label(province_stat.yesterday_local_confirmed_count.to_string());
                            });
                        });
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                ui.label(province_stat.yesterday_asymptomatic_count.to_string());
                            });
                        });
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                ui.label(province_stat.current_confirmed_count.to_string());
                            });
                        });
                        // row.col(|ui| {
                        //     ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        //         ui.label(province_stat.danger_count_incr.to_string());
                        //     });
                        // });
                        row.col(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                ui.label(province_stat.current_danger_count.to_string());
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
    }
}