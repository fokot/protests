use sqlx::{Error, PgPool};
use crate::model::{Protest, ProtestSave, ProtestSearch, Region};


pub async fn list_protests(db: &PgPool, search: ProtestSearch) -> Result<Vec<Protest>, Error> {

    let mut where_clauses = vec!["p.deleted IS NULL".to_string()];

    if let Some(town) = &search.town {
        where_clauses.push(format!("(r.id = '{}' OR r2.id = '{}')", town, town));
    }

    if let Some(date_from) = &search.date_from {
        where_clauses.push(format!("p.protest_date >= '{}'", date_from));
    }

    if let Some(tags) = &search.tags {
        let tags_list = tags.iter().map(|tag| format!("'{}'", tag)).collect::<Vec<_>>().join(", ");
        where_clauses.push(format!("t.name IN ({})", tags_list));
    }

    if let Some(created_by) = &search.created_by {
        where_clauses.push(format!("u.name = '{}'", created_by));
    }

    let where_expression = where_clauses.join(" AND ");

    let query = format!(
    r#"SELECT
            p.id,
            p.title,
            p.description,
            ARRAY_AGG(t.name) as tags,
            r.name AS town,
            r2.name AS region,
            p.protest_date AS date,
            p.protest_time AS time,
            p.location,
            p.user_id,
            COALESCE(u.name, u.email) as user_name,
            i.name as image_name
        FROM
            protest p
                JOIN
            users u ON p.user_id = u.id
                LEFT JOIN
            region r ON p.region_id = r.id
                LEFT JOIN
            region r2 ON r.parent_id = r2.id
                JOIN
            protest_tag pt ON p.id = pt.protest_id
                LEFT JOIN
            tag t ON pt.tag_id = t.id
                LEFT JOIN
            image i ON p.image_id = i.id
        WHERE {}
        GROUP BY
            p.id, r.name, r2.name, user_name, image_name"#,
        where_expression
    );

    sqlx::query_as::<_, Protest>(&query).fetch_all(db).await
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
            p.location,
            p.user_id,
            COALESCE(u.name, u.email) as user_name,
            i.name as image_name
        FROM
            protest p
                JOIN
            users u ON p.user_id = u.id
                LEFT JOIN
            region r ON p.region_id = r.id
                LEFT JOIN
            region r2 ON r.parent_id = r2.id
                JOIN
            protest_tag pt ON p.id = pt.protest_id
                LEFT JOIN
            tag t ON pt.tag_id = t.id
                LEFT JOIN
            image i ON p.image_id = i.id
        WHERE p.id = $1 AND p.deleted IS NULL
        GROUP BY
            p.id, r.name, r2.name, user_name, image_name"#
    ).bind(id).fetch_one(db).await
}

pub async fn create_protest(db: &PgPool, protest: &ProtestSave, user_id: i32) -> Result<(), Error> {
    let mut tx = db.begin().await?;

    //FIXME
    let region_id: i32 = 1;

    let protest_id: (i32,) = sqlx::query_as(
        r#"INSERT INTO protest (title, description, protest_date, protest_time, location, user_id, region_id, image_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id"#
    )
        .bind(&protest.title)
        .bind(&protest.description)
        .bind(protest.date)
        .bind(protest.time)
        .bind(&protest.location)
        .bind(user_id)
        .bind(region_id)
        .bind(protest.image_id)
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
        .bind(protest.date)
        .bind(protest.time)
        .bind(&protest.location)
        //FIXME
        .bind(1)
        .bind(protest.id)
        .execute(&mut * tx).await?;

    sqlx::query(
        r#"
        DELETE FROM protest_tag WHERE protest_id = $1
        "#
    ).bind(protest.id)
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
    ).bind(protest.id)
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

pub async fn save_login_code(db: &PgPool, email: &str, login_code: &str) -> Result<i32, Error> {
    sqlx::query_scalar(
        r#"INSERT INTO users (email, login_code, login_code_created)
            VALUES ($1, $2, NOW())
            ON CONFLICT (email)
            DO UPDATE SET
                login_code = EXCLUDED.login_code,
                login_code_created = EXCLUDED.login_code_created
            RETURNING id;"#
    )
        .bind(email)
        .bind(login_code)
        .fetch_one(db).await
}

pub async fn check_login_code(db: &PgPool, user_id: i32, login_code: &str, expiration_days: i32) -> Result<Option<i32>, Error> {
    sqlx::query_scalar(
        r#"SELECT id FROM users WHERE id = $1 AND login_code = $2 AND NOW() < (login_code_created + INTERVAL '$3 days')"#
    )
        .bind(user_id)
        .bind(login_code)
        .bind(expiration_days)
        .fetch_optional(db).await
}

pub async fn save_image(db: &PgPool, image_name: &str, user_id: i32) -> Result<i32, Error> {
    sqlx::query_scalar(
        r#"INSERT INTO image (name, user_id) VALUES ($1, $2) RETURNING id"#
    )
        .bind(image_name)
        .bind(user_id)
        .fetch_one(db).await
}

pub async fn list_regions(db: &PgPool) -> Result<Vec<Region>, Error> {
    sqlx::query_as("SELECT id, name, parent_id FROM region").fetch_all(db).await
}