use rust_cli_sqlite::{Record, create_record, read_record, update_record, delete_record};


#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_crud_operations() -> Result<(), rusqlite::Error> {
        // Open an in-memory database connection for testing
        let conn = Connection::open("HateCrimesDB.db")?;

        // Create a record
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
        create_record(&conn, &record)?;

        // Read the record
        let retrieved_record = read_record(&conn, &record.state)?;
        assert_eq!(retrieved_record, record);

        // Update the record
        let new_value = "Female";
        update_record(&conn, &record, "gender", new_value)?;
        let updated_record = read_record(&conn, &record.state)?;
        assert_eq!(updated_record.gender, new_value);

        // Delete the record
        delete_record(&conn, &record)?;
        let result = read_record(&conn, &record.state);
        assert!(result.is_err());

        Ok(())
    }
}