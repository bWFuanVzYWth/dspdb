#[derive(Debug, serde::Deserialize)]
struct RecipeItem {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ID")]
    id: i64,
    #[serde(rename = "SID")]
    sid: String,
    #[serde(rename = "Type")]
    type_: i64, // 避讳关键字 "type"
    #[serde(rename = "Handcraft")]
    handcraft: bool,
    #[serde(rename = "Explicit")]
    explicit: bool,
    #[serde(rename = "TimeSpend")]
    time_spend: i64,
    #[serde(rename = "Items")]
    items: Vec<i16>,
    #[serde(rename = "ItemCounts")]
    item_counts: Vec<i64>,
    #[serde(rename = "Results")]
    results: Vec<i16>,
    #[serde(rename = "ResultCounts")]
    result_counts: Vec<i64>,
    #[serde(rename = "GridIndex")]
    grid_index: i64,
    #[serde(rename = "IconPath")]
    icon_path: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "NonProductive")]
    non_productive: bool,
}
