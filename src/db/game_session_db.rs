use sqlx::SqlitePool;
use crate::models::game_session_model::GameSession;
use crate::models::game_session_model::NewGameSession;

pub async fn create_session(pool: &SqlitePool, session: NewGameSession) -> Result<i64, sqlx::Error> {
    let json_string = session.session_data.to_string();

    let rec = sqlx::query!(
        r#"
        INSERT INTO game_session (description, completed, date_start, last_date, session_data)
        VALUES (?, 'f', ?, ?, ?)
        RETURNING id
        "#,
        session.description,
        session.date_start,
        session.last_date,
        json_string,
    )
    .fetch_one(pool)
    .await?;

    Ok(rec.id)
}

pub async fn get_all_sessions(_pool: &SqlitePool) -> Result<Vec<GameSession>, sqlx::Error> {
    // TODO: implement DB fetch all logic
    Ok(vec![])
}

pub async fn get_session_by_id(_pool: &SqlitePool, _id: i64) -> Result<Option<GameSession>, sqlx::Error> {
    // TODO: implement DB fetch by id logic
    Ok(None)
}

pub async fn update_session(_pool: &SqlitePool, _id: i64, _session: GameSession) -> Result<(), sqlx::Error> {
    // TODO: implement DB update logic
    Ok(())
}

pub async fn delete_session(_pool: &SqlitePool, _id: i64) -> Result<(), sqlx::Error> {
    // TODO: implement DB delete logic
    Ok(())
}