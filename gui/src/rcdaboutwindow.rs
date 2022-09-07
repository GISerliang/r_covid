//! #   rCovid
//!                         rcdaboutwindow.rs
//!                         -------------------------------------
//!     begin               2022/05/27
//!     copyright           (C) 2022 by GISerliang
//!     email               hml8431386@163.com
//!                         -------------------------------------
//!
////////////////////////////////////////////////////////////////////////////////

use egui::{Align2, Color32, RichText, Vec2, Window};
use egui_extras::RetainedImage;

pub struct RcdAboutWindow {
    logo: RetainedImage,
}

impl Default for RcdAboutWindow {
    fn default() -> Self {
        RcdAboutWindow {
            logo: RetainedImage::from_image_bytes("logo.png", include_bytes!("../../assets/logo.png")).unwrap()
        }
    }
}

impl RcdAboutWindow {
    fn name(&self) -> &'static str {
        "关于rCovid"
    }

    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        Window::new(self.name())
            .open(open)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::new(0., -80.))
            .show(ctx, |ui| {
                ui.image(self.logo.texture_id(ctx), Vec2::new(64., 64.));

                ui.label("由Rust和egui构建的展示2019新型冠状病毒（COVID-19/2019-nCoV）实时疫情的应用");

                ui.separator();
                ui.vertical(|ui| {
                    ui.heading("更新日志");
                    ui.collapsing("V0.1.0", |ui| {
                        ui.label("获取丁香园实时数据");
                        ui.label("增加疫情热点");
                        ui.label("增加近期疫情（各省详细信息）");
                        ui.label("增加国内疫情（各省详细信息）");
                        ui.label("增加全球疫情（各国详细信息）");
                    });
                    ui.collapsing("V0.2.0", |ui| {
                        ui.label("增加wasm编译");
                    });
                });

                ui.separator();
                ui.label(RichText::new("注：项目为个人爱好，如有谬误请指正；如侵权，请及时告知").size(14.).color(Color32::from_rgb(204, 204, 0)));
            });
    }
}