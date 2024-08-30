use std::fs::{self, File};
use std::io::{Write, BufWriter};
use std::path::Path;

fn write_to_db_file<P: AsRef<Path>>(path: P, data: &[u8]) -> std::io::Result<()> {
	let file = File::create(path)?;

	let mut writer = BufWriter::new(file);

	writer.write_all(data)?;

	writer.flush()?;

	Ok(())
}

fn main() -> std::io::Result<()> {
	let data_dir = "/Users/user/go/github.com/dbms/data";
	let data_file = format!("{}/datafile.db", data_dir);

	if !Path::new(data_dir).exists() {
		fs::create_dir_all(data_dir)?;
	}

	let data = b"Example database content";

	write_to_db_file(&data_file, data)?;

	println!("Data written to {}", data_file);

	Ok(())
}
