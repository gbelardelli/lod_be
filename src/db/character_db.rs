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

pub async fn get_all_characters(pool: &SqlitePool) -> Result<Vec<CharacterModel>, sqlx::Error> {
    let rows = sqlx::query_as!(
        CharacterModel,
        r#"
        SELECT id, player_id, specie_id, profession_id, name, level, experience, stats, skills, effects
        FROM game_character
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_character_by_id(pool: &SqlitePool, id: i64) -> Result<Option<CharacterModel>, sqlx::Error> {
    let character = sqlx::query_as::<_, CharacterModel>(
        r#"SELECT id, player_id, specie_id, profession_id, name, level, experience, stats, skills, effects
           FROM game_character 
           WHERE id = ?"#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(character)
}

pub async fn update_character(pool: &SqlitePool, id: i64, character: CharacterModel) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE game_character
         SET name = ?, level = ?, experience = ?, stats = ?, skills = ?, effects = ?
         WHERE id = ?"
    )
    .bind(&character.name)
    .bind(character.level)
    .bind(character.experience)
    .bind(&character.stats)
    .bind(&character.skills)
    .bind(&character.effects)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_character(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM game_character WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}