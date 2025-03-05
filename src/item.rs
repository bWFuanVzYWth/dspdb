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
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Deserialize)]
pub struct ItemData {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "SID")]
    pub sid: String,
    #[serde(rename = "Type")]
    pub type_: i32, // 避讳关键字 "type"
    #[serde(rename = "SubID")]
    pub sub_id: i32,
    #[serde(rename = "MiningFrom")]
    pub mining_from: String,
    #[serde(rename = "ProduceFrom")]
    pub produce_from: String,
    #[serde(rename = "StackSize")]
    pub stack_size: i32,
    #[serde(rename = "Grade")]
    pub grade: i32,
    #[serde(rename = "Upgrades")]
    pub upgrades: Vec<i32>,
    #[serde(rename = "IsFluid")]
    pub is_fluid: bool,
    #[serde(rename = "IsEntity")]
    pub is_entity: bool,
    #[serde(rename = "CanBuild")]
    pub can_build: bool,
    #[serde(rename = "BuildInGas")]
    pub build_in_gas: bool,
    #[serde(rename = "IconPath")]
    pub icon_path: String,
    #[serde(rename = "ModelIndex")]
    pub model_index: i32,
    #[serde(rename = "ModelCount")]
    pub model_count: i32,
    #[serde(rename = "HpMax")]
    pub hp_max: i32,
    #[serde(rename = "Ability")]
    pub ability: i32,
    #[serde(rename = "HeatValue")]
    pub heat_value: i32,
    #[serde(rename = "Potential")]
    pub potential: i32,
    #[serde(rename = "ReactorInc")]
    pub reactor_inc: f32,
    #[serde(rename = "FuelType")]
    pub fuel_type: i32,
    #[serde(rename = "AmmoType")]
    pub ammo_type: i32,
    #[serde(rename = "BombType")]
    pub bomb_type: i32,
    #[serde(rename = "CraftType")]
    pub craft_type: i32,
    #[serde(rename = "BuildIndex")]
    pub build_index: i32,
    #[serde(rename = "BuildMode")]
    pub build_mode: i32,
    #[serde(rename = "GridIndex")]
    pub grid_index: i32,
    #[serde(rename = "UnlockKey")]
    pub unlock_key: i32,
    #[serde(rename = "PreTechOverride")]
    pub pre_tech_override: i32,
    #[serde(rename = "Productive")]
    pub productive: bool,
    #[serde(rename = "MechaMaterialID")]
    pub mecha_material_id: i32,
    #[serde(rename = "DropRate")]
    pub drop_rate: f32,
    #[serde(rename = "EnemyDropLevel")]
    pub enemy_drop_level: i32,
    #[serde(rename = "EnemyDropRange")]
    pub enemy_drop_range: Vector2,
    #[serde(rename = "EnemyDropCount")]
    pub enemy_drop_count: f32,
    #[serde(rename = "EnemyDropMask")]
    pub enemy_drop_mask: i32,
    #[serde(rename = "EnemyDropMaskRatio")]
    pub enemy_drop_mask_ratio: f32,
    #[serde(rename = "DescFields")]
    pub desc_fields: Vec<i32>,
    #[serde(rename = "Description")]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemProtoSet {
    #[serde(rename = "m_GameObject")]
    pub game_object: GameObject,
    #[serde(rename = "m_Enabled")]
    pub enabled: i32,
    #[serde(rename = "m_Script")]
    pub script: GameObject,
    #[serde(rename = "m_Name")]
    pub name: String,
    #[serde(rename = "TableName")]
    pub table_name: String,
    #[serde(rename = "Signature")]
    pub signature: String,
    #[serde(rename = "dataArray")]
    pub data_array: Vec<ItemData>,
}

pub fn item() -> ItemProtoSet {
    let json_str = include_str!("../data/ItemProtoSet.json");
    serde_json::from_str(json_str).unwrap()
}
