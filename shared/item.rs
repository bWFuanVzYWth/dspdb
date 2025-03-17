#[derive(Debug, serde::Deserialize)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

#[derive(serde::Deserialize)]
pub struct ItemData {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ID")]
    pub id: i16,
    #[serde(rename = "SID")]
    pub sid: String,
    #[serde(rename = "Type")]
    pub type_: i64, // 避讳关键字 "type"
    #[serde(rename = "SubID")]
    pub sub_id: i64,
    #[serde(rename = "MiningFrom")]
    pub mining_from: String,
    #[serde(rename = "ProduceFrom")]
    pub produce_from: String,
    #[serde(rename = "StackSize")]
    pub stack_size: i64,
    #[serde(rename = "Grade")]
    pub grade: i64,
    #[serde(rename = "Upgrades")]
    pub upgrades: Vec<i64>,
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
    pub model_index: i64,
    #[serde(rename = "ModelCount")]
    pub model_count: i64,
    #[serde(rename = "HpMax")]
    pub hp_max: i64,
    #[serde(rename = "Ability")]
    pub ability: i64,
    #[serde(rename = "HeatValue")]
    pub heat_value: i64,
    #[serde(rename = "Potential")]
    pub potential: i64,
    #[serde(rename = "ReactorInc")]
    pub reactor_inc: f64,
    #[serde(rename = "FuelType")]
    pub fuel_type: i64,
    #[serde(rename = "AmmoType")]
    pub ammo_type: i64,
    #[serde(rename = "BombType")]
    pub bomb_type: i64,
    #[serde(rename = "CraftType")]
    pub craft_type: i64,
    #[serde(rename = "BuildIndex")]
    pub build_index: i64,
    #[serde(rename = "BuildMode")]
    pub build_mode: i64,
    #[serde(rename = "GridIndex")]
    pub grid_index: i64,
    #[serde(rename = "UnlockKey")]
    pub unlock_key: i64,
    #[serde(rename = "PreTechOverride")]
    pub pre_tech_override: i64,
    #[serde(rename = "Productive")]
    pub productive: bool,
    #[serde(rename = "MechaMaterialID")]
    pub mecha_material_id: i64,
    #[serde(rename = "DropRate")]
    pub drop_rate: f64,
    #[serde(rename = "EnemyDropLevel")]
    pub enemy_drop_level: i64,
    #[serde(rename = "EnemyDropRange")]
    pub enemy_drop_range: Vector2,
    #[serde(rename = "EnemyDropCount")]
    pub enemy_drop_count: f64,
    #[serde(rename = "EnemyDropMask")]
    pub enemy_drop_mask: i64,
    #[serde(rename = "EnemyDropMaskRatio")]
    pub enemy_drop_mask_ratio: f64,
    #[serde(rename = "DescFields")]
    pub desc_fields: Vec<i64>,
    #[serde(rename = "Description")]
    pub description: String,
}
