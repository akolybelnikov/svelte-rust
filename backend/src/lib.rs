pub mod parser;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(a: usize);
}

#[wasm_bindgen]
pub fn greet(_a: usize) {
    log("Hello, rust!");
}

pub struct OwnerID {
    id: usize,
}

#[wasm_bindgen]
pub struct Car {
    pub number: usize,
    pub color: usize,
    owner: OwnerID,
}

#[wasm_bindgen]
pub fn color(car: &Car, color: usize) -> Car {
    Car {
        number: car.number,
        color,
        owner: OwnerID { id: car.owner.id },
    }
}

#[wasm_bindgen]
impl Car {
    pub fn new() -> Self {
        Car {
            number: 0,
            color: 0,
            owner: OwnerID { id: 0 },
        }
    }
    pub fn duplicate(&self) -> Self {
        Self {
            number: self.number + 1,
            color: self.color,
            owner: OwnerID { id: self.owner.id },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Car, color, OwnerID};

    #[test]
    fn it_works() {
        let c: usize = 456789;
        let car = Car {
            number: 12345,
            color: c,
            owner: OwnerID { id: 0 },
        };
        let car2 = color(&car, c);
        assert_eq!(car.color, car2.color);
    }
}
