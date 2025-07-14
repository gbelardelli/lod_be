#[cfg(test)]
mod tests {
    use crate::db::session_db::{create_session, delete_session, get_all_sessions, get_session_by_id, update_session};
    use crate::models::session_model::NewSessionData;

    use sqlx::{SqlitePool, Executor};

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        pool.execute("
            CREATE TABLE game_session (
                id INTEGER PRIMARY KEY,
                description VARCHAR NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                date_start VARCHAR NOT NULL,
                last_date VARCHAR,
                session_data TEXT NOT NULL
            );
        ").await.unwrap();
        pool
    }

    #[actix_rt::test]
    async fn test_create_and_get() {
        let pool = setup_db().await;
        let new = NewSessionData {
            description: "T1".into(),
            date_start: "2025-01-01".into(),
            last_date: None,
            session_data: serde_json::json!({"k":"v"}),
        };
        let id = create_session(&pool, new).await.unwrap();
        let fetched = get_session_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.description, "T1");
    }

    #[actix_rt::test]
    async fn test_update() {
        let pool = setup_db().await;
        let new = NewSessionData {
            description: "T1".into(),
            date_start: "2025-01-01".into(),
            last_date: None,
            session_data: serde_json::json!({"k":"v"}),
        };
        let id = create_session(&pool, new).await.unwrap();
        let fetched = get_session_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.description, "T1");

        let mut session=fetched;
        session.description="T2".into();
        let res = update_session(&pool, id, session).await.unwrap();
        assert_eq!(res,());
        let fetched = get_session_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(fetched.description, "T2");
    }

    #[actix_rt::test]
    async fn test_delete() {
        let pool = setup_db().await;
        let new = NewSessionData {
            description: "T1".into(),
            date_start: "2025-01-01".into(),
            last_date: None,
            session_data: serde_json::json!({"k":"v"}),
        };
        let id = create_session(&pool, new).await.unwrap();
        let fetched = get_session_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.description, "T1");

        let res = delete_session(&pool, id).await.unwrap();
        assert_eq!(res,());
        let fetched = get_session_by_id(&pool, id).await.unwrap();
        assert!(fetched.is_none());
    }    

}
