/* Perform CRUD operations on HateCrimesDB.db database.
*/

// Import sqlite3 crate
use rusqlite::{params, Connection, Result};

// Declare record struct
#[derive(Debug, PartialEq)]
pub struct Record {
    pub state: String,
    pub agency_type: String,
    pub agency_name: String,
    pub race: String,
    pub religion: String,
    pub sexual_orientation: String,
    pub ethnicity: String,
    pub disability: String,
    pub gender: String,
    pub gender_identity: String,
    pub q1: String,
    pub q2: String,
    pub q3: String,
    pub q4: String,
    pub population: String,
}

// CREATE
pub fn create_record(conn: &Connection, record: &Record) -> Result<()> {
    conn.execute(
        "INSERT INTO HateCrimesDB (state, agency_type, agency_name, race, religion, sexual_orientation, ethnicity, disability, gender, gender_identity, q1, q2, q3, q4, population)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            record.state,
            record.agency_type,
            record.agency_name,
            record.race,
            record.religion,
            record.sexual_orientation,
            record.ethnicity,
            record.disability,
            record.gender,
            record.gender_identity,
            record.q1,
            record.q2,
            record.q3,
            record.q4,
            record.population,
        ],
    )?;

    Ok(())
}

// READ
pub fn read_record(conn: &Connection, state: &str) -> Result<Record> {
    conn.query_row(
        "SELECT state, agency_type, agency_name, race, religion, sexual_orientation, ethnicity, disability, gender, gender_identity, q1, q2, q3, q4, population FROM HateCrimesDB WHERE state = ?1",
        params![state],
        |row| {
            Ok(Record {
                state: row.get(0)?,
                agency_type: row.get(1)?,
                agency_name: row.get(2)?,
                race: row.get(3)?,
                religion: row.get(4)?,
                sexual_orientation: row.get(5)?,
                ethnicity: row.get(6)?,
                disability: row.get(7)?,
                gender: row.get(8)?,
                gender_identity: row.get(9)?,
                q1: row.get(10)?,
                q2: row.get(11)?,
                q3: row.get(12)?,
                q4: row.get(13)?,
                population: row.get(14)?,
            })
        },
    ).map_err(|err| match err {
        rusqlite::Error::QueryReturnedNoRows => rusqlite::Error::InvalidQuery,
        _ => err,
    })
}

// UPDATE
pub fn update_record(conn: &Connection, record: &Record, field: &str, new_value: &str) -> Result<()> {
    let mut stmt = conn.prepare(&format!(
        "UPDATE HateCrimesDB SET {} = ?1 WHERE state = ?2",
        field
    ))?;

    stmt.execute(params![new_value, record.state])?;

    Ok(())
}

// DELETE
pub fn delete_record(conn: &Connection, record: &Record) -> Result<()> {
    conn.execute(
        "DELETE FROM HateCrimesDB WHERE state = ?1",
        params![record.state],
    )?;

    Ok(())
}

