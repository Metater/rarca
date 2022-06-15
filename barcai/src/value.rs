use std::fmt::Debug;

use crate::constant::BarcaiConstant;

#[derive(Debug, Clone)]
pub enum BarcaiValue {
	Constant(BarcaiConstant),
    I64(i64),
    F64(f64),
    String(String),
}

impl BarcaiValue {
	
}