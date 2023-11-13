use crate::model::coefficient::Coefficient;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Objective {
    pub name: String,
    pub coefficients: Vec<Coefficient>,
}
