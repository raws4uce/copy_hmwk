use std::sync::{Arc, Mutex};

struct SimpleDB {
	tables: Arc<Mutex<Table>>,
}

impl SimpleDB {
	fn new() -> SimpleDB {
		SimpleDB {
			tables: Arc::new(Mutex::new(Table::new())),
		}
	}

	fn transaction<F>(&self, operations: F)
	where
		F: FnOnce(&mut Table),
	{
		let mut table = self.tables.lock().unwrap();
		operations(&mut table);
	}
}