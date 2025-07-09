use sqlx::SqlitePool;
use crate::models::game_character_model::GameCharacter;

pub async fn create_character(_pool: &SqlitePool, _character: GameCharacter) -> Result<i64, sqlx::Error> {
    // TODO: implement DB insert logic
    Ok(0)
}

pub async fn get_all_characters(_pool: &SqlitePool) -> Result<Vec<GameCharacter>, sqlx::Error> {
    // TODO: implement DB fetch all logic
    Ok(vec![])
}

pub async fn get_character_by_id(_pool: &SqlitePool, _id: i64) -> Result<Option<GameCharacter>, sqlx::Error> {
    // TODO: implement DB fetch by id logic
    Ok(None)
}

pub async fn update_character(_pool: &SqlitePool, _id: i64, _character: GameCharacter) -> Result<(), sqlx::Error> {
    // TODO: implement DB update logic
    Ok(())
}

pub async fn delete_character(_pool: &SqlitePool, _id: i64) -> Result<(), sqlx::Error> {
    // TODO: implement DB delete logic
    Ok(())
}