use std::env;
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
        " static ITEMS_ID: phf::Map<&'static str, i16> = {};",
        phf.build()
    )
    .unwrap();
}

fn gen_items_data(items: &ItemProtoSet, file: &mut BufWriter<File>) {
    write!(
        file,
        "fn items_data() -> Vec<ItemData> {{ vec!{:#?} }}",
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
    gen_items_data(&items, &mut file);
}
