#[cfg(test)]
mod tests {
 
    use sqlx::{SqlitePool, Executor};

    use crate::{db::character_db::{create_character, get_character_by_id}, models::character_model::NewCharacter};

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        pool.execute("
            CREATE TABLE game_character (
                id INTEGER PRIMARY KEY,
                player_id INT NOT NULL,
                specie_id INT NOT NULL,
                profession_id INT NOT NULL,
                name TEXT NOT NULL,
                level INT DEFAULT 1,
                experience INT DEFAULT 0,
                condition TEXT DEFAULT '',
                comment TEXT DEFAULT '',
                stats TEXT DEFAULT '',
                skills TEXT DEFAULT '',
                effects TEXT DEFAULT ''
            );
        ").await.unwrap();
        pool
    }

    #[actix_rt::test]
    async fn test_create_and_get() {
        let pool = setup_db().await;
        let new = NewCharacter {
            name: "C1".into(),
            player_id: 1,
            specie_id: 1,
            profession_id: 1,
        };
        let id = create_character(&pool, new).await.unwrap();
        let fetched = get_character_by_id(&pool, id).await.unwrap().unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.name, "C1");
    }

}