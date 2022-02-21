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
}

impl Float64 {
    pub fn new(value: f64) -> Self {
        Float64 { value }
    }

    pub fn integer_decode(&self) -> (u64, i16, i8) {
        let bits: u64 = unsafe { mem::transmute(&self.value) };
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };

        exponent -= 1023 + 52;
        (mantissa, exponent, sign)
    }
}

impl Hash for Float64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.integer_decode().hash(state)
    }
}

impl PartialEq for Float64 {
    fn eq(&self, other: &Float64) -> bool {
        let value = self.integer_decode();
        let other = other.integer_decode();

        return value.0 == other.0 && value.1 == other.1 && value.2 == other.2;
    }

    fn ne(&self, other: &Self) -> bool {
        self.integer_decode() != other.integer_decode()
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
    fn float64_hash_words() {
        let t = Type::Float64(Float64::new(2.1994));

        match t {
            Type::Float64(a) => assert!(a.eq(&Float64::new(2.1994))),
            _ => panic!("These values aren't equal"),
        }
    }
}
