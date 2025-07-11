use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CharacterModel {
    pub id: i64,
    pub player_id: i64,
    pub specie_id: i64,
    pub profession_id: i64,
    pub name: String,
    pub level: u32,
    pub experience: u32,
    pub stats: String,
    pub skills: String,
    pub effects: String
}

impl From<web::Json<CharacterModel>> for CharacterModel {
    fn from(value: web::Json<CharacterModel>) -> Self {
        value.0
    }
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct NewCharacter {
    pub player_id: i64,
    pub specie_id: i64,
    pub profession_id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct CharacterFull {
    pub id: i64,
    pub player: String,
    pub specie: String,
    pub profession: String,
    pub name: String,
    pub level: u32,
    pub experience: u32,    
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterStats {
    strength: u16,
    db: u16,
    constitution: u16,
    natual_armour: u16,
    dexterity: u16,
    wisdom: u16,
    resolve: u16,
    movement: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterSkills {
    cs: u16,
    rs: u16,
    dodge: u16,
    pick_locks: u16,
    barter: u16,
    heal: u16,
    alchemy: u16,
    perception: u16,
    foraging: u16,
    prayers: u16,
    arcane_arts: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Perk {
    name: String,
    effect: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Talent {
    name: String,
    effect: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameCharacterPlayerLong {
    pub player_name: String,
    pub player_color: u32,
    pub specie: String,
    pub profession: String,
    pub name: String,
    pub level: u32,
    pub experience: u32,
    pub stats: CharacterStats,
    pub skills: CharacterSkills,
    pub effects: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameCharacterPlayerShort {
    pub player_name: String,
    pub player_color: u32,
    pub specie: String,
    pub profession: String,
    pub name: String,
    pub level: u32,
}
