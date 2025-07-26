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
    pub condition: Option<String>,
    pub comment: Option<String>,
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
    pub condition: Option<String>,
    pub comment: Option<String>,
    pub stats: Option<CharacterStats>,
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
pub struct CharacterPlayerRow {
    pub id: i64,
    pub player: String,
    pub specie: String,
    pub profession: String,
    pub name: String,
    pub level: i64,
    pub experience: i64,    
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
    encumbrance: i16,    
    movement: i8,
    hp: i8,
    luck: i8,
    energy: i8,
    sanity: i8,
    weapon_class1: i8,
    weapon_class2: i8,
    morale: i8,     // Morale che aggiunge al gruppo
    coin: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterSkills {
    pub cs: i16,
    pub rs: i16,
    pub dodge: i16,
    pub pick_locks: i16,
    pub barter: i16,
    pub heal: i16,
    pub alchemy: i16,
    pub perception: i16,
    pub foraging: i16,
    pub prayers: i16,
    pub arcane_arts: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Weapon {
    name: String,
    slot: i8,           // 0 = Nothing, 1=Hand, 2=Shield, 3=Both
    dmg: String,        // { "dice" : [{ "faces": 6, "roll": 1 }], "mod" : 7 }
    wclass: i8,
    encumbrance: u8,
    special: String,
    tot_dmg: String,    // { "dice" : [{ "faces": 6, "roll": 1 }], "mod" : 7 }
    durability: i8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Armour {
    name: String,
    def: i8,
    encumbrance: u8,
    special: String,
    durability: i8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Gear {
    head: Vec<Armour>,
    arms: Vec<Armour>,
    shield: Vec<Armour>,
    torso: Vec<Armour>,
    legs: Vec<Armour>,
    quick: Vec<Armour>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Perk {
    id: i64,
    name: String,
    effect: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Talent {
    id: i64,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameCharacterPlayerShort {
    pub id: i64,
    pub specie: String,
    pub profession: String,
    pub name: String,
    pub level: u16,
}
