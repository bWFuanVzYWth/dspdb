include!(concat!(env!("OUT_DIR"), "\\dspdb_items.rs"));

pub fn item_name(id: i16) -> Option<String> {
    ITEMS_NAME.get(&id).map(|name| name.to_string())
}

pub fn item_id(name: &str) -> Option<i16> {
    ITEMS_ID.get(name).copied()
}
