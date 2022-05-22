pub const APP_KEY: &str = "rCovid";

pub const COVID_URL: &str = "https://ncov.dxy.cn/ncovh5/view/pneumonia";

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CovidDataType {
    StatisticsService = 0,
    AreaStat,
    ListByCountryTypeService2true,
    TimelineService1,
    // 疫情热点
    TimelineService2,
    IndexRumorList,
    RecentStatV2,
}