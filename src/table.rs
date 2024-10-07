use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::result::Result::Ok;
#[derive(Serialize, Deserialize)]
struct TableSchema {
    columns: Vec<Variable>,
}

pub struct Table {
    pub rows: Vec<Vec<String>>,
    pub index: BTreeMap<String, usize>,
    pub schema: TableSchema,
    pub offset: usize,
    path: PathBuf,
}
#[derive(Serialize, Deserialize)]
pub enum Variable {
    VC(String),
    TF(String),
    NU(String),
}
impl Variable {
    pub fn to_string(self) -> String {
        match self {
            Variable::VC(s) => "VC({s})".to_string(),
            Variable::TF(s) => "TF({s})".to_string(),
            Variable::NU(s) => "NU({s})".to_string(),
        }
    }
}

impl Table {
    pub fn new(table_name: &str, schema: Vec<Variable>) -> Result<Table> {
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
            offset: 0,
            path: PathBuf::from(path),
        })
    }
    pub fn load_schema(schema_path: &str) -> Result<TableSchema> {
        let file = File::open(schema_path)?;
        let reader = BufReader::new(file);
        let schema: TableSchema = serde_json::from_reader(reader)?;
        Ok(schema)
    }

    pub fn load_csv(csv_path: &str) -> Result<Vec<Vec<String>>> {
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

    pub fn save_schema(&self) -> Result<()> {
        let schema_file = format!("{}/{}.schema", self.path.display(), "table");
        println!("Saving Schema to: {}", schema_file);
        let file = File::create(schema_file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.schema.columns)?;
        Ok(())
    }

    pub fn save_csv(&mut self) -> Result<()> {
        let csv_file = format!("{}/{}.csv", self.path.display(), "table");
        println!("Saving CSV to: {}", csv_file);
        if Path::new(&csv_file).exists() {
            let mut log = OpenOptions::new()
                .append(true)
                .create(true)
                .open(csv_file) //so if this fails, dir wrong or there is nothing to save.. probably
                .expect("fail to open log file");
            for row in &self.rows {
                writeln!(log, "{}", row.join(","))?;
            }
        } else {
            let file = File::create(csv_file)?;
            let mut writer = BufWriter::new(file);

            for row in &self.rows {
                writeln!(writer, "{}", row.join(","))?;
            }
            writer.flush()?;
        }

        self.rows.clear();
        Ok(())
    }

    pub fn search(&mut self, key: String) -> Result<String> {
        //update map
        self.update_map()?;
        //retieve byte offset
        println!("");
        println!("{:?}", self.index);
        println!("");
        let file = File::open("./data/users/table.csv")?;  //THIS IS A TEMP FIX
        let mut reader = BufReader::new(file);
        let entry = self
            .index
            .get(&key)
            .with_context(|| anyhow!("KEY NOT FOUND BROTHER"))
            .and_then(move |&byt| {
                println!("byte offset for key is: {}", byt);
                reader
                    .seek(SeekFrom::Start(byt as u64))
                    .expect("Failed to seek from byte pos");
                let mut line = String::new();
                reader
                    .read_line(&mut line)
                    .expect("Failed to read line at byte offset");
                if !line.is_empty() {
                    Ok(line.trim().to_string())
                } else {
                    Err(anyhow!("panik"))
                }
            });
        entry
    }
    pub fn insert(&mut self, entry: Vec<String>) -> Result<()> {
        Self::valid_entry(&entry, &self.schema).context("format of entry does not match schema")?;
        //pus to rows
        self.rows.push(entry.clone());
        //insert entry in map
        self.index
            .insert(entry.get(0).unwrap().to_string(), self.offset);
        self.offset += 1 + format!("{entry:?}").len();
        Ok(())
    }
    pub fn delete(&mut self, key: String) -> Result<()> {
        //update map
        self.update_map()?;
        //retieve byte offset
        let file = File::open(self.path.clone())?;
        let mut reader = BufReader::new(file);

        //let entry = self
        //    .index
        //    .get(&key)
        //    .with_context(|| anyhow!("KEY NOT FOUND BROTHER"))
        //    .and_then(|&byt| {
        //
        //
        //    });
        Ok(())
    }

    //map<key, byte_offset>
    fn update_map(&mut self) -> Result<()> {
        let mut map: BTreeMap<String, usize> = BTreeMap::new();
        //update index
        let file = File::open("./data/users/table.csv")?;  //THIS IS A TEMP FIX
        let reader = BufReader::new(file);
        let mut offset_byte = 0;
        for ( i, line ) in reader.lines().enumerate() {
            if let Ok(line) = line {
                let entry: Vec<&str> = line.split(",").collect();
                if let Some(entry) = entry.get(0) {
                    println!("count{i}, {:?}", line);
                    map.insert(entry.to_string(), offset_byte);
                }
                offset_byte += line.len() + 1;
            } else {
                println!(":: :: :: :: this is the bad side :: :: :: :: count{i}, {:?}", line);
            }
        }
        self.offset = offset_byte;
        self.index = map;
        self.save_csv();
        Ok(())
    }

    fn valid_entry(entry: &Vec<String>, schema: &TableSchema) -> Result<()> {
        if entry.len() != schema.columns.len() {
            return Err(anyhow!(
                "entry length {} does not match schema length {}",
                entry.len(),
                schema.columns.len()
            ));
        }
        for (i, column) in schema.columns.iter().enumerate() {
            match column {
                Variable::TF(_) => {
                    entry[i].parse::<bool>().with_context(|| {
                        format!("{}th entry '{}, has failed to parse as bool", i, entry[i])
                    })?;
                }
                Variable::VC(_) => {
                    //erm.. we should be fine
                }
                Variable::NU(_) => {
                    entry[i].parse::<isize>().with_context(|| {
                        format!(
                            "{}th entry '{}, has failed to parse as a number",
                            i, entry[i]
                        )
                    })?;
                }
            }
        }
        Ok(())
    }

    //pub fn append(&mut self, num: &String, data: &str) -> Result<(), Box<dyn Error>> {
    //	let index = self.index.get(num).ok_or("key not found")?;
    //	let row = &mut self.rows[*index];
    //
    //	let update: Value = serde_json::from_str(data)?;
    //
    //	for (key, value) in update.as_object().ok_or("Invalid format")?.iter() {
    //		if let Some(col_index) = self.schema.columns.iter().position(|c| c == key) {
    //			if let Some(new_value) = value.as_str() {
    //				row[col_index] = new_value.to_string();
    //			}
    //		} else {
    //			return Err("Column not found".into());
    //		}
    //	}
    //
    //	self.save_csv()?;
    //	Ok(())
    //}
}
