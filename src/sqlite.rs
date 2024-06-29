const BASE: &str = "users.db";

pub struct Item {
    pub name: String,
    pub stock: u32,
}

pub mod sqlite {
    use super::{Item, BASE};
    pub fn create(item: Item) {
	let connection = sqlite::open(BASE).unwrap();
	let query = format!("
CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('{0}', {1});
",item.name, item.stock);
	connection.execute(query).unwrap()
    }
    pub fn read(q: String) {
	let connection = sqlite::open(BASE).unwrap();
	let query = format!("
SELECT * FROM {};
", q);
	connection
            .iterate(query, |pairs| {
                for &(name, value) in pairs.iter() {
                    println!("{} - {}", name, value.unwrap());
                }
                true
            })
            .unwrap();
    }
    
    pub fn update() {
	let connection = sqlite::open(BASE).unwrap();

	let query = "
CREATE TABLE users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('Alice', 42);
";
	connection.execute(query).unwrap()
    }
    
    pub fn delete() {
	let connection = sqlite::open(BASE).unwrap();

	let query = "
CREATE TABLE users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('Alice', 42);
";
	connection.execute(query).unwrap()
    }

}
