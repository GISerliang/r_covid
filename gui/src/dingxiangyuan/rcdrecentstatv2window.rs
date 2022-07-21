//! #   rCovid
//!                         rcdrecentstatv2window.rs
//!                         -------------------------------------
//!     begin               2022/05/24
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use std::collections::{BTreeMap, HashMap};
use std::ops::RangeInclusive;
use egui::{Context, Direction, Hyperlink, Response, RichText, Ui, widgets, Window};
use egui::plot::{Bar, BarChart, Legend, Line, Plot};
use json::JsonValue;
use linked_hash_map::LinkedHashMap;

use rcovid_core::CovidDataType;

#[derive(PartialEq, Eq)]
enum ChartType {
    YesterdayChart,
    ConfirmedChart,
}

impl Default for ChartType {
    fn default() -> Self {
        Self::YesterdayChart
    }
}

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
    chart_type: ChartType,
}

impl super::Window for RcdRecentStatV2Window {
    fn name(&self) -> &'static str {
        "💽 近期疫情"
    }

    fn window_type(&self) -> CovidDataType {
        CovidDataType::RecentStatV2
    }

    fn show(&mut self, ctx: &Context, open: &mut bool, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) {
        Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View as _;
            self.ui(ui, data, statistics_data);
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
    fn ui(&mut self, ui: &mut Ui, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) {
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

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.chart_type, ChartType::YesterdayChart, "本土新增和本土无症状");
            ui.selectable_value(&mut self.chart_type, ChartType::ConfirmedChart, "现存确诊");
        });
        match self.chart_type {
            ChartType::YesterdayChart => self.plot_yesterday(ui),
            ChartType::ConfirmedChart => self.plot_confirmed(ui)
        };

        ui.separator();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(48.0).at_least(32.0))
            .column(Size::initial(64.0).at_least(48.0))
            .column(Size::initial(72.0).at_least(64.0))
            .column(Size::initial(64.0).at_least(32.0))
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

impl RcdRecentStatV2Window {
    fn plot_yesterday(&mut self, ui: &mut Ui) -> Response {
        let mut yesterday_asymptomatic_bars = Vec::new();
        let mut yesterday_local_confirmed_bars = Vec::new();
        let mut names = Vec::new();

        let mut i = 0.25;
        for (_, province_stat) in &self.provinces_stat {
            if province_stat.yesterday_local_confirmed_count == 0 && province_stat.yesterday_asymptomatic_count == 0 {
                continue;
            }

            yesterday_asymptomatic_bars.push(Bar::new(i,
                                                      province_stat.yesterday_asymptomatic_count as f64)
                .name(province_stat.short_name.as_str()));
            yesterday_local_confirmed_bars.push(Bar::new(i,
                                                         province_stat.yesterday_local_confirmed_count as f64)
                .name(province_stat.short_name.as_str()));
            names.push(String::from(province_stat.short_name.as_str()));
            i += 1.;
        }
        let mut yesterday_asymptomatic_chart = BarChart::new(yesterday_asymptomatic_bars)
            .element_formatter(Box::new(recent_chart_label))
            .width(0.5)
            .name("本土无症状");

        let mut yesterday_local_confirmed_chart = BarChart::new(yesterday_local_confirmed_bars)
            .element_formatter(Box::new(recent_chart_label))
            .width(0.5)
            .name("本土新增")
            .stack_on(&[&yesterday_asymptomatic_chart]);

        let x_fmt = move |x: f64, _range: &RangeInclusive<f64>| {
            if let Some(name) = names.get(x.floor() as usize) {
                format!("{}", name.as_str())
            } else {
                String::new()
            }
        };

        Plot::new("plot_yesterday")
            .legend(Legend::default())
            .height(160.)
            .x_axis_formatter(x_fmt)
            .y_axis_formatter(|y, _range| {
                String::new()
            })
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(yesterday_asymptomatic_chart);
                plot_ui.bar_chart(yesterday_local_confirmed_chart);
            })
            .response
    }

    fn plot_confirmed(&mut self, ui: &mut Ui) -> Response {
        let mut confirmed_values = BTreeMap::new();
        for (_, province_stat) in &self.provinces_stat {
            confirmed_values.insert(province_stat.current_confirmed_count, province_stat.short_name.as_str());
        }

        let mut y_value = ((confirmed_values.len() - 1) as f64) + 0.25;
        let mut confirmed_bars = Vec::new();

        let mut names = Vec::new();
        for (count, name) in confirmed_values.iter() {
            confirmed_bars.push(Bar::new(y_value,
                                         *count as f64)
                .name(*name));
            names.push(String::from(*name));
            y_value = y_value - 1.;
        }
        names.reverse();

        let confirmed_chart = BarChart::new(confirmed_bars)
            .element_formatter(Box::new(recent_chart_label))
            .width(0.5)
            .name("现存确诊");

        let x_fmt = move |x: f64, _range: &RangeInclusive<f64>| {
            if let Some(name) = names.get(x.floor() as usize) {
                format!("{}", name.as_str())
            } else {
                String::new()
            }
        };

        Plot::new("plot_confirmed")
            .legend(Legend::default())
            .height(160.)
            .x_axis_formatter(x_fmt)
            .y_axis_formatter(|y, _range| {
                String::new()
            })
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(confirmed_chart);
            })
            .response
    }
}

fn recent_chart_label(bar: &Bar, chart: &BarChart) -> String {
    format!("{}: {} 例", bar.name, bar.value)
}