use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TableSchema {
    columns: Vec<String>,
}

pub struct Table {
    rows: Vec<Vec<Variable>>,
    index: BTreeMap<String, usize>,
    schema: TableSchema,
    path: PathBuf,
}

//note_to_self, this is cool
macro_rules! init_attribute {
    ($val:expr, bool) => {
        Variable::TF($val)
    };
    ($val:expr, usize) => {
        Variable::NU($val)
    };
    ($val:expr, String) => {
        Variable::VC($val)
    };
}

#[derive(Clone, Debug)]
pub enum Variable {
    TF(bool),
    NU(usize),
    VC(String),
}

impl Variable {
    pub fn name(self) -> String {
        match self {
            Variable::TF(bool) => format!("TF({})", bool),
            Variable::NU(usize) => format!("NU({})", usize),
            Variable::VC(string) => format!("VC(\"{}\")", string),
        }
    }
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
            path: path,
        })
    }

    pub fn load_schema(schema_path: &str) -> Result<TableSchema, Box<dyn Error>> {
        let file = File::open(schema_path)?;
        let reader = BufReader::new(file);
        let schema: TableSchema = serde_json::from_reader(reader)?;
        Ok(schema)
    }

    pub fn load_csv(csv_path: &str) -> Result<Vec<Vec<Variable>>, Box<dyn Error>> {
        let file = File::open(csv_path)?;
        let reader = BufReader::new(file);
        let mut rows = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let row: Vec<Variable> = line
                .split(',')
                .map(|s| Self::parse_variable(s).unwrap())
                .collect();
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
            let line = row
                .iter()
                .map(|v| v.clone().name())
                .collect::<Vec<_>>()
                .join(",");
            println!("{line:?}");
            writeln!(writer, "{}", line)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn insert(&mut self, row: Vec<String>) -> Result<(), Box<dyn Error>> {
        //its gonna sound tedious but ppl are stupid so check that vec alligns with schema
        //Self::check_vec(row);

        //parse as Variable

        //push
        let parsed_vec: Vec<Variable> = row
            .into_iter()
            .map(|s| Self::parse_variable(&s).unwrap())
            .collect::<Vec<Variable>>();

        let key = parsed_vec[0].clone(); // use first row as a key
        self.index.insert(key.clone().name(), self.rows.len());
        self.rows.push(parsed_vec);

        self.save_csv()?;
        Ok(())
    }

    pub fn select(&self, key: &String) -> Option<&Vec<Variable>> {
        self.index.get(key).map(|&i| &self.rows[i])
    }

    pub fn update(&mut self, new_att: &Variable) -> Result<(), Box<dyn Error>> {
    	  //idk how to do this
        Ok(())
    }


    fn parse_variable(s: &str) -> Result<Variable, Box<dyn Error>> {
        //use regex when bothered
        let bool_pattern = Regex::new(r"^TF\((true|false)\)$")?;
        let int_pattern = Regex::new(r"^NU\((\d+)\)$")?;
        let varchar_pattern = Regex::new(r#"^VC\("(.*)"\)$"#)?;

        if let Some(cap) = bool_pattern.captures(s) {
            let val = cap[1].parse::<bool>()?;
            return Ok(Variable::TF(val));
        } else if let Some(cap) = int_pattern.captures(s) {
            let val = cap[1].parse::<usize>()?;
            return Ok(Variable::NU(val));
        } else if let Some(cap) = varchar_pattern.captures(s) {
            let val = cap[1].to_string();
            return Ok(Variable::VC(val));
        }

        Err("pat not found, {s}".into())
    }
}
