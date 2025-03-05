use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
struct GameObject {
    #[serde(rename = "m_FileID")]
    file_id: i32,
    #[serde(rename = "m_PathID")]
    path_id: i32,
}

#[derive(Debug, Deserialize)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Debug, Deserialize)]
struct ItemData {
    Name: String,
    ID: i32,
    SID: String,
    Type: i32,
    SubID: i32,
    MiningFrom: String,
    ProduceFrom: String,
    StackSize: i32,
    Grade: i32,
    Upgrades: Vec<i32>,
    IsFluid: bool,
    IsEntity: bool,
    CanBuild: bool,
    BuildInGas: bool,
    IconPath: String,
    ModelIndex: i32,
    ModelCount: i32,
    HpMax: i32,
    Ability: i32,
    HeatValue: i32,
    Potential: i32,
    ReactorInc: f32,
    FuelType: i32,
    AmmoType: i32,
    BombType: i32,
    CraftType: i32,
    BuildIndex: i32,
    BuildMode: i32,
    GridIndex: i32,
    UnlockKey: i32,
    PreTechOverride: i32,
    Productive: bool,
    MechaMaterialID: i32,
    DropRate: f32,
    EnemyDropLevel: i32,
    #[serde(rename = "EnemyDropRange")]
    EnemyDropRange: Vector2,
    EnemyDropCount: f32,
    EnemyDropMask: i32,
    EnemyDropMaskRatio: f32,
    DescFields: Vec<i32>,
    Description: String,
}

#[derive(Debug, Deserialize)]
struct ItemProtoSet {
    #[serde(rename = "m_GameObject")]
    game_object: GameObject,
    #[serde(rename = "m_Enabled")]
    enabled: i32,
    #[serde(rename = "m_Script")]
    script: GameObject,
    #[serde(rename = "m_Name")]
    name: String,
    TableName: String,
    Signature: String,
    dataArray: Vec<ItemData>,
}

pub fn item() -> ItemProtoSet {
    let json_str = include_str!("../data/ItemProtoSet.json");
    serde_json::from_str(json_str).unwrap()
}
