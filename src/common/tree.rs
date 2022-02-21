use crate::common::variable_type::Type;

enum Condition {
    ExpressiveCondition(StaticCondition),
    StaticCondition(ExpressiveCondition),
}

#[derive(Hash, PartialEq, Eq)]
pub struct StaticCondition {
    pub variable_name: String,
    pub value: Type,
}

#[derive(Hash, PartialEq, Eq)]
pub struct JsonStaticCondition {
    pub variable_name: String,
    pub value: String,
    pub operator: String,
}

pub struct ExpressiveCondition {}

pub struct JsonExpressiveCondition {}

enum Action {
    ExpressiveAction(ExpressiveAction),
    StaticAction(StaticAction),
}

pub struct StaticAction {}

pub struct JsonStaticAction {}

pub struct ExpressiveAction {}

pub struct JsonExpressiveAction {}
