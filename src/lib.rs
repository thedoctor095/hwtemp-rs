pub mod parser;

use parser::{Properties, fetch_temps};
use std::{collections::HashMap, iter::from_fn};

pub fn from_stream(interval: Option<u64>) -> impl Iterator<Item = Option<HashMap<String, Vec<Properties>>>>{
    from_fn(move || {
        Some(fetch_temps(interval))
    })
}

pub fn from_func() -> Option<HashMap<String, Vec<Properties>>> {
    return fetch_temps(Some(0));
}