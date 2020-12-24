extern crate toml;
extern crate structopt;
use structopt::StructOpt;
use std::path::{PathBuf};

extern crate serde_derive;
extern crate serde_json;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
    unit: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Path file
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    file: PathBuf,

    /// Path output file
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: PathBuf,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Input {
    xml_file : String,
    json_file: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Redis {
    host: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Sqlite {
    db_file: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}
#[allow(unused)]
#[derive(Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql,
}

fn main() -> Result<(), std::io::Error> {
    let path = Opt::from_args();
    let config_const_values: Config = {
        let config_text = std::fs::read_to_string(&path.file).unwrap();
        // config_text.parse::<toml::Value>().unwrap()
        toml::from_str(&config_text).unwrap()
    };

    // Dynamic parse -> Like python, javascript -> long statement
    /*println!("Original: {:#?}", config_const_values);
    println!("[postgresql].database: {}",
             config_const_values
                 .get("postgresql").unwrap()
                 .get("database").unwrap()
                 .as_str().unwrap());*/

    // Static parse -> Rust, C++ -> short statement
    // println!("[postgresql].database: {}", config_const_values.postgresql.database);

    let mut sales_and_products = {
        let sales_and_products_text =
            std::fs::read_to_string(&config_const_values.input.json_file)?;
        serde_json::from_str::<SalesAndProducts>(&sales_and_products_text)
            .unwrap()
    };

    sales_and_products.sales[1].quantity += 1.5;

    std::fs::write(&path.output,
                   serde_json::to_string_pretty(&sales_and_products)
                       .unwrap())?;

    Ok(())
}
