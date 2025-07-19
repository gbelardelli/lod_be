use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize,sqlx::FromRow)]
pub struct SpecieModel {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub start_stats: String,
}

#[derive(Debug, Deserialize, Serialize,sqlx::FromRow)]
pub struct ProfessionModel {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub start_gear: Option<String>,
    pub notes: Option<String>,
    pub start_stats: String,
}

#[derive(Debug, Deserialize, Serialize,sqlx::FromRow)]
pub struct WeaponModel {
    pub id: i64,
    pub name: String,
    pub damage: String,
    pub encumbrance: i64,
    pub class: i64,
    pub special: Option<String>,
    pub cost: i64,
    pub availability: i64,
    pub reload: i64,
}

#[derive(Debug, Deserialize, Serialize,sqlx::FromRow)]
pub struct InfoModel {
    pub species: Vec<SpecieModel>,
    pub professions: Vec<ProfessionModel>,
    pub weapons: Vec<WeaponModel>,
}