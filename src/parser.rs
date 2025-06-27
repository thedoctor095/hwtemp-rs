use glob;
use std::{collections::{HashMap, HashSet}, fs::read_to_string, path::PathBuf, thread::sleep, time::Duration};

#[derive(Debug)]
pub struct Properties {
    pub label: String,
    pub current_temp: f64,
    pub max_temp: f64,
    pub critical_temp: f64
}


// fixed size static array known at compile time
pub static BASE_PATTERNS: [&str; 2]= [
    "/sys/class/hwmon/hwmon*/temp*_*", "/sys/class/hwmon/hwmon*/device/temp*_*"
    ];

pub fn retrieve_paths() -> HashSet<String>{
    let mut hwmon_paths: HashSet<String> =  HashSet::new();
    
    for pattern in BASE_PATTERNS {
        if let Ok(entries) = glob::glob(pattern) {
            for entry in entries {
                match entry {
                    Ok(path) => {
                        let path = normalize(&path);
                        if !path.is_empty(){
                            hwmon_paths.insert(path);
                        } else {
                            continue;
                        }
                },
                    Err(e) => {
                        eprintln!("Could not find files in {:?}: {:?}", pattern, e);
                        continue
                    }
                }
            }
        }
    }
    return hwmon_paths;
}

pub fn normalize(entry: &PathBuf) -> String {
    if let Some(path) = entry.to_str() {
        return path
        .split("_")
        .next()
        .unwrap_or_default()
        .to_string();
    } else {
        return String::new();
    }
}


pub fn parse_temps(hwmons: &HashSet<String>) -> HashMap<String, Vec<Properties>> {
    let mut res: HashMap<String, Vec<Properties>> = HashMap::new();
    for hwmon in hwmons {
        let temp_path = PathBuf::from(
            hwmon
            .to_owned() + "_input"
        );
        let unit_path = PathBuf::from(
            hwmon
            .to_owned())
            .parent()
            .unwrap()
            .join("name"
        );
        let label_path = PathBuf::from(
            hwmon
            .to_owned() + "_label"
        );
        let max_temp_path = PathBuf::from(
            hwmon
            .to_owned() + "_max"
        );
        let crit_temp_path = PathBuf::from(
            hwmon
            .to_owned() + "_crit"
        );


        let unit_name = read_to_string(unit_path)
                                .expect("Could not read sensor unit name")
                                .trim()
                                .to_string();

        let properties: Properties = Properties { 
            label: read_to_string(label_path)
                    .unwrap_or_else(|_| String::new())
                    .trim()
                    .to_string(),
            current_temp: read_temp(temp_path), 
            max_temp: read_temp(max_temp_path), 
            critical_temp: read_temp(crit_temp_path) 
        };

        res
        .entry(unit_name)
        .or_insert_with(Vec::new)
        .push(properties);
    }
    return res;
}


pub fn read_temp(path: PathBuf) -> f64 {
    if let Ok(content) = read_to_string(path) {
            return content
                    .trim()
                    .parse::<f64>()
                    .expect("Could not parse temp file contents") / 1000.0;
    } else {
            return 0.0;
    }
}

pub fn fetch_temps(interval: Option<u64>) -> Option<HashMap<String, Vec<Properties>>> {
    let interval = interval.unwrap_or(0) * 1000;
    sleep(Duration::from_millis(interval));

    let hwmons = retrieve_paths();
    if !hwmons.is_empty() {
        // TODO: work on streaming results;
        loop {
            let temps = parse_temps(&hwmons);
            return Some(temps);
        }
    } else {
        eprintln!("Could not retrieve paths from either of {:?}", BASE_PATTERNS);
        return None;
    }
}

