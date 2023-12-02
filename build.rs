use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

//recursive functin to loop over files and insert into hashmap, then enter subdirs and repeat
fn recursive_insert(dir : , hashmap: &mut HashMap<String, Vec<String>>) {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let (Some(path), Some(parent)) = (entry.path().to_str(), entry.path().parent().and_then(|p| p.to_str())) {
                let dir = String::from(parent.strip_prefix("assets")
                    .expect("Failed to strip dir prefix"));
                let filename = path.strip_prefix(parent)
                    .expect("Failed to strip path prefix")
                    .to_string();
                assets_map.entry(dir).or_insert_with(Vec::new).push(filename);
            }
        }
    }

    hashmap.entry(key).or_insert_with(Vec::new).push(value);
}

fn main() -> std::io::Result<()> {
    let assets_dir = Path::new("assets");
    let mut assets_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(assets_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let (Some(path), Some(parent)) = (entry.path().to_str(), entry.path().parent().and_then(|p| p.to_str())) {
                let dir = String::from(parent.strip_prefix("assets")
                    .expect("Failed to strip dir prefix"));
                let filename = path.strip_prefix(parent)
                    .expect("Failed to strip path prefix")
                    .to_string();
                assets_map.entry(dir).or_insert_with(Vec::new).push(filename);
            }
        }
    }

    // Optionally sort each vector of paths
    for (_, paths) in assets_map.iter_mut() {
        paths.sort();
    }

    let yaml_string = serde_yaml::to_string(&assets_map)
        .expect("Failed to serialize asset paths to YAML");

    let mut file = File::create(Path::new("assets.yaml"))?;
    file.write_all(yaml_string.as_bytes())?;

    Ok(())
}