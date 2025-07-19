use sqlx::SqlitePool;
use crate::models::player_model::{PlayerData, PlayerModel};


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
