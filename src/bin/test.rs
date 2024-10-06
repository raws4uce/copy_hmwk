use anyhow::Result;
//use encryption::{decrypt_file, encrypt_file};

use dbms::table::{Table, Variable};
fn main() -> Result<()> {
    let schema = vec![
        Variable::NU("favnum".to_string()),
        Variable::VC("name".to_string()),
        Variable::TF("is_true".to_string()),
    ];

    let mut table = Table::new("users", schema)?;

    table.insert(vec![
        "1".to_string(),
        "Alice".to_string(),
        "true".to_string(),
    ])?;

    table.insert(vec![
        "2".to_string(),
        "Alice".to_string(),
        "true".to_string(),
    ])?;

    table.insert(vec![
        "1".to_string(),
        "Alice".to_string(),
        "true".to_string(),
    ])?;
    //table.insert(vec![
    //    "true".to_string(),
    //    "25".to_string(),
    //    "Alice".to_string(),
    //])?; -- should and does fail but panics so.. need to change that.

    table.save_csv()?;
    table.save_schema()?;

    //let key = encryption::generate_key();
    let csv_path = format!("data/{}/{}.csv", "users", "table");

    // encrypt_file(&csv_path, &key)?;

    // decrypt_file(&csv_path, &key)?;

    //table.patch(&"1".to_string(),r#"{"name":"change"}"#)?;

    if let Some(row) = table.select(&"0".to_string()) {
        println!("Found row: {:?}", row);
    }

    Ok(())
}
