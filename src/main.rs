use crate::sqlite::{sqlite::{create, list_all}, Item};

pub mod sqlite;

fn main() {
    let melaleuca = Item {
	name: String::from("Alecrim"),
	stock: 3,
    };
    create(melaleuca);
    list_all();
    println!("Sucess!");
}
