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
use egui::{Color32, WidgetText, self, RichText, Hyperlink};
use egui_extras::{Size, TableBuilder};

use rcovid_core;
use crate::Window;

#[derive(Default)]
pub struct RcdTimelineService1Window {}

impl super::Window for RcdTimelineService1Window {
    fn name(&self) -> &'static str {
        "🔥 疫情热点"
    }

    fn window_type(&self) -> rcovid_core::CovidDataType {
        rcovid_core::CovidDataType::TimelineService1
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool, data: Option<&JsonValue>) {
        egui::Window::new(self.name()).open(open).show(ctx, |ui| {
            use super::View as _;
            self.ui(ui, data);
        });
    }
}

impl super::View for RcdTimelineService1Window {
    fn ui(&mut self, ui: &mut egui::Ui, data: Option<&JsonValue>) { // , data: Option<&JsonValue>
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right().with_cross_align(egui::Align::Center))
            .column(Size::initial(26.0).at_least(26.0))
            .column(Size::remainder().at_least(60.0))
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