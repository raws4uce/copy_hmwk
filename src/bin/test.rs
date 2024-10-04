use std::error::Error;

use dbms::{encryption, table::{Table, Variable}};
mod table;
#[macro_use]
extern crate strum_macros; // 0.9.0

fn main() -> Result<(), Box<dyn Error>> {
    let schema = vec!["num".to_string(), "name".to_string(), "age".to_string()];

    let mut table = Table::new("users", schema)?;

    table.insert(vec![
        "NU(1)".to_owned(),
        "VC(\"Alice\")".to_owned(),
        "NU(25)".to_owned(),
    ])?;

    table.insert(vec![
        "NU(2)".to_owned(),
        "VC(\"Bizzob\")".to_owned(),
        "NU(18)".to_owned(),
    ])?;

    table.save_csv()?;
    table.save_schema()?;

    let key = encryption::generate_key();
    let csv_path = format!("data/{}/{}.csv", "users", "table");

    // encrypt_file(&csv_path, &key)?;

    // decrypt_file(&csv_path, &key)?;
    let var : Variable = Variable::NU(1);

    table.patch(&"1".to_string(),&var)?;

    if let Some(row) = table.select(&"1".to_string()) {
        println!("Found row: {:?}", row);
    }

    Ok(())
}
