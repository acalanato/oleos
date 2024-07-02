const BASE: &str = "users.db";

pub struct Item {
    pub name: String,
    pub stock: i64,
}

pub mod sqlite {
    use super::{Item, BASE};
    pub fn create(item: Item) {
	let connection = sqlite::open(BASE).unwrap();
	let query = format!("
CREATE TABLE IF NOT EXISTS users (name TEXT, age INTEGER);
INSERT INTO users VALUES ('{0}', {1});
",item.name, item.stock);
	connection.execute(query).unwrap();
        println!("DB created!");

    }
    pub fn read(column: &str) {
        use sqlite::State;
	let connection = sqlite::open(BASE).unwrap();        
        let query = format!("SELECT * FROM users WHERE {} > ?", column);
        let mut statement = connection.prepare(query).unwrap();
        statement.bind((1, 50)).unwrap();

        statement.read(column)
            .ok().
            expect(" ");
//        while let Ok(State::Row) = statement.next() {
//            println!("name = {}", statement.read::<String, _>("name").unwrap());
//            println!("stock = {}", statement.read::<i64, _>("stock").unwrap());
//        }
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
