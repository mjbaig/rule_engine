use std::mem;

use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
pub enum Type {
    String(String),
    Int32(i32),
    Int64(i64),
    Float64(Float64),
}

pub struct Float64 {
    pub value: f64,
    integral: i64,
    fractional: i64,
}

impl Float64 {
    pub fn new(value: f64) -> Self {
        let integral = value as i64;

        let fractional = ((value - (integral as f64)) * 1000.0) as i64;

        Float64 {
            value,
            integral,
            fractional,
        }
    }
}

impl Hash for Float64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.integral, self.fractional).hash(state)
    }
}

impl PartialEq for Float64 {
    fn eq(&self, other: &Float64) -> bool {
        self.fractional == other.fractional && self.integral == other.integral
    }

    fn ne(&self, other: &Self) -> bool {
        self.fractional != other.fractional || self.integral != other.integral
    }
}

impl Eq for Float64 {}

#[cfg(test)]
mod tests {

    use crate::common::variable_type::*;

    #[test]
    fn it_works() {
        let t = Type::String("".to_string());

        match t {
            Type::String(a) => assert!(a == "".to_string()),
            _ => panic!("this value don't match"),
        }
    }

    #[test]
    fn it_works2() {
        let t: f32 = 1.5;

        let y: i32 = t as i32;

        let f = ((t - (y as f32)) * 1000.0) as i32;

        println!("{}.{}", y, f);
    }

    #[test]
    fn float64_hash_words() {
        let t = Type::Float64(Float64::new(2.19));

        match t {
            Type::Float64(a) => assert!(a.eq(&Float64::new(2.19))),
            _ => panic!("These values aren't equal"),
        }
    }
}
