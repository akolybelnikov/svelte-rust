use std::collections::HashMap;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let mut map: HashMap<String, Vec<f32>> = HashMap::new();
    let (keys, values) = input.split_once(';').unwrap();
    let keys: Vec<_> = keys.split(',').collect();
    let mut temp: Vec<Vec<f32>> = Vec::with_capacity(keys.len());
    for _ in 0..keys.len() {
        temp.push(Vec::new());
    }
    for row in values.split(';') {
        for (i, v) in row.split(',').enumerate() {
            temp[i].push(v.parse().unwrap());
        }
    }
    for (k, v) in keys.into_iter().zip(temp.into_iter()) {
        map.insert(k.to_owned(), v);
    }
    JsValue::from_serde(&map).unwrap()
}
