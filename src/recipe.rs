use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct GameObject {
    #[serde(rename = "m_FileID")]
    pub file_id: i64,
    #[serde(rename = "m_PathID")]
    pub path_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct ScriptInfo {
    #[serde(rename = "m_FileID")]
    pub file_id: i64,
    #[serde(rename = "m_PathID")]
    pub path_id: i64,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct RecipeProtoSet {
    #[serde(rename = "m_GameObject")]
    pub game_object: GameObject,
    #[serde(rename = "m_Enabled")]
    pub enabled: i64,
    #[serde(rename = "m_Script")]
    pub script: ScriptInfo,
    #[serde(rename = "m_Name")]
    pub name: String,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Signature")]
    pub signature: String,
    #[serde(rename = "dataArray")]
    pub data_array: Vec<RecipeItem>,
}

pub fn recipes() -> RecipeProtoSet {
    let json_str = include_str!("../data/RecipeProtoSet.json");
    serde_json::from_str(json_str).unwrap()
}
