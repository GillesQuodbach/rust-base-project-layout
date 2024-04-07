// #![allow(unused)] // For beginning only.

use crate::prelude::*;
use std::fs::read_dir;
mod error;
mod prelude;
mod utils;

fn main() -> Result<()>{
    println!("Hello, world!");

    // EXEMPLE (we use read_dir to show all files in our project directory)
    // return entry but we want string to store all values
    // so we map all values to str
    for entry in read_dir("./")?.filter_map(|e| e.ok()){
    let entry: String = W(&entry).try_into()?;
        println!("{entry:?}")
    }

    Ok(())
}
