use sqlx::{Error, PgPool};
use crate::Protest;

pub async fn list_protests(db: &PgPool) -> Result<Vec<Protest>, Error> {
    sqlx::query_as::<_, Protest>(
        "SELECT id, name, description, labels, town, region, country, date, time, place FROM protests ORDER BY id"
    ).fetch_all(db).await
}

pub async fn get_protest(db: &PgPool, id: i32) -> Result<Protest, Error> {
    sqlx::query_as::<_, Protest>(
        "SELECT id, name, description, labels, town, region, country, date, time, place FROM protests WHERE id = $1"
    ).bind(id).fetch_one(db).await
}

pub async fn create_protest(db: &PgPool, protest: &Protest) -> Result<(), Error> {
    sqlx::query(
        r#"INSERT INTO protests (name, description, labels, town, region, country, date, time, place)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#
    )
        .bind(&protest.name)
        .bind(&protest.description)
        .bind(&protest.labels)
        .bind(&protest.town)
        .bind(&protest.region)
        .bind(&protest.country)
        .bind(&protest.date)
        .bind(&protest.time)
        .bind(&protest.place)
        .execute(db).await?;
    Ok(())
}

pub async fn edit_protest(db: &PgPool, protest: &Protest) -> Result<(), Error> {
    sqlx::query(
        r#"UPDATE protests SET
            name = $1,
            description = $2,
            labels = $3,
            town = $4,
            region = $5,
            country = $6,
            date = $7,
            time = $8,
            place = $9
            WHERE id = $10"#
    )
        .bind(&protest.name)
        .bind(&protest.description)
        .bind(&protest.labels)
        .bind(&protest.town)
        .bind(&protest.region)
        .bind(&protest.country)
        .bind(&protest.date)
        .bind(&protest.time)
        .bind(&protest.place)
        .bind(protest.id)
        .execute(db).await?;
    Ok(())
}

pub async fn delete_protest(db: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query("DELETE FROM protests WHERE id = $1")
        .bind(id)
        .execute(db).await?;
    Ok(())
}