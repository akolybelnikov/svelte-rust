use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

use backend::parser::parse;

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use super::*;

    #[wasm_bindgen_test]
    fn parser_works() {
        let input = "a,b;1.1,2.1;4.7,5.3";
        let get = parse(input);
        let mut want_map: HashMap<String, Vec<f32>> = HashMap::new();
        want_map.insert("a".to_owned(), vec![1.1, 4.7]);
        want_map.insert("b".to_owned(), vec![2.1, 5.3]);
        let want = JsValue::from_serde(&want_map).unwrap();
        assert!(get.is_object() && want.is_object());

        let get_map: HashMap<String, Vec<f32>> = get.into_serde().unwrap();
        assert_eq!(get_map.len(), want_map.len());
        assert!(get_map.keys().eq(want_map.keys()));
        for (k, v) in get_map.iter() {
            assert_eq!(v, want_map.get(k).unwrap());
        }
    }
}