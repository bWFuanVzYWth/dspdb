use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

include!("../shared/recipe.rs");

#[derive(Debug, serde::Deserialize)]
struct GameObject {
    #[serde(rename = "m_FileID")]
    file_id: i64,
    #[serde(rename = "m_PathID")]
    path_id: i64,
}

#[derive(Debug, serde::Deserialize)]
struct ScriptInfo {
    #[serde(rename = "m_FileID")]
    file_id: i64,
    #[serde(rename = "m_PathID")]
    path_id: i64,
}

#[derive(Debug, serde::Deserialize)]
struct RecipeProtoSet {
    #[serde(rename = "m_GameObject")]
    game_object: GameObject,
    #[serde(rename = "m_Enabled")]
    enabled: i64,
    #[serde(rename = "m_Script")]
    script: ScriptInfo,
    #[serde(rename = "m_Name")]
    name: String,
    #[serde(rename = "TableName")]
    table_name: String,
    #[serde(rename = "Signature")]
    signature: String,
    #[serde(rename = "dataArray")]
    data_array: Vec<RecipeItem>,
}

impl fmt::Debug for RecipeItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RecipeItem {{").unwrap();

        writeln!(f, "name: String::from({:?}),", &self.name).unwrap();
        writeln!(f, "id: {},", &self.id).unwrap();
        writeln!(f, "sid: String::from({:?}),", &self.sid).unwrap();
        writeln!(f, "type_: {},", &self.type_).unwrap();
        writeln!(f, "handcraft: {},", &self.handcraft).unwrap();
        writeln!(f, "explicit: {},", &self.explicit).unwrap();
        writeln!(f, "time_spend: {},", &self.time_spend).unwrap();
        writeln!(f, "items: {:?}.to_vec(),", &self.items).unwrap();
        writeln!(f, "item_counts: {:?}.to_vec(),", &self.item_counts).unwrap();
        writeln!(f, "results: {:?}.to_vec(),", &self.results).unwrap();
        writeln!(f, "result_counts: {:?}.to_vec(),", &self.result_counts).unwrap();
        writeln!(f, "grid_index: {},", &self.grid_index).unwrap();
        writeln!(f, "icon_path: String::from(\"{}\"),", &self.icon_path).unwrap();
        writeln!(f, "description: String::from({:?}),", &self.description).unwrap();
        writeln!(f, "non_productive: {},", &self.non_productive).unwrap();

        // 完成调试输出
        writeln!(f, "}}").unwrap();

        Ok(())
    }
}

fn recipes() -> RecipeProtoSet {
    let json_str = include_str!("../data/RecipeProtoSet.json");
    serde_json::from_str(json_str).unwrap()
}

fn gen_recipes_struct(file: &mut BufWriter<File>) {
    let struct_string = include_str!("../shared/recipe.rs");
    write!(file, "{}", struct_string).unwrap();
}

fn gen_recipes_data(recipes: &RecipeProtoSet, file: &mut BufWriter<File>) {
    write!(
        file,
        "fn recipes_data() -> Vec<RecipeItem> {{ vec!{:#?} }}",
        recipes.data_array
    )
    .unwrap();
}

pub fn gen_recipes() {
    let path =
        Path::new(&env::var("OUT_DIR").expect("can not get OUT_DIR")).join("dspdb_recipes.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let recipes = recipes();

    gen_recipes_struct(&mut file);
    gen_recipes_data(&recipes, &mut file);
}
