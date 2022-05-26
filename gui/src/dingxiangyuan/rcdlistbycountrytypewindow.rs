//! #   rCovid
//!                         rcdlistbycountrytypewindow.rs
//!                         -------------------------------------
//!     begin               2022/05/25
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use std::collections::BTreeMap;
use egui::{Align, Color32, Context, Direction, FontFamily, FontId, Layout, RichText, TextFormat, Ui, Window};
use egui::text::LayoutJob;
use json::JsonValue;

use rcovid_core::CovidDataType;

#[derive(Debug)]
struct IncrVo {
    pub current_confirmed_incr: i64,
    pub confirmed_incr: i64,
    pub cured_incr: i64,
    pub dead_incr: i64,
}

#[derive(Debug)]
struct ProvinceStat {
    pub province_id: String,
    pub province_name: String,
    pub province_short_name: String,
    pub city_name: String,
    // 现存确诊
    pub current_confirmed_count: i64,
    // 累计确诊
    pub confirmed_count: i64,
    // 确诊排序
    pub confirmed_count_rank: u32,
    // 死亡
    pub dead_count: i64,
    // 治愈
    pub cured_count: i64,
    // 疑似
    pub suspected_count: i64,
    // 死亡排序
    pub dead_count_rank: u32,
    // 死亡率
    pub dead_rate: f32,
    // 死亡率排名
    pub dead_rate_rank: u32,
    // 备注
    pub comment: String,
    // 地区代码
    pub location_id: i32,
    // 国家简称
    pub country_short_code: String,
    // 国家全称
    pub country_full_name: String,
    // JSON统计数据
    pub statistic_data_uri: String,
    pub update_time: chrono::DateTime<chrono::Utc>,
    pub country_type: u8,
    // TODO 未知数据，暂不处理
    pub incr_vo: Option<IncrVo>,
    // TODO 各国家以下数据都一致，暂不处理
    // 新增
    pub yesterday_local_confirmed_count: i64,
    // 无症状
    pub yesterday_asymptomatic_count: i64,
    // 新增确诊
    pub yesterday_confirmed_count: i64,
    // 其他确诊
    pub yesterday_other_confirmed_count: i64,
    pub high_danger: String,
    pub mid_danger: String,
    pub high_in_desc: String,
    pub low_in_desc: String,
    pub out_desc: String,
}

#[derive(Debug)]
struct ContinentStat {
    pub continent: String,
    // 现存确诊
    pub current_confirmed_count: i64,
    // 累计确诊
    pub confirmed_count: i64,
    // 死亡
    pub dead_count: i64,
    // 治愈
    pub cured_count: i64,
    // 疑似
    pub suspected_count: i64,
    pub provinces: Vec<ProvinceStat>,
}

#[derive(Debug)]
struct GlobalStatistics {
    // 现存确诊
    pub current_confirmed_count: i64,
    // 累计确诊
    pub confirmed_count: i64,
    // 死亡
    pub dead_count: i64,
    // 治愈
    pub cured_count: i64,
    pub current_confirmed_incr: i64,
    pub confirmed_incr: i64,
    pub cured_incr: i64,
    pub dead_incr: i64,
    pub statistic_datetime: chrono::DateTime<chrono::Utc>,
    pub yesterday_confirmed_count_incr: i64,
}

#[derive(Default)]
pub struct RcdListByCountryTypeWindow {
    continents_stat: BTreeMap<String, ContinentStat>,
    global_statistics: Option<GlobalStatistics>,
}

impl super::Window for RcdListByCountryTypeWindow {
    fn name(&self) -> &'static str {
        "🌐 全球疫情"
    }

    fn window_type(&self) -> CovidDataType {
        CovidDataType::ListByCountryTypeService2true
    }

    fn show(&mut self, ctx: &Context, open: &mut bool, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) {
        Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View as _;
            self.ui(ui, data, statistics_data);
        });
    }
}

