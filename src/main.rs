use crate::sqlite::{sqlite::{create, list_all, read}, Item};

pub mod sqlite;

fn main() {
    let melaleuca = Item {
	name: String::from("Alecrim"),
	stock: 3,
    };
    create(melaleuca);
    list_all();
    println!("Sucess!");
    let item = Item {
	name: String::from("Melaleuca"),
	stock: 1,
    };
    create(item);
    read();
}
