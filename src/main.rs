mod table;

use std::error::Error;

use crate::table::Table;

fn main() -> Result<(), Box<dyn Error>> {
	let schema = vec![
		"num".to_string(),
		"name".to_string(),
		"age".to_string(),
	];

	let mut table = Table::new("users", schema)?;
	
	table.insert(vec![
		"1".to_string(),
		"Alice".to_string(),
		"25".to_string(),
	])?;

	table.insert(vec![
		"2".to_string(),
		"Bob".to_string(),
		"30".to_string(),
	])?;

	table.save_csv()?;
	table.save_schema()?;

	if let Some(row) = table.select(&"1".to_string()) {
		println!("Found row: {:?}", row);
	}

	Ok(())
}