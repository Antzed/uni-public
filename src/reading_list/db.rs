use rusqlite::{Connection, Result};

pub fn setup_reading_list(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS reading_list (
            id INTEGER PRIMARY KEY,
            url TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'unread',
            added_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            read_at TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

pub fn add_reading_list_item(conn: &Connection, url: &str) -> Result<()> {
    if url_already_exists(conn, url)? {
        return Err(rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error { code: rusqlite::ErrorCode::ConstraintViolation, extended_code: 0 },
            Some("URL already exists in the database".to_string())
        ));
    }

    conn.execute(
        "INSERT INTO reading_list (url) VALUES (?1)",
        &[url],
    )?;
    Ok(())
}

fn url_already_exists(conn: &Connection, url: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM reading_list WHERE url = ?1)")?;
    let exists: bool = stmt.query_row(&[url], |row| row.get(0))?;
    Ok(exists)
}

pub fn list_reading_list(conn: &Connection) -> Result<Vec<(i32, String, String, String, Option<String>)>> {
    let mut stmt = conn.prepare("SELECT id, url, status, added_at, read_at FROM reading_list")?;
    let items_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    })?;

    let mut items = Vec::new();
    for item in items_iter {
        items.push(item?);
    }

    Ok(items)
}

pub fn mark_as_read(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "UPDATE reading_list SET status = 'read', read_at = CURRENT_TIMESTAMP WHERE id = ?1",
        &[&id],
    )?;
    Ok(())
}

pub fn mark_as_unread(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "UPDATE reading_list SET status = 'unread', read_at = NULL WHERE id = ?1",
        &[&id],
    )?;
    Ok(())
}

pub fn delete_reading_list_item(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM reading_list WHERE id = ?1",
        &[&id],
    )?;
    Ok(())
}