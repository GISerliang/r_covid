use json::JsonValue;
use egui;

use rcovid_core;

/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, data: Option<&JsonValue>);
}

/// Something to view
pub trait Window {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    fn window_type(&self) -> rcovid_core::CovidDataType;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool, data: Option<&JsonValue>);
}

pub mod rcdtimelineservice1window;
