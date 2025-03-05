use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct GameObject {
    #[serde(rename = "m_FileID")]
    pub file_id: i32,
    #[serde(rename = "m_PathID")]
    pub path_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct ScriptInfo {
    #[serde(rename = "m_FileID")]
    pub file_id: i32,
    #[serde(rename = "m_PathID")]
    pub path_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct RecipeItem {
    pub Name: String,
    pub ID: i32,
    pub SID: String,
    pub Type: i32,
    pub Handcraft: bool,
    pub Explicit: bool,
    pub TimeSpend: i32,
    pub Items: Vec<i32>,
    pub ItemCounts: Vec<i32>,
    pub Results: Vec<i32>,
    pub ResultCounts: Vec<i32>,
    pub GridIndex: i32,
    pub IconPath: String,
    pub Description: String,
    pub NonProductive: bool,
}

#[derive(Debug, Deserialize)]
pub struct RecipeProtoSet {
    #[serde(rename = "m_GameObject")]
    pub game_object: GameObject,
    #[serde(rename = "m_Enabled")]
    pub enabled: i32,
    #[serde(rename = "m_Script")]
    pub script: ScriptInfo,
    #[serde(rename = "m_Name")]
    pub name: String,
    pub TableName: String,
    pub Signature: String,
    pub dataArray: Vec<RecipeItem>,
}

pub fn recipe() -> RecipeProtoSet {
    let json_str = include_str!("../data/RecipeProtoSet.json");
    serde_json::from_str(json_str).unwrap()
}
