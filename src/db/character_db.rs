use sqlx::SqlitePool;
use crate::models::character_model::{CharacterModel, CharacterSkills, CharacterStats, GameCharacterRow, NewCharacter};

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
        GameCharacterRow,
        r#"
        SELECT id, player_id, specie_id,
            profession_id, name, level,
            experience, condition, comment,
            stats, skills, effects
        FROM game_character
        "#
    )
    .fetch_all(pool)
    .await?;

    let characters = rows
        .into_iter()
        .map(|row| {
            let stats = row.stats
                .as_ref()
                .and_then(|s| serde_json::from_str::<CharacterStats>(s).ok());

            let skills = row.skills
                .as_ref()
                .and_then(|s| serde_json::from_str::<CharacterSkills>(s).ok());

            CharacterModel {
                id: row.id,
                player_id: row.player_id,
                specie_id: row.specie_id,
                profession_id: row.profession_id,
                name: row.name,
                level: row.level,
                experience: row.experience,
                condition: row.condition,
                comment: row.comment,
                stats,
                skills,
                effects: row.effects,
            }
        })
        .collect();

    Ok(characters)  
}

pub async fn get_character_by_id(pool: &SqlitePool, id: i64) -> Result<Option<CharacterModel>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, player_id, specie_id,
                profession_id, name, level,
                experience, condition, comment,
                stats, skills, effects
        FROM game_character WHERE id = ?
        "#,
        id
    )
    .fetch_one(pool)
    .await?;    

    Ok(Some(CharacterModel {
        id: row.id,
        player_id: row.player_id,
        specie_id: row.specie_id,
        profession_id: row.profession_id,
        name: row.name,
        level: row.level,
        experience: row.experience,
        condition: row.condition,
        comment: row.comment,
        stats: row.stats.and_then(|s| serde_json::from_str(&s).ok()),
        skills: row.skills.and_then(|s| serde_json::from_str(&s).ok()),
        effects: row.effects.and_then(|s| serde_json::from_str(&s).ok()),
    }))
}

pub async fn update_character(pool: &SqlitePool, id: i64, character: CharacterModel) -> Result<(), sqlx::Error> {
    let skills_json = serde_json::to_string(&character.skills).map_err(|e| {
        sqlx::Error::Decode(Box::new(e))
    })?;

    let stats_json = serde_json::to_string(&character.stats).map_err(|e| {
        sqlx::Error::Decode(Box::new(e))
    })?;
    sqlx::query(
        "UPDATE game_character
         SET name = ?, level = ?, experience = ?, stats = ?, skills = ?, effects = ?
         WHERE id = ?"
    )
    .bind(&character.name)
    .bind(character.level)
    .bind(character.experience)
    .bind(&stats_json)
    .bind(&skills_json)
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