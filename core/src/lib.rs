pub const APP_KEY: &str = "rCovid";

pub const COVID_URL: &str = "https://ncov.dxy.cn/ncovh5/view/pneumonia";

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CovidDataType {
    StatisticsService = 0,
    // 国内疫情
    AreaStat,
    ListByCountryTypeService2true,
    // 疫情热点
    TimelineService1,
    TimelineService2,
    IndexRumorList,
    // 近期疫情
    RecentStatV2,
}