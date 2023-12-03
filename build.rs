use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let assets_dir = Path::new("assets");
    let mut assets_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(assets_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let (Some(path), Some(parent)) = (entry.path().to_str(), entry.path().parent().and_then(|p| p.to_str())) {
                let parent_key = parent.strip_prefix("assets/").unwrap_or("root/").to_string();
                let parent_to_strip = format!("{}{}",parent,"/");
                let path_value = path.strip_prefix(&parent_to_strip).unwrap().to_string();   
                assets_map.entry(parent_key).or_insert_with(Vec::new).push(path_value);
            }
        }
    }

    // Optionally sort each vector of paths
    for (_, paths) in assets_map.iter_mut() {
        paths.sort();
    }

    let yaml_string = serde_yaml::to_string(&assets_map)
        .expect("Failed to serialize asset paths to YAML");

    let mut file = File::create(Path::new("assets/assets.yaml"))?;
    file.write_all(yaml_string.as_bytes())?;

    Ok(())
}
