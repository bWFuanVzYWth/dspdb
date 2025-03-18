use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

include!("../shared/item.rs");

#[derive(Debug, serde::Deserialize)]
struct GameObject {
    #[serde(rename = "m_FileID")]
    file_id: i64,
    #[serde(rename = "m_PathID")]
    path_id: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct ItemProtoSet<'a> {
    #[serde(rename = "m_GameObject")]
    pub game_object: GameObject,
    #[serde(rename = "m_Enabled")]
    pub enabled: i64,
    #[serde(rename = "m_Script")]
    pub script: GameObject,
    #[serde(rename = "m_Name")]
    pub name: &'a str,
    #[serde(rename = "TableName")]
    pub table_name: &'a str,
    #[serde(rename = "Signature")]
    pub signature: &'a str,
    #[serde(rename = "dataArray")]
    pub data_array: Vec<ItemData>,
}

fn items() -> ItemProtoSet<'static> {
    let json_str = include_str!("../data/ItemProtoSet.json");
    serde_json::from_str(json_str).unwrap()
}

fn gen_items_id(items: &ItemProtoSet, file: &mut BufWriter<File>) {
    let mut phf = phf_codegen::Map::new();
    for item in &items.data_array {
        phf.entry(item.name.clone(), &item.id.to_string());
    }

    writeln!(
        file,
        "static ITEMS_ID: phf::Map<&'static str, i16> = {};",
        phf.build()
    )
    .unwrap();
}

fn gen_items_name(items: &ItemProtoSet, file: &mut BufWriter<File>) {
    let mut phf = phf_codegen::Map::new();
    for item in &items.data_array {
        phf.entry(&item.id, &format!("{:?}", &item.name));
    }

    writeln!(
        file,
        "static ITEMS_NAME: phf::Map<i16, &'static str> = {};",
        phf.build()
    )
    .unwrap();
}

impl fmt::Debug for ItemData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ItemData {{").unwrap();

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
        writeln!(f, "reactor_inc: {:?},", &self.reactor_inc).unwrap();
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
        writeln!(f, "drop_rate: {:?},", &self.drop_rate).unwrap();
        writeln!(f, "enemy_drop_level: {},", &self.enemy_drop_level).unwrap();
        writeln!(f, "enemy_drop_range: {:?},", &self.enemy_drop_range).unwrap();
        writeln!(f, "enemy_drop_count: {:?},", &self.enemy_drop_count).unwrap();
        writeln!(f, "enemy_drop_mask: {},", &self.enemy_drop_mask).unwrap();
        writeln!(
            f,
            "enemy_drop_mask_ratio: {:?},",
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

fn gen_items_data(items: &ItemProtoSet, file: &mut BufWriter<File>) {
    write!(
        file,
        "pub fn items_data() -> Vec<ItemData> {{ vec!{:#?} }}",
        items.data_array
    )
    .unwrap();
}

fn gen_items_struct(file: &mut BufWriter<File>) {
    let struct_string = include_str!("../shared/item.rs");
    write!(file, "{}", struct_string).unwrap();
}

pub fn gen_items() {
    let path = Path::new(&env::var("OUT_DIR").expect("can not get OUT_DIR")).join("dspdb_items.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let items = items();

    gen_items_struct(&mut file);
    gen_items_id(&items, &mut file);
    gen_items_name(&items, &mut file);
    gen_items_data(&items, &mut file);
}
