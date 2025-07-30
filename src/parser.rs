use glob;
use std::{collections::{HashMap, HashSet}, fs::read_to_string, path::PathBuf};

#[derive(Debug)]
pub struct Properties {
    label: String,
    current_temp: f64,
    max_temp: f64,
    critical_temp: f64,
    sensor_path: String
}


static BASE_PATTERNS: [&str; 2]= [
    "/sys/class/hwmon/hwmon*/temp*_*", "/sys/class/hwmon/hwmon*/device/temp*_*"
    ];

fn retrieve_hwmon_paths() -> HashSet<String>{
    // no need in having them ordered
    let mut hwmon_paths: HashSet<String> =  HashSet::new();
    
    for pattern in BASE_PATTERNS {
        if let Ok(entries) = glob::glob(pattern) {
            for entry in entries {
                match entry {
                    Ok(path) => {
                        let path = normalize(&path);
                        if !path.is_empty() {
                            hwmon_paths.insert(path.to_owned());
                        } else {
                            continue;
                        }
                },
                    Err(e) => {
                        eprintln!("Could not find files in {pattern}: {e}");
                        continue
                    }
                }
            }
        }
    }
    hwmon_paths
}

pub fn normalize(entry: &PathBuf) -> &str {
    if let Some(path) = entry.to_str() {
        path
        .split("_")
        .next()
        .unwrap_or_default()
    } else {
        ""
    }
}


fn parse_temps(hwmons: HashSet<String>) -> HashMap<String, Vec<Properties>> {
    let mut result: HashMap<String, Vec<Properties>> = HashMap::new();
    for hwmon in hwmons {
        let temp_path = PathBuf::from(
            hwmon
            .clone() + "_input"
        );
        let unit_path = PathBuf::from(
            hwmon
            .clone())
            .parent()
            .expect("Could not extract sensor unit parent path")
            .join("name");
        let label_path = PathBuf::from(
            hwmon
            .clone() + "_label"
        );
        let max_temp_path = PathBuf::from(
            hwmon
            .clone() + "_max"
        );
        let crit_temp_path = PathBuf::from(
            hwmon
            .clone() + "_crit"
        );


        let unit_name = read_to_string(&unit_path)
                                .expect("Could not read sensor unit name")
                                .trim()
                                .to_string();

        let properties = Properties {
            label: read_to_string(&label_path)
                    .unwrap_or_else(|_| String::new() // some sensors do not have a temp*_label file such as acpitz
                    )
                    .trim()
                    .to_string(),
            current_temp: read_temp(&temp_path),
            max_temp: read_temp(&max_temp_path),
            critical_temp: read_temp(&crit_temp_path),
            sensor_path: hwmon + "_*"
        };
        result.entry(unit_name).or_insert_with(Vec::new).push(properties);
    }
    result
}


pub fn read_temp(path: &PathBuf) -> f64 {
    if let Ok(content) = read_to_string(path) {
            content
            .trim()
            .parse::<f64>()
            .unwrap_or(0.0) / 1000.0
    } else {
            0.0
    }
}

pub fn fetch_temps() -> Option<HashMap<String, Vec<Properties>>> {
    let hwmons = retrieve_hwmon_paths();
    if !hwmons.is_empty() {
        let temps = parse_temps(hwmons);
        Some(temps)
    } else {
        eprintln!("Could not retrieve paths from either of globs {:?}", BASE_PATTERNS);
        None
    }
}
