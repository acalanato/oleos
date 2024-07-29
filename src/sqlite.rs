pub struct Item {
    pub name: String,
    pub stock: u32,
}

pub mod sqlite {
    use super::Item;
    pub fn create(item: Item) {
	let connection = sqlite::open(":memory:").unwrap();
	let query = format!("
CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('{0}', {1});
",item.name, item.stock);
	connection.execute(query).unwrap()
    }

    pub fn list_all(){
	let connection = sqlite::open(":memory:").unwrap();
	let query = format!("
SELECT name
FROM sqlite_master;
");
	connection.execute(query).unwrap()
    }
    
    pub fn read() {
	let connection = sqlite::open(":memory:").unwrap();

	let query = "
CREATE TABLE users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('Alice', 42);
";
	connection.execute(query).unwrap()
    }
    pub fn update() {
	let connection = sqlite::open(":memory:").unwrap();

	let query = "
CREATE TABLE users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('Alice', 42);
";
	connection.execute(query).unwrap()
    }
    pub fn delete() {
	let connection = sqlite::open(":memory:").unwrap();

	let query = "
CREATE TABLE users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('Alice', 42);
";
	connection.execute(query).unwrap()
    }

}
