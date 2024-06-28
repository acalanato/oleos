use crate::sqlite::{sqlite::create, Item};

pub mod sqlite;

fn main() {
    let melaleuca = Item {
	name: String::from("Melaleuca"),
	stock: 1,
    };
    create(melaleuca);
    println!("Sucess!");
}
