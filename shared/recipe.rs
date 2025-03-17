#[derive(serde::Deserialize)]
pub struct RecipeItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "SID")]
    pub sid: String,
    #[serde(rename = "Type")]
    pub type_: i64, // 避讳关键字 "type"
    #[serde(rename = "Handcraft")]
    pub handcraft: bool,
    #[serde(rename = "Explicit")]
    pub explicit: bool,
    #[serde(rename = "TimeSpend")]
    pub time_spend: i64,
    #[serde(rename = "Items")]
    pub items: Vec<i16>,
    #[serde(rename = "ItemCounts")]
    pub item_counts: Vec<i64>,
    #[serde(rename = "Results")]
    pub results: Vec<i16>,
    #[serde(rename = "ResultCounts")]
    pub result_counts: Vec<i64>,
    #[serde(rename = "GridIndex")]
    pub grid_index: i64,
    #[serde(rename = "IconPath")]
    pub icon_path: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "NonProductive")]
    pub non_productive: bool,
}
