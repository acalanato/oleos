<<<<<<< HEAD
use crate::sqlite::{sqlite::{create, list_all}, Item};
=======
use crate::sqlite::{sqlite::{create, read}, Item};
>>>>>>> main

pub mod sqlite;

fn main() {
<<<<<<< HEAD
    let melaleuca = Item {
	name: String::from("Alecrim"),
	stock: 3,
    };
    create(melaleuca);
    list_all();
    println!("Sucess!");
=======
    let item = Item {
	name: String::from("Melaleuca"),
	stock: 1,
    };
    create(item);
    read(String::from("1"));
>>>>>>> main
}
