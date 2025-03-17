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

use std::fmt;

// 手动实现 Debug trait
impl fmt::Debug for ItemData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ItemData{{").unwrap();

        writeln!(f, "name: String::from({:?}),", &self.name).unwrap();
        writeln!(f, "id: {},", &self.id).unwrap();
        writeln!(f, "sid: String::from({:?}),", &self.sid).unwrap();
        writeln!(f, "type_: {},", &self.type_).unwrap();
        writeln!(f, "sub_id: {},", &self.sub_id).unwrap();
        writeln!(f, "mining_from: String::from({:?}),", &self.mining_from).unwrap();
        writeln!(f, "produce_from: String::from({:?}),", &self.produce_from).unwrap();
        writeln!(f, "stack_size: {},", &self.stack_size).unwrap();
        writeln!(f, "grade: {},", &self.grade).unwrap();
        writeln!(f, "upgrades: {:?}.to_vec(),", &self.upgrades).unwrap();
        writeln!(f, "is_fluid: {},", &self.is_fluid).unwrap();
        writeln!(f, "is_entity: {},", &self.is_entity).unwrap();
        writeln!(f, "can_build: {},", &self.can_build).unwrap();
        writeln!(f, "build_in_gas: {},", &self.build_in_gas).unwrap();
        writeln!(f, "icon_path: String::from(\"{}\"),", &self.icon_path).unwrap();
        writeln!(f, "model_index: {},", &self.model_index).unwrap();
        writeln!(f, "model_count: {},", &self.model_count).unwrap();
        writeln!(f, "hp_max: {},", &self.hp_max).unwrap();
        writeln!(f, "ability: {},", &self.ability).unwrap();
        writeln!(f, "heat_value: {},", &self.heat_value).unwrap();
        writeln!(f, "potential: {},", &self.potential).unwrap();
        writeln!(f, "reactor_inc: {:.15},", &self.reactor_inc).unwrap();
        writeln!(f, "fuel_type: {},", &self.fuel_type).unwrap();
        writeln!(f, "ammo_type: {},", &self.ammo_type).unwrap();
        writeln!(f, "bomb_type: {},", &self.bomb_type).unwrap();
        writeln!(f, "craft_type: {},", &self.craft_type).unwrap();
        writeln!(f, "build_index: {},", &self.build_index).unwrap();
        writeln!(f, "build_mode: {},", &self.build_mode).unwrap();
        writeln!(f, "grid_index: {},", &self.grid_index).unwrap();
        writeln!(f, "unlock_key: {},", &self.unlock_key).unwrap();
        writeln!(f, "pre_tech_override: {},", &self.pre_tech_override).unwrap();
        writeln!(f, "productive: {},", &self.productive).unwrap();
        writeln!(f, "mecha_material_id: {},", &self.mecha_material_id).unwrap();
        writeln!(f, "drop_rate: {:.15},", &self.drop_rate).unwrap();
        writeln!(f, "enemy_drop_level: {},", &self.enemy_drop_level).unwrap();
        writeln!(f, "enemy_drop_range: {:?},", &self.enemy_drop_range).unwrap();
        writeln!(f, "enemy_drop_count: {:.15},", &self.enemy_drop_count).unwrap();
        writeln!(f, "enemy_drop_mask: {},", &self.enemy_drop_mask).unwrap();
        writeln!(
            f,
            "enemy_drop_mask_ratio: {:.15},",
            &self.enemy_drop_mask_ratio
        )
        .unwrap();
        writeln!(f, "desc_fields: {:?}.to_vec(),", &self.desc_fields).unwrap();
        writeln!(f, "description: String::from({:?}),", &self.description).unwrap();

        // 完成调试输出
        writeln!(f, "}}").unwrap();

        Ok(())
    }
}
