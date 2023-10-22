use rusqlite::{Connection, Result};
use rust_cli_sqlite::Record;
use rust_cli_sqlite::{create_record, read_record, update_record, delete_record};

fn main() -> Result<()> {
    let conn = Connection::open("HateCrimesDB.db")?;

    let record = Record {
        state: "North Carolina".to_string(),
        agency_type: "City".to_string(),
        agency_name: "Durham Police Department".to_string(),
        race: "Hispanic".to_string(),
        religion: "Atheist".to_string(),
        sexual_orientation: "Straight".to_string(),
        ethnicity: "Hispanic".to_string(),
        disability: "Physical".to_string(),
        gender: "Male".to_string(),
        gender_identity: "Cisgender".to_string(),
        q1: "Yes".to_string(),
        q2: "No".to_string(),
        q3: "No".to_string(),
        q4: "No".to_string(),
        population: "4000000".to_string(),
    };

    // create
    create_record(&conn, &record)?;
    println!("CREATED RECORD SUCCESSFULLY");
    println!("Record created: {:?}", record);

    // read
    println!("READ RECORD");
    let retrieved_record = read_record(&conn, &record.state)?;
    println!("Read record: {:?}", retrieved_record);

    // update
    println!("UPDATE RECORD");
    update_record(&conn, &retrieved_record, "q1", "No")?;
    let updated_record = read_record(&conn, &record.state)?;
    println!("Updated record: {:?}", updated_record);

    // delete
    println!("DELETE RECORD");
    delete_record(&conn, &updated_record)?;
    let deleted_record = read_record(&conn, &record.state);
    println!("Deleted record: {:?}", deleted_record);

    Ok(())
}