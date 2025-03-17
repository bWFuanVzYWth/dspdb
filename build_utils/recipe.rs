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

fn recipes() -> RecipeProtoSet {
    let json_str = include_str!("../data/RecipeProtoSet.json");
    serde_json::from_str(json_str).unwrap()
}
