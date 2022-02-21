use crate::common::variable_type::Type;

#[derive(Hash, PartialEq, Eq)]
pub struct Condition {
    pub variable_name: String,
    pub value: Type,
}

#[derive(Hash, PartialEq, Eq)]
pub struct JsonCondition {
    pub variable_name: String,
    pub value: String,
    pub operator: String,
}

enum Action {
    ExpressiveAction(ExpressiveAction),
    StaticAction(StaticAction),
}

pub struct StaticAction {}

pub struct JsonStaticAction {}

pub struct ExpressiveAction {}

pub struct JsonExpressiveAction {}
