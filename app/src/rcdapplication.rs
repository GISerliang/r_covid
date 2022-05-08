//! #   rCovid
//!                         rcdappliacation
//!                         -------------------------------------
//!     begin               2022/05/08
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use eframe;
use eframe::glow;
use egui::epaint::ahash::AHashMap;
use egui::{vec2, widgets::Widget, Context, FontFamily, ImageButton, Rgba};
use egui_extras::RetainedImage;
use std::collections::BTreeMap;
use std::ops::Deref;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct RcdApplication {
    can_exit: bool,
    is_exiting: bool,
}

impl RcdApplication {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            return epi::get_value(storage, rgis_core::APP_KEY).unwrap_or_default();
        }
        tracing::debug!("Setup");

        let mut style: egui::Style = cc.egui_ctx.style().deref().clone();
        style.visuals.override_text_color = Some(egui::Color32::BLACK);
        style.override_text_style = Some(egui::TextStyle::Body);
        style
            .text_styles
            .get_mut(&egui::TextStyle::Body)
            .unwrap()
            .family = egui::FontFamily::Monospace;

        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(16.0, egui::FontFamily::Monospace),
        );

        let visuals = style.visuals.clone();
        cc.egui_ctx.set_style(style);
        cc.egui_ctx.set_visuals(visuals);
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        let mut fonts = egui::FontDefinitions::default();
        // Install my own font (maybe supporting non-latin characters):
        // 2. register the font content with a name.
        fonts.font_data.insert(
            "SimHei".to_owned(),
            egui::FontData::from_static(include_bytes!("../../fonts/simhei.ttf")), // 黑体
        );
        fonts.font_data.insert(
            "MSYH".to_owned(),
            egui::FontData::from_static(include_bytes!("../../fonts/msyh.ttc")), // 微软雅黑
        );
        fonts.font_data.insert(
            "maptool-iconfont".to_owned(),
            egui::FontData::from_static(include_bytes!("../../fonts/maptool-iconfont.ttf")),
        );

        // 3. Set two font families to use the font, font's name must have been
        // Put new font first (highest priority)registered in `font_data`.
        fonts.families.insert(
            FontFamily::Monospace,
            vec![
                "maptool-iconfont".to_owned(),
                "MSYH".to_owned(),
                "SimHei".to_owned(),
            ],
        );
        fonts.families.insert(
            FontFamily::Proportional,
            vec![
                "maptool-iconfont".to_owned(),
                "MSYH".to_owned(),
                "SimHei".to_owned(),
            ],
        );

        // 4. Configure context with modified `FontDefinitions`.
        cc.egui_ctx.set_fonts(fonts);

        Self {
            can_exit: false,
            is_exiting: false,
        }
    }
}

impl eframe::App for RcdApplication {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("m_app_menubar").show(ctx, |ui| {
            egui::trace!(ui, "m_app_menubar");
            // self.show_menu_bar(ui, frame);
        });

        let mut fill = ctx.style().visuals.extreme_bg_color;
        if !cfg!(target_arch = "wasm32") {
            // Native: WrapApp uses a transparent window, so let's show that off:
            // NOTE: the OS compositor assumes "normal" blending, so we need to hack it:
            let [r, g, b, _] = fill.to_array();
            fill = egui::Color32::from_rgba_premultiplied(r, g, b, 180);
        }
        let _frame = egui::Frame::none().fill(fill);

        egui::CentralPanel::default().frame(_frame).show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // self.m_map_canvas.ui(ui);
            });
        });

        if self.is_exiting {
            let res = rfd::MessageDialog::new()
                .set_title("退出rCovid")
                .set_description("真的要退出rCovid？")
                .set_buttons(rfd::MessageButtons::OkCancel)
                .show();
            if res {
                self.can_exit = true;
                frame.quit();
            } else {
                self.is_exiting = false;
            }
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        tracing::debug!("App saved");
        epi::set_value(storage, rgis_core::APP_KEY, self);
    }

    fn on_exit_event(&mut self) -> bool {
        self.is_exiting = true;
        self.can_exit
    }

    fn on_exit(&mut self, _gl: &glow::Context) {
        tracing::debug!("App exit");
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> Rgba {
        egui::Color32::WHITE.into()
    }

    fn warm_up_enabled(&self) -> bool {
        // The example windows use a lot of emojis. Pre-cache them by running one frame where everything is open
        cfg!(not(debug_assertions))
    }
}

impl RcdApplication {
    fn show_menu_bar(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        egui::menu::bar(ui, |ui| {
        });
    }
}