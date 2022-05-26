//! #   rCovid
//!                         rcdtimelineservice1window
//!                         -------------------------------------
//!     begin               2022/05/17
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use json::JsonValue;
use egui::{Color32, WidgetText, self, RichText};
use egui_extras::{Size, TableBuilder};

use rcovid_core;

#[derive(Default)]
pub struct RcdTimelineService1Window {}

impl super::Window for RcdTimelineService1Window {
    fn name(&self) -> &'static str {
        "🔥 疫情热点"
    }

    fn window_type(&self) -> rcovid_core::CovidDataType {
        rcovid_core::CovidDataType::TimelineService1
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View as _;
            self.ui(ui, data, statistics_data);
        });
    }
}

impl super::View for RcdTimelineService1Window {
    fn ui(&mut self, ui: &mut egui::Ui, data: Option<&JsonValue>, statistics_data: Option<&JsonValue>) { // , data: Option<&JsonValue>
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(26.0).at_least(26.0))
            .column(Size::initial(120.0).at_least(120.0))
            .column(Size::remainder().at_least(64.0))
            .body(|mut body| {
                if let Some(json_value) = data {
                    if json_value.is_array() {
                        let members = json_value.members();
                        for member in members {
                            body.row(30., |mut row| {
                                row.col(|ui| {
                                    ui.label(RichText::new("最新").background_color(Color32::from_rgb(247, 76, 49)).color(Color32::WHITE));
                                });
                                row.col(|ui| {
                                    let timestamp = member["pubDate"].as_i64().unwrap_or(0);
                                    let china_timezone = chrono::FixedOffset::east(8 * 3600);
                                    let datetime = format!("{}",
                                                           chrono::NaiveDateTime::from_timestamp(timestamp / 1000,
                                                                                                 (timestamp % 1000) as u32).
                                                               format("%Y-%m-%d %H:%M:%SZ")).as_str().parse::<chrono::DateTime<chrono::Utc>>().unwrap();
                                    ui.code(format!("{}", datetime.with_timezone(&china_timezone).format("%Y-%m-%d %H:%M")).as_str());
                                });
                                row.col(|ui| {
                                    ui.hyperlink_to(WidgetText::from(member["title"].as_str().unwrap_or_default()),
                                                    member["sourceUrl"].as_str().unwrap_or_default())
                                        .on_hover_text(member["summary"].as_str().unwrap_or_default());
                                });
                            });
                        }
                    }
                }
            });
    }
}