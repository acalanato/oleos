use crate::sqlite::{sqlite::{create, read}, Item};

pub mod sqlite;

fn main() {
    let item = Item {
	name: String::from("Melaleuca"),
	stock: 1,
    };
    create(item);
    read(String::from("users"));
    println!("Sucess!");
}
