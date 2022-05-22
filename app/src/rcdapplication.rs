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
use egui::{vec2, widgets::Widget, Context, FontFamily, ImageButton, Rgba, Window, Align2, Vec2, TextBuffer, ScrollArea};
use egui_extras::RetainedImage;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ops::Deref;
use ehttp::{self};
use json::JsonValue;
use poll_promise::Promise;
use scraper::Html;

use rcovid_core::CovidDataType;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct RcdApplication {
    can_exit: bool,
    is_exiting: bool,
    trigger_fetch: bool,
    first_loaded: bool,
    promise: Option<Promise<ehttp::Result<(String, String)>>>,
    error_msg: String,
    script_id_map: HashMap<String, CovidDataType>,
    covid_json_map: HashMap<CovidDataType, JsonValue>,
    windows: Vec<Box<dyn rcovid_gui::Window>>,
    open_windows: BTreeSet<CovidDataType>,
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
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "SimHei".to_owned());
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "maptool-iconfont".to_owned());
        fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "MSYH".to_owned());

        fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "SimHei".to_owned());
        fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "maptool-iconfont".to_owned());
        fonts.families.get_mut(&FontFamily::Monospace).unwrap().insert(0, "MSYH".to_owned());

        for family in &fonts.families {
            println!("{:?}", family);
        }

        // 4. Configure context with modified `FontDefinitions`.
        cc.egui_ctx.set_fonts(fonts);

        let mut script_id_map = HashMap::new();
        script_id_map.insert(String::from("getAreaStat"), CovidDataType::AreaStat);
        script_id_map.insert(String::from("getStatisticsService"), CovidDataType::StatisticsService);
        script_id_map.insert(String::from("getListByCountryTypeService2true"), CovidDataType::ListByCountryTypeService2true);
        script_id_map.insert(String::from("getTimelineService1"), CovidDataType::TimelineService1);
        script_id_map.insert(String::from("getTimelineService2"), CovidDataType::TimelineService2);
        script_id_map.insert(String::from("getIndexRumorList"), CovidDataType::IndexRumorList);
        script_id_map.insert(String::from("fetchRecentStatV2"), CovidDataType::RecentStatV2);

        let windows: Vec<Box<dyn rcovid_gui::Window>> = vec![
            Box::new(rcovid_gui::rcdtimelineservice1window::RcdTimelineService1Window::default()),
        ];

        let mut open_windows = BTreeSet::new();
        open_windows.insert(CovidDataType::TimelineService1);

        Self {
            can_exit: false,
            is_exiting: false,
            trigger_fetch: false,
            first_loaded: true,
            promise: None,
            error_msg: String::new(),
            script_id_map,
            covid_json_map: HashMap::new(),
            windows,
            open_windows,
        }
    }
}

impl eframe::App for RcdApplication {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("m_app_menubar").show(ctx, |ui| {
            egui::trace!(ui, "m_app_menubar");
            // self.show_menu_bar(ui, frame);
        });

        egui::SidePanel::right("rcovid_right_panel").min_width(150.).default_width(180.)
            .show(ctx, |ui| {
                egui::trace!(ui);
                ui.vertical_centered(|ui| {
                    ui.heading("窗口选项");
                });

                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    for window in &self.windows {
                        let data = self.covid_json_map.get(&window.window_type());
                        let mut is_open = self.open_windows.contains(&window.window_type());
                        ui.checkbox(&mut is_open, window.name());
                        set_open(&mut self.open_windows, &window.window_type(), is_open);
                    }
                });
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
                if !self.trigger_fetch {
                    self.load_covid();
                }

                if let Some(promise) = &self.promise {
                    if let Some(result) = promise.ready() {
                        if self.first_loaded {
                            match result {
                                Ok((content, error)) => {
                                    let document = Html::parse_document(content);
                                    let tree_node = document.tree;
                                    tree_node.nodes().for_each(|node_ref| {
                                        let node = node_ref.value();
                                        if node.is_element() {
                                            let element = node.as_element().unwrap();

                                            if let Some(element_id) = element.id() {
                                                if self.script_id_map.contains_key(element_id) {
                                                    let covid_data_type = *self.script_id_map.get(element_id).unwrap();
                                                    if node_ref.has_children() {
                                                        let child_node_ref = node_ref.first_child().unwrap();
                                                        let child_node = child_node_ref.value();
                                                        if child_node.is_text() {
                                                            let text = child_node.as_text().unwrap();

                                                            let try_str = format!("try {} window.{} = ", "{", element_id);
                                                            let html_content = text.trim()
                                                                .replace(try_str.as_str(), "").as_str()
                                                                .replace("}catch(e){}", "").as_str()
                                                                .replace("}catch(e) {}", "").as_str()
                                                                .replace("} catch(e){}", "").as_str()
                                                                .replace("} catch(e) {}", "");

                                                            let json_res = json::parse(html_content.as_str());
                                                            if json_res.is_ok() {
                                                                self.covid_json_map.insert(covid_data_type, json_res.unwrap());
                                                            } else {
                                                                tracing::error!("{} error, error info: {}", element_id, json_res.unwrap_err().to_string());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    });
                                }
                                Err(err) => {
                                    self.error_msg = err.to_string();
                                }
                            }
                            self.first_loaded = false;
                        }
                    }
                }
            });
        });

        if self.is_exiting {
            Window::new("退出rCovid")
                .collapsible(false)
                .resizable(false)
                .default_width(320.)
                .default_height(240.)
                .anchor(Align2::CENTER_CENTER, Vec2::new(0., -80.))
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("真的要退出rCovid？");

                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                                if ui.button("取消").clicked() {
                                    self.is_exiting = false;
                                }

                                if ui.button("确定").clicked() {
                                    self.can_exit = true;
                                    frame.quit();
                                }
                            });
                        });
                    });
                });
        }

        self.windows(ctx);
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
        egui::menu::bar(ui, |ui| {});
    }

    fn load_covid(&mut self) {
        let (sender, promise) = Promise::new();
        let request = ehttp::Request::get(rcovid_core::COVID_URL);
        ehttp::fetch(request, move |response| {
            let result = response.map(|response| {
                if response.ok {
                    if let Some(text) = response.text() {
                        (String::from(text), String::new())
                    } else {
                        (String::new(), format!("Load covid data error, error info: {:?}", response.status_text))
                    }
                } else {
                    panic!("Load covid data error, error info: {:?}", response.status_text);
                }
            });
            sender.send(result);
        });
        self.trigger_fetch = true;
        self.promise = Some(promise);
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { windows, open_windows, covid_json_map, .. } = self;
        for window in windows {
            let mut is_open = open_windows.contains(&window.window_type());
            window.show(ctx, &mut is_open, covid_json_map.get(&window.window_type()));
            set_open(open_windows, &window.window_type(), is_open);
        }
    }
}

fn set_open(open_windows: &mut BTreeSet<CovidDataType>, key: &CovidDataType, is_open: bool) {
    if is_open {
        if !open_windows.contains(key) {
            open_windows.insert(key.to_owned());
        }
    } else {
        open_windows.remove(key);
    }
}