use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableSchema {
	columns: Vec<String>,
}

pub struct Table {
	rows: Vec<Vec<String>>,
	index: BTreeMap<String, usize>,
	schema: TableSchema,
	path: PathBuf,
}


impl Table {
	pub fn new(table_name: &str, schema: Vec<String>) -> Result<Table, Box<dyn Error>> {
		let table_dir = format!("./data/{}", table_name);
		let schema_file = format!("{}/{}.schema", table_dir, table_name);
		let csv_file = format!("{}/{}.csv", table_dir, table_name);
		let path = PathBuf::from(table_dir);

		if !Path::new(&path).exists() {
			fs::create_dir_all(&path)?;
		}

		let schema = if Path::new(&schema_file).exists() {
			Table::load_schema(&schema_file)?
		} else {
			TableSchema { columns: schema }
		};

		let rows = if Path::new(&csv_file).exists() {
			Table::load_csv(&csv_file)?
		} else {
			Vec::new()
		};

		Ok(Table {
			rows,
			index: BTreeMap::new(),
			schema,
			path: PathBuf::from(path),
		})
	}

	pub fn load_schema(schema_path: &str) -> Result<TableSchema, Box<dyn Error>> {
		let file = File::open(schema_path)?;
		let reader = BufReader::new(file);
		let schema: TableSchema = serde_json::from_reader(reader)?;
		Ok(schema)
	}

	pub fn load_csv(csv_path: &str) -> Result <Vec<Vec<String>>, Box<dyn Error>> {
		let file = File::open(csv_path)?;
		let reader = BufReader::new(file);
		let mut rows = Vec::new();

		for line in reader.lines() {
			let line = line?;
			let row: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
			rows.push(row);
		}

		Ok(rows)
	}

	pub fn save_schema(&self) -> Result<(), Box<dyn Error>> {
		let schema_file = format!("{}/{}.schema", self.path.display(), "table");
		println!("Saving Schema to: {}", schema_file);
		let file = File::create(schema_file)?;
		let writer = BufWriter::new(file);
		serde_json::to_writer(writer, &self.schema)?;
		Ok(())
	}

	pub fn save_csv(&self) -> Result<(), Box<dyn Error>> {
		let csv_file = format!("{}/{}.csv", self.path.display(), "table");
		println!("Saving CSV to: {}", csv_file);
		let file = File::create(csv_file)?;
		let mut writer = BufWriter::new(file);

		for row in &self.rows {
			writeln!(writer, "{}", row.join(","))?;
		}

		writer.flush()?;
		Ok(())
	}

	pub fn insert(&mut self, row: Vec<String>) -> Result<(), Box<dyn Error>> {
		let key = row[0].clone(); // use first row as a key
		self.index.insert(key.clone(), self.rows.len());
		self.rows.push(row);

		self.save_csv();
		Ok(())
	}

	pub fn select(&self, key: &String) -> Option<&Vec<String>> {
		self.index.get(key).map(|&i| &self.rows[i])
	}
}

