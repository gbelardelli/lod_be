use sqlx::SqlitePool;

use crate::models::info_model::{InfoModel, ProfessionModel, SpecieModel, WeaponModel};


pub async fn get_all_info(pool: &SqlitePool) -> Result<InfoModel, sqlx::Error> {
    let species = sqlx::query_as!(
        SpecieModel,
        r#"
        SELECT id, name, description, notes, start_stats 
        FROM character_specie
        "#
    )
    .fetch_all(pool)
    .await?;

    let professions = sqlx::query_as!(
        ProfessionModel,
        r#"
        SELECT id, name, description, start_gear, notes, start_stats 
        FROM character_profession
        "#
    )
    .fetch_all(pool)
    .await?;

    let weapons = sqlx::query_as!(
        WeaponModel,
        r#"
        SELECT id, name, damage, encumbrance, class, special,
            cost, availability, reload 
        FROM weapon
        "#
    )
    .fetch_all(pool)
    .await?;

    let info = InfoModel {
        species,
        weapons,
        professions
    };

    Ok(info)
}
