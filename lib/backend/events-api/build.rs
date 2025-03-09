use std::collections::HashMap;

fn main() {
    let db_structure = include_str!("./database-structure.json");
    let constants: HashMap<String, String> = serde_json::from_str(db_structure).unwrap();
    let db_structure_mod = constants
        .iter()
        .map(|(k, v)| {
            format!(
                "#[allow(dead_code)]\npub const {}: &str = \"{}\";",
                k.to_uppercase(),
                v
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("db_structure.rs");
    std::fs::write(&dest_path, db_structure_mod).unwrap();
}
