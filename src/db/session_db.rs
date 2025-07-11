use sqlx::SqlitePool;
use crate::models::session_model::{NewSessionData, SessionModel};

pub async fn create_session(pool: &SqlitePool, session: NewSessionData) -> Result<i64, sqlx::Error> {
    let json_string = session.session_data.to_string();

    let rec = sqlx::query!(
        r#"
        INSERT INTO game_session (description, date_start, last_date, session_data)
        VALUES (?, ?, ?, ?)
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

pub async fn get_all_sessions(pool: &SqlitePool) -> Result<Vec<SessionModel>, sqlx::Error> {
    let rows = sqlx::query_as!(
        SessionModel,
        r#"
        SELECT id, description, completed, date_start, last_date, session_data as "session_data: _"
        FROM game_session
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_session_by_id(pool: &SqlitePool, id: i64) -> Result<Option<SessionModel>, sqlx::Error> {
    let session = sqlx::query_as::<_, SessionModel>(
        r#"SELECT id, description, completed, date_start, last_date, session_data FROM game_session WHERE id = ?"#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

pub async fn update_session(pool: &SqlitePool, id: i64, session: SessionModel) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE game_session
         SET description = ?, completed = ?, date_start = ?, last_date = ?, session_data = ?
         WHERE id = ?"
    )
    .bind(&session.description)
    .bind(session.completed)
    .bind(&session.date_start)
    .bind(&session.last_date)
    .bind(session.session_data.to_string()) // serializza il JSON
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_session(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM game_session WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
