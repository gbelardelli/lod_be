use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
pub struct GameCharacterRow {
    pub id: i64,
    pub player_id: i64,
    pub specie_id: i64,
    pub profession_id: i64,
    pub name: String,
    pub level: Option<i64>,
    pub experience: Option<i64>,
    pub stats: Option<String>,
    pub skills: Option<String>,
    pub effects: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CharacterModel {
    pub id: i64,
    pub player_id: i64,
    pub specie_id: i64,
    pub profession_id: i64,
    pub name: String,
    pub level: Option<i64>,
    pub experience: Option<i64>,
    pub stats: Option<String>,
    pub skills: Option<CharacterSkills>,
    pub effects: Option<String>
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
    strength: i16,
    db: i16,
    constitution: i16,
    natual_armour: i16,
    dexterity: i16,
    wisdom: i16,
    resolve: i16,
    movement: i16,
    hp: i16,
    weapon_class1: i16,
    weapon_class2: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterSkills {
    cs: i16,
    rs: i16,
    dodge: i16,
    pick_locks: i16,
    barter: i16,
    heal: i16,
    alchemy: i16,
    perception: i16,
    foraging: i16,
    prayers: i16,
    arcane_arts: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weapon {
    name: String,
    slot: String,
    dmg: i16,
    wclass: String,
    encumbrance: i16,
    special: String,
    tot_dmg: i16,
    durability: i16,
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
