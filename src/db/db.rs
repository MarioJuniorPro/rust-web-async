use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Database {
    pub name: String,
    pub version: i32,
    pub tables: HashMap<String, Table>,

    // pub get_table(name: String) -> Option<&Table> {
    //     self.tables.get(name);
    // }
    
}

pub trait Db {
    fn new(name: String, version: i32) -> Self;
    fn get_database(&self) -> Option<String>;
}

impl Db for Database {

    fn get_database(&self) -> Option<String> {
        Some(self.name.clone())
    }

    fn new(name: String, version: i32) -> Self {
        Database {
            name,
            version,
            tables: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Table {
    pub name: String,
    // pub columns: Vec<Column>,
}

// struct Column {
//     pub name: String,
//     pub data_type: String,
// }
