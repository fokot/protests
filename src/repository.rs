use sqlx::{Error, PgPool};
use crate::model::{Protest, ProtestSave};


pub async fn list_protests(db: &PgPool) -> Result<Vec<Protest>, Error> {
    sqlx::query_as::<_, Protest>(
    r#"SELECT
            p.id,
            p.title,
            p.description,
            ARRAY_AGG(t.name) as tags,
            r.name AS town,
            r2.name AS region,
            p.protest_date AS date,
            p.protest_time AS time,
            p.location
        FROM
            protest p
                LEFT JOIN
            region r ON p.region_id = r.id
                LEFT JOIN
            region r2 ON r.parent_id = r2.id
                JOIN
            protest_tag pt ON p.id = pt.protest_id
                LEFT JOIN
            tag t ON pt.tag_id = t.id
        WHERE p.deleted IS NULL
        GROUP BY
            p.id, r.name, r2.name"#

    ).fetch_all(db).await
}

pub async fn get_protest(db: &PgPool, id: i32) -> Result<Protest, Error> {
    sqlx::query_as::<_, Protest>(
        r#"SELECT
            p.id,
            p.title,
            p.description,
            ARRAY_AGG(t.name) as tags,
            r.name AS town,
            r2.name AS region,
            p.protest_date AS date,
            p.protest_time AS time,
            p.location
        FROM
            protest p
                LEFT JOIN
            region r ON p.region_id = r.id
                LEFT JOIN
            region r2 ON r.parent_id = r2.id
                JOIN
            protest_tag pt ON p.id = pt.protest_id
                LEFT JOIN
            tag t ON pt.tag_id = t.id
        WHERE p.id = $1
        GROUP BY
            p.id, r.name, r2.name"#
    ).bind(id).fetch_one(db).await
}

pub async fn create_protest(db: &PgPool, protest: &ProtestSave) -> Result<(), Error> {
    let mut tx = db.begin().await?;

    //FIXME
    let user_id: i32 = 1;
    let region_id: i32 = 1;

    let protest_id: (i32,) = sqlx::query_as(
        r#"INSERT INTO protest (title, description, protest_date, protest_time, location, user_id, region_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id"#
    )
        .bind(&protest.title)
        .bind(&protest.description)
        .bind(&protest.date)
        .bind(&protest.time)
        .bind(&protest.location)
        .bind(user_id)
        .bind(region_id)
        .fetch_one(&mut *tx).await?;

    sqlx::query(
        r#"
        INSERT INTO tag (name)
        SELECT * FROM UNNEST(ARRAY[$1])
        ON CONFLICT (name) DO NOTHING
        "#
    ).bind(&protest.tags)
    .execute(&mut *tx).await?;

    sqlx::query(
        r#"
        INSERT INTO protest_tag (protest_id, tag_id)
        SELECT $1, id FROM tag WHERE name = ANY($2)
        "#
    ).bind(protest_id.0)
    .bind(&protest.tags)
    .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(())
}

pub async fn edit_protest(db: &PgPool, protest: &Protest) -> Result<(), Error> {
    let mut tx = db.begin().await?;

    sqlx::query(
        r#"UPDATE protest SET
            title = $1,
            description = $2,
            protest_date = $3,
            protest_time = $4,
            location = $5,
            region_id = $6,
            updated = NOW()
            WHERE id = $7"#
    )
        .bind(&protest.title)
        .bind(&protest.description)
        .bind(&protest.date)
        .bind(&protest.time)
        .bind(&protest.location)
        //FIXME
        .bind(1)
        .bind(protest.id)
        .execute(&mut * tx).await?;

    sqlx::query(
        r#"
        DELETE FROM protest_tag WHERE protest_id = $1
        "#
    ).bind(&protest.id)
        .execute(&mut *tx).await?;

    sqlx::query(
        r#"
        INSERT INTO tag (name)
        SELECT * FROM UNNEST(ARRAY[$1])
        ON CONFLICT (name) DO NOTHING
        "#
    ).bind(&protest.tags)
        .execute(&mut *tx).await?;

    sqlx::query(
        r#"
        INSERT INTO protest_tag (protest_id, tag_id)
        SELECT $1, id FROM tag WHERE name = ANY($2)
        "#
    ).bind(&protest.id)
        .bind(&protest.tags)
        .execute(&mut *tx).await?;

    tx.commit().await?;
    Ok(())
}

pub async fn delete_protest(db: &PgPool, id: i32) -> Result<(), Error> {
    sqlx::query("UPDATE protest SET deleted = NOW() WHERE id = $1")
        .bind(id)
        .execute(db).await?;
    Ok(())
}