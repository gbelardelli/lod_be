use sqlx::SqlitePool;
use crate::models::character_model::{CharacterModel, NewCharacter};

pub async fn create_character(pool: &SqlitePool, character: NewCharacter) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO game_character (player_id, specie_id, profession_id, name)
        VALUES (?, ?, ?, ?)
        RETURNING id
        "#,
        character.player_id,
        character.specie_id,
        character.profession_id,
        character.name,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id.unwrap())
}

pub async fn get_all_characters(_pool: &SqlitePool) -> Result<Vec<CharacterModel>, sqlx::Error> {
    // TODO: implement DB fetch all logic
    Ok(vec![])
}

pub async fn get_character_by_id(_pool: &SqlitePool, _id: i64) -> Result<Option<CharacterModel>, sqlx::Error> {
    // TODO: implement DB fetch by id logic
    Ok(None)
}

pub async fn update_character(_pool: &SqlitePool, _id: i64, _character: CharacterModel) -> Result<(), sqlx::Error> {
    // TODO: implement DB update logic
    Ok(())
}

pub async fn delete_character(_pool: &SqlitePool, _id: i64) -> Result<(), sqlx::Error> {
    // TODO: implement DB delete logic
    Ok(())
}