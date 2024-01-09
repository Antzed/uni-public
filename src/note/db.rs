// note/db.rs

use rusqlite::{Connection, Result};

pub fn setup_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_note(conn: &Connection, note: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO notes (content) VALUES (?1)",
        &[note],
    )?;
    Ok(())
}

pub fn query_notes(conn: &Connection) -> Result<Vec<(i32, String, String)>> {
    let mut stmt = conn.prepare("SELECT id, content, created_at FROM notes")?;
    let notes_iter = stmt.query_map([], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    })?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note?);
    }

    Ok(notes)
}
