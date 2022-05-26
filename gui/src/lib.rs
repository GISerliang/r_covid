use json::JsonValue;
use egui;

use rcovid_core;

#[derive(Debug, Clone)]
pub(crate) struct DangerArea {
    // 市/区名称
    pub city_name: String,
    // 区域名称
    pub area_name: String,
    // 风险等级
    pub danger_level: u8,
}

pub(crate) struct CityStat {
    // 名称
    pub name: String,
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
    // 地区代码
    pub location_id: i32,
    // 高风险数量
    pub high_danger_count: u32,
    // 中风险数量
    pub mid_danger_count: u32,
}

pub(crate) struct ProvinceStat {
    // 名称
    pub name: String,
    // 简称
    pub short_name: String,
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
    // 地区代码
    pub location_id: i32,
    // 备注
    pub comment: String,
    // JSON统计数据
    pub statistic_data_uri: String,
    // 高风险数量
    pub high_danger_count: u32,
    // 中风险数量
    pub mid_danger_count: u32,
    // 检测机构数量
    pub detect_org_count: u32,
    // 疫苗接种机构数量
    pub vaccination_org_count: u32,
    // 风险区域
    pub danger_areas: Vec<DangerArea>,
    // 市/区情况
    pub cities: Vec<CityStat>,
}

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
pub mod rcdareastatwindow;
pub(crate) mod rcdprovincedetailwindow;
pub mod rcdrecentstatv2window;
pub mod rcdlistbycountrytypewindow;
