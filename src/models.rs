use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct TibToken {
    pub size: i32,
    #[serde(rename = "hex")]
    pub hex_str: String,
    pub text: String,
}

pub type TokenMap = HashMap<String, TibToken>;