impl super::View for RcdListByCountryTypeWindow {
    fn ui(&mut self, ui: &mut Ui, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) {
        use egui_extras::{Size, TableBuilder};

        if self.continents_stat.len() <= 0 {
            if let Some(json_value) = data {
                if json_value.is_array() {
                    let members = json_value.members();
                    for member in members {
                        let timestamp = member["modifyTime"].as_i64().unwrap_or(0);
                        let datetime = format!("{}",
                                               chrono::NaiveDateTime::from_timestamp(timestamp / 1000,
                                                                                     (timestamp % 1000) as u32).
                                                   format("%Y-%m-%d %H:%M:%SZ")).as_str().parse::<chrono::DateTime<chrono::Utc>>().unwrap();

                        let incr_vo = if member.has_key("incrVo") {
                            let incr_vo_value = &member["incrVo"];
                            Some(IncrVo {
                                current_confirmed_incr: incr_vo_value["currentConfirmedIncr"].as_i64().unwrap_or(0),
                                confirmed_incr: incr_vo_value["confirmedIncr"].as_i64().unwrap_or(0),
                                cured_incr: incr_vo_value["curedIncr"].as_i64().unwrap_or(0),
                                dead_incr: incr_vo_value["deadIncr"].as_i64().unwrap_or(0),
                            })
                        } else {
                            None
                        };

                        let province = ProvinceStat {
                            province_id: member["provinceId"].as_str().unwrap_or("").to_string(),
                            province_name: member["provinceName"].as_str().unwrap_or("").to_string(),
                            province_short_name: member["provinceShortName"].as_str().unwrap_or("").to_string(),
                            city_name: member["cityName"].as_str().unwrap_or("").to_string(),
                            current_confirmed_count: member["currentConfirmedCount"].as_i64().unwrap_or(0),
                            confirmed_count: member["confirmedCount"].as_i64().unwrap_or(0),
                            confirmed_count_rank: member["confirmedCountRank"].as_u32().unwrap_or(0),
                            dead_count: member["deadCount"].as_i64().unwrap_or(0),
                            cured_count: member["curedCount"].as_i64().unwrap_or(0),
                            suspected_count: member["suspectedCount"].as_i64().unwrap_or(0),
                            dead_count_rank: member["deadCountRank"].as_u32().unwrap_or(0),
                            dead_rate: member["deadRate"].as_f32().unwrap_or(0.),
                            dead_rate_rank: member["deadRateRank"].as_u32().unwrap_or(0),
                            comment: member["comment"].as_str().unwrap_or("").to_string(),
                            location_id: member["locationId"].as_i32().unwrap_or(0),
                            country_short_code: member["countryShortCode"].as_str().unwrap_or("").to_string(),
                            country_full_name: member["countryFullName"].as_str().unwrap_or("").to_string(),
                            statistic_data_uri: member["statisticsData"].as_str().unwrap_or("").to_string(),
                            update_time: datetime,
                            country_type: member["countryType"].as_u8().unwrap_or(0),
                            incr_vo,
                            yesterday_local_confirmed_count: member["yesterdayLocalConfirmedCount"].as_i64().unwrap_or(0),
                            yesterday_asymptomatic_count: member["yesterdayAsymptomaticCount"].as_i64().unwrap_or(0),
                            yesterday_confirmed_count: member["yesterdayConfirmedCount"].as_i64().unwrap_or(0),
                            yesterday_other_confirmed_count: member["yesterdayOtherConfirmedCount"].as_i64().unwrap_or(0),
                            high_danger: member["highDanger"].as_str().unwrap_or("").to_string(),
                            mid_danger: member["midDanger"].as_str().unwrap_or("").to_string(),
                            high_in_desc: member["highInDesc"].as_str().unwrap_or("").to_string(),
                            low_in_desc: member["lowInDesc"].as_str().unwrap_or("").to_string(),
                            out_desc: member["outDesc"].as_str().unwrap_or("").to_string(),
                        };

                        let continent_name = member["continents"].as_str().unwrap_or("").to_string();
                        if let Some(continent_stat) = self.continents_stat.get_mut(&continent_name) {
                            continent_stat.suspected_count += province.suspected_count;
                            continent_stat.cured_count += province.cured_count;
                            continent_stat.dead_count += province.dead_count;
                            continent_stat.confirmed_count += province.confirmed_count;
                            continent_stat.current_confirmed_count += province.current_confirmed_count;
                            continent_stat.provinces.push(province);
                        } else {
                            let continent_stat = ContinentStat {
                                continent: member["continents"].as_str().unwrap_or("").to_string(),
                                current_confirmed_count: province.current_confirmed_count,
                                confirmed_count: province.confirmed_count,
                                dead_count: province.dead_count,
                                cured_count: province.cured_count,
                                suspected_count: province.suspected_count,
                                provinces: vec![province],
                            };
                            self.continents_stat.insert(continent_name, continent_stat);
                        }
                    }
                }

                for (_, continent_stat) in &mut self.continents_stat {
                    continent_stat.provinces.sort_by(|a, b| {
                        b.current_confirmed_count.cmp(&a.current_confirmed_count)
                    });
                }
            }
        }

        if self.global_statistics.is_none() {
            if let Some(json_value) = statistics_data {
                if json_value.is_object() && json_value.has_key("globalStatistics") {
                    let json = &json_value["globalStatistics"];

                    let timestamp = json_value["modifyTime"].as_i64().unwrap_or(0);
                    let datetime = format!("{}",
                                           chrono::NaiveDateTime::from_timestamp(timestamp / 1000,
                                                                                 (timestamp % 1000) as u32).
                                               format("%Y-%m-%d %H:%M:%SZ")).as_str().parse::<chrono::DateTime<chrono::Utc>>().unwrap();

                    self.global_statistics = Some(GlobalStatistics {
                        current_confirmed_count: json["currentConfirmedCount"].as_i64().unwrap_or(0),
                        confirmed_count: json["confirmedCount"].as_i64().unwrap_or(0),
                        dead_count: json["deadCount"].as_i64().unwrap_or(0),
                        cured_count: json["curedCount"].as_i64().unwrap_or(0),
                        current_confirmed_incr: json["currentConfirmedIncr"].as_i64().unwrap_or(0),
                        confirmed_incr: json["confirmedIncr"].as_i64().unwrap_or(0),
                        cured_incr: json["curedIncr"].as_i64().unwrap_or(0),
                        dead_incr: json["deadIncr"].as_i64().unwrap_or(0),
                        statistic_datetime: datetime,
                        yesterday_confirmed_count_incr: json["yesterdayConfirmedCountIncr"].as_i64().unwrap_or(0),
                    });
                }
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(global_stat) = &self.global_statistics {
                let china_timezone = chrono::FixedOffset::east(8 * 3600);
                ui.code(format!("截止北京时间 {}", global_stat.statistic_datetime.with_timezone(&china_timezone).format("%Y-%m-%d %H:%M")));

                ui.separator();

                TableBuilder::new(ui).striped(true)
                    .column(Size::initial(80.).at_least(80.))
                    .column(Size::initial(80.).at_least(80.))
                    .column(Size::initial(80.).at_least(80.))
                    .column(Size::initial(80.).at_least(80.))
                    .body(|mut body| {
                        body.row(12., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("昨日{}{}", if global_stat.current_confirmed_incr >= 0 { "+" } else { "-" },
                                                                   global_stat.current_confirmed_incr.abs()).as_str()).size(12.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("昨日{}{}", if global_stat.confirmed_incr >= 0 { "+" } else { "-" },
                                                                   global_stat.confirmed_incr.abs()).as_str()).size(12.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("昨日{}{}", if global_stat.dead_incr >= 0 { "+" } else { "-" },
                                                                   global_stat.dead_incr.abs()).as_str()).size(12.));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("昨日{}{}", if global_stat.cured_incr >= 0 { "+" } else { "-" },
                                                                   global_stat.cured_incr.abs()).as_str()).size(12.));
                                });
                            });
                        });
                        body.row(20., |mut row| {
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", global_stat.current_confirmed_count).as_str()));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", global_stat.confirmed_count).as_str()));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", global_stat.dead_count).as_str()));
                                });
                            });
                            row.col(|ui| {
                                ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.label(RichText::new(format!("{}", global_stat.cured_count).as_str()));
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

            let mut iter = Vec::from_iter(&self.continents_stat);
            iter.sort_by(|&(_, a), &(_, b)| {
                b.current_confirmed_count.cmp(&a.current_confirmed_count)
            });
            for (continent_name, continent_stat) in iter {
                egui::CollapsingHeader::new(continent_name.as_str()).show(ui, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                        .column(Size::initial(64.0).at_least(56.0))
                        .column(Size::initial(80.0).at_least(72.0))
                        .column(Size::initial(80.0).at_least(72.0))
                        .column(Size::initial(80.0).at_least(72.0))
                        .column(Size::initial(80.0).at_least(72.0))
                        .column(Size::initial(80.0).at_least(72.0))
                        .header(32., |mut header| {
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.heading(RichText::new("地区"));
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
                                    ui.heading(RichText::new("死亡"));
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                    ui.heading(RichText::new("治愈"));
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                    ui.heading(RichText::new("疑似"));
                                });
                            });
                        })
                        .body(|mut body| {
                            body.row(30., |mut row| {
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                        ui.strong(RichText::new(continent_stat.continent.as_str()).size(18.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(continent_stat.current_confirmed_count.to_string()).size(18.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(continent_stat.confirmed_count.to_string()).size(18.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(continent_stat.dead_count.to_string()).size(18.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(continent_stat.cured_count.to_string()).size(18.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(continent_stat.suspected_count.to_string()).size(18.));
                                    });
                                });
                            });

                            for province_stat in &continent_stat.provinces {
                                body.row(30., |mut row| {
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                            ui.label(province_stat.province_name.as_str());
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
                                });
                            }
                        });
                });
            }
        });
    }
}