pub mod parser;

use parser::{Properties, fetch_temps};
use std::{collections::HashMap, iter::from_fn, thread::sleep, time::Duration};

pub type HWProperties = Option<HashMap<String, Vec<Properties>>>;

pub fn from_stream(interval: Option<u64>) -> impl Iterator<Item = HWProperties>{
    let interval = interval.unwrap_or(0) * 1000;
    let mut first_iter = true;
    from_fn(move || {
        if !first_iter {
            sleep(Duration::from_millis(interval));
        } else {
            first_iter = false;
        }
        return Some(fetch_temps());
    })
}

pub fn from_func() -> HWProperties {
    return fetch_temps();
}
