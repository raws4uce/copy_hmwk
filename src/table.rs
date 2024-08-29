use std::collections::BTreeMap;

struct Table {
	rows: Vec<Vec<String>>,
	index: BTreeMap<String, usize>,
}

impl Table {
	fn new() -> Table {
		Table {
			rows: Vec::new(),
			index: BTreeMap::new(),
		}
	}

	fn insert(&mut self, row: Vec<String>) {
		let key = row[0].clone(); // use first row as a key
		self.index.insert(key.clone(), self.rows.len());
		self.rows.push(row);
	}

	fn select(&self, key: &String) -> Option<&Vec<String>> {
		self.index.get(key).map(|&i| &self.rows[i])
	}
}
