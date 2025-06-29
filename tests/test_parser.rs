use std::env;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use hwtemp_rs::parser::{fetch_temps, normalize, read_temp};

    use super::*;

    #[test]
    fn test_successful_read_temp(){
        let test_path = PathBuf::from("tests/test_temps/test_temp1");
        let cwd = env::current_dir().unwrap().join(test_path);
        assert_eq!(read_temp(cwd.into()), 1.0 / 1000.0);
    }

    #[test]
    fn test_failed1_read_temp(){
        let test_path = &PathBuf::from("tests/test_temps/test_no_temp1");
        let cwd = env::current_dir().unwrap().join(test_path);
        assert_eq!(read_temp(cwd), 0.0);
    }

    #[test]
    fn test_invalid_utf_read_temp() {
        let test_path = &PathBuf::from("tests/test_temps/test_invalid_utf_temp");
        let cwd = env::current_dir().unwrap().join(test_path);
        assert_eq!(read_temp(cwd), 0.0);
    }

    #[test]
    fn test_successful_normalize() {
        let entry = &PathBuf::from("tests/test_temps/test_temp1");
        assert_eq!(normalize(entry), "tests/test_temps/test");
    }

    // test will fail locally but succeed on a system without hwmon interface
    #[test]
    fn test_failed_fetch_temps() {
        assert!(fetch_temps(Some(0)).is_none());
    }
}