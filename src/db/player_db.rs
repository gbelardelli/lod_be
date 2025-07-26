use std::collections::HashMap;

use actix_session::Session;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use crate::models::{character_model::GameCharacterPlayerShort, player_model::{PlayerData, PlayerModel}, session_model::SessionModel};


#[derive(Deserialize,Serialize)]
struct OpenedSessionInfo {
    session: SessionModel,
    character: Vec<GameCharacterPlayerShort>,
}

#[derive(Debug,sqlx::FromRow)]
struct PlayerSessionInfoRow {
    session_id: i64,
    session_title: Option<String>,
    session_last_date: Option<String>,
    session_start_date: Option<String>,
    session_data: Option<serde_json::Value>,
    player_id: i64,
    character_id: i64,
    character_name: Option<String>,
    character_level: Option<i64>,
    specie_name: Option<String>,
    profession_name: Option<String>,
    character_session_stats: Option<String>,
}

#[derive(Deserialize,Serialize)]
pub struct PlayerSessionsInfo {
    sessions: Vec<OpenedSessionInfo>,
}

pub async fn get_player_sessions(pool: &SqlitePool, id: i64) -> Result<Option<PlayerSessionsInfo>, sqlx::Error> {
    let rows = sqlx::query_as::<_, PlayerSessionInfoRow>(
        r#"
            SELECT
                s.id AS session_id,
                s.description AS session_title,
                s.last_date AS session_last_date,
                s.date_start AS session_start_date,
                s.session_data ,
                p.id AS player_id,
                gc.id AS character_id,
                gc.name AS character_name,
                gc.level AS character_level,
                cs.name AS specie_name,
                cp.name AS profession_name,
                gcs.session_stats AS character_session_stats
            FROM
                player p
            JOIN game_character gc ON gc.player_id = p.id
            JOIN game_character_session gcs ON gcs.game_character_id = gc.id
            JOIN game_session s ON s.id = gcs.game_session_id
            JOIN character_specie cs ON cs.id = gc.specie_id
            JOIN character_profession cp ON cp.id = gc.profession_id
            WHERE s.completed = 0 AND p.id=?
            ORDER BY s.id, p.id
        "#
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let mut sessions_map: HashMap<i64, OpenedSessionInfo> = HashMap::new();

    for row in rows {
        let character = GameCharacterPlayerShort {
            id: row.character_id,
            specie: row.specie_name.unwrap(),
            profession: row.profession_name.unwrap(),
            name: row.character_name.unwrap(),
            level: row.character_level.unwrap().try_into().unwrap(),
        };

        sessions_map
            .entry(row.session_id)
            .and_modify(|entry| entry.character.push(character.clone()))
            .or_insert_with(|| OpenedSessionInfo { 
                session: SessionModel { 
                    id: row.session_id,
                    description: row.session_title.unwrap_or_default(),
                    completed: 0,
                    date_start: row.session_start_date.unwrap_or_default(),
                    last_date: Some(row.session_last_date.unwrap_or_default()), 
                    session_data: row.session_data.unwrap_or(serde_json::json!({})), 
                }, character: vec![character],
            });
    }

    Ok(Some(PlayerSessionsInfo {
        sessions: sessions_map.into_values().collect(),
    }))
}

pub async fn create_player(pool: &SqlitePool, player: PlayerModel) -> Result<i64, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO player (username, name, color, roles, password)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id
        "#,
        player.username,
        player.name,
        player.color,
        player.roles ,
        player.password,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn get_player_login(pool: &SqlitePool, username: &String) -> Result<Option<PlayerModel>, sqlx::Error> {
    let player = sqlx::query_as::<_, PlayerModel>(
        r#"SELECT id, username, name, color, roles, password FROM player WHERE username = ?"#
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    Ok(player)
}

pub async fn get_all_players(pool: &SqlitePool) -> Result<Vec<PlayerData>, sqlx::Error> {
    let rows = sqlx::query_as!(
        PlayerData,
        r#"
        SELECT id, username, name, color, roles
        FROM player
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_player_by_id(pool: &SqlitePool, id: i64) -> Result<Option<PlayerData>, sqlx::Error> {
    let player = sqlx::query_as::<_, PlayerData>(
        r#"SELECT id, username, name, color, roles FROM player WHERE id = ?"#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(player)
}

pub async fn update_player(pool: &SqlitePool, id: i64, player: PlayerData) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE player
         SET name = ?, color = ?
         WHERE id = ?"
    )
    .bind(&player.name)
    .bind(player.color)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_player_password(pool: &SqlitePool, id: i64, player: PlayerModel) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE player
         SET password = ?, roles = ?
         WHERE id = ?"
    )
    .bind(&player.password)
    .bind(player.roles)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_player(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM player WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
