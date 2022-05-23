//! #   rCovid
//!                         rcdprovincedetailwindow.rs
//!                         -------------------------------------
//!     begin               2022/05/22
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use egui::{Context, Direction, Hyperlink, RichText, Ui, Window};
use egui_extras::{Size, TableBuilder};
use json::JsonValue;
use rcovid_core::CovidDataType;

use crate::ProvinceStat;

#[derive(Default)]
pub(crate) struct RcdProvinceDetailWindow {}

impl RcdProvinceDetailWindow {
    pub fn show(&mut self, ctx: &Context, open: &mut bool, province_data: Option<&ProvinceStat>) {
        if let Some(province_stat) = province_data {
            Window::new(format!("{} 疫情详情", province_stat.short_name).as_str()).open(open).show(ctx, |ui| {
                use super::View as _;

                ui.collapsing("各市/区疫情", |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                        .column(Size::initial(56.0).at_least(32.0))
                        .column(Size::initial(64.0).at_least(48.0))
                        .column(Size::initial(64.0).at_least(48.0))
                        .column(Size::initial(64.0).at_least(32.0))
                        .column(Size::initial(64.0).at_least(32.0))
                        .column(Size::initial(64.0).at_least(32.0))
                        .column(Size::initial(80.0).at_least(72.0))
                        .column(Size::initial(80.0).at_least(72.0))
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
                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                    ui.heading(RichText::new("高风险地区")); // .background_color(Color32::from_rgb(227, 231, 243)));
                                });
                            });
                            header.col(|ui| {
                                ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                    ui.heading(RichText::new("中风险地区")); // .background_color(Color32::from_rgb(227, 231, 243)));
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
                                        ui.strong(RichText::new(province_stat.dead_count.to_string()).size(20.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(province_stat.cured_count.to_string()).size(20.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(province_stat.suspected_count.to_string()).size(20.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(province_stat.high_danger_count.to_string()).size(20.));
                                    });
                                });
                                row.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.strong(RichText::new(province_stat.mid_danger_count.to_string()).size(20.));
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
                                            ui.label(city_stat.dead_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(city_stat.cured_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(city_stat.suspected_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(city_stat.high_danger_count.to_string());
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                            ui.label(city_stat.mid_danger_count.to_string());
                                        });
                                    });
                                });
                            }
                        });
                });

                ui.separator();

                ui.collapsing("风险地区详情", |ui| {
                    if province_stat.danger_areas.len() <= 0 {
                        ui.label("无");
                    } else {
                        TableBuilder::new(ui)
                            .striped(true)
                            .resizable(true)
                            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
                            .column(Size::initial(64.0).at_least(48.0))
                            .column(Size::initial(180.0).at_least(118.0))
                            .column(Size::initial(64.0).at_least(56.0))
                            .header(32., |mut header| {
                                header.col(|ui| {
                                    ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                        ui.heading(RichText::new("市/区名称")); // .background_color(Color32::from_rgb(227, 231, 243))
                                    });
                                });
                                header.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.heading(RichText::new("风险地区名称")); //.background_color(Color32::from_rgb(243, 186, 176)));
                                    });
                                });
                                header.col(|ui| {
                                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                        ui.heading(RichText::new("风险等级")); //.background_color(Color32::from_rgb(230, 154, 141)));
                                    });
                                });
                            })
                            .body(|mut body| {
                                for danger_area in &province_stat.danger_areas {
                                    body.row(30., |mut row| {
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                                                ui.label(danger_area.city_name.as_str());
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.label(danger_area.area_name.to_string());
                                            });
                                        });
                                        row.col(|ui| {
                                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                                ui.label(if danger_area.danger_level == 2 { "中" } else { "高" });
                                            });
                                        });
                                    });
                                }
                            });
                    }
                });

                ui.separator();

                ui.collapsing("其他信息", |ui| {
                    ui.label(format!("核酸检测点数量：{} 个", province_stat.detect_org_count));
                    ui.label(format!("疫苗接种机构数量：{} 个", province_stat.vaccination_org_count));
                    ui.add(Hyperlink::from_label_and_url("JSON统计数据", province_stat.statistic_data_uri.as_str()));
                });

                if !province_stat.comment.is_empty() {
                    ui.separator();
                    ui.label(format!("注：{}", province_stat.comment));
                }
            });
        }
    }
}