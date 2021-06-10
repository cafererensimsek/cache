use chrono::Duration;
use memoization::cache;

use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

//use chrono::{Local, NaiveDate};
use chrono::Local;
use lazy_static::lazy_static;

type SharedAny = Arc<RwLock<dyn Any + Sync + Send>>;

lazy_static! {
    static ref CACHE: RwLock<HashMap<String, HashMap<String, SharedAny>>> =
        RwLock::new(HashMap::new());
}

fn wrap_shared<T: 'static + Sync + Send>(any: T) -> SharedAny {
    Arc::new(RwLock::new(any))
}

pub fn write_cache(
    function_name: String,
    params: Vec<String>,
    valid_to: SharedAny,
    return_value: SharedAny,
) {
    let mut key = function_name;
    key.push('-');
    key.push_str(&params.join("-"));

    let mut value: HashMap<String, SharedAny> = HashMap::new();

    value.insert("value".to_string(), return_value);
    value.insert("valid_to".to_string(), valid_to);

    CACHE.write().unwrap().insert(key, value);
}

pub fn read_cache(function_name: String, params: Vec<String>) -> Option<SharedAny> {
    let mut key = function_name;

    key.push_str("-");
    key.push_str(&params.join("-"));

    let cache = CACHE.read().unwrap();

    match cache.get(&key) {
        Some(val) => Some(val["value"].clone()),
        None => None,
    }
}

pub fn read_cache_as<T>(function_name: String, params: Vec<String>) -> Option<T>
where
    T: 'static + Clone,
{
    if let Some(shared_val) = read_cache(function_name, params) {
        let any_val = &*shared_val.read().unwrap();

        match any_val.downcast_ref::<T>() {
            Some(v) => Some(v.clone()),
            None => None,
        }
    } else {
        None
    }
}


// Bununla okuyunca clonelamaya gerek kalmıyor ama ben üsttekini kullandım.
pub fn apply_cache_data<T, F, R>(function_name: &String, params: &Vec<String>, func: F) -> Option<R>
where
    F: FnOnce(&T) -> R,
    T: 'static,
{
    if let Some(shared_val) = read_cache(function_name.to_string().to_string(), params.to_vec()) {
        let any_val = &*shared_val.read().unwrap();

        match any_val.downcast_ref::<T>() {
            // No cloning involved. Casts value and passes it to callback.
            Some(v) => Some(func(v)),
            None => None,
        }
    } else {
        None
    }
}

struct Test {
    field1: String,
}

fn main() {

    #[cache]
    pub fn test(bu_geliyo_mu: String, bu_nasil_geliyor: i32) -> Result<Vec<Test>, Error> {
        let function_name = "Zaxd".to_string();
        let date = Local::today().naive_local();
        let params = vec!["zuha".to_string(), "haha".to_string(), "hahaha".to_string()];
        let test = Test {
            field1: "hehe".to_string()
        };
        write_cache(
            function_name.clone(),
            params.clone(),
            wrap_shared(date),
            wrap_shared(18_i32),
        );

        println!("{:#?}", CACHE.read().unwrap());

        let val = read_cache_as::<i32>(function_name, params).unwrap();

        println!("val = {:?}", val);

        let res = vec![test];

        Ok(res.into_iter().map(|t| t).collect::<Vec<Test>>())
    }
}
