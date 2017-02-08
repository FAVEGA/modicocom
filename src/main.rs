extern crate csv;
extern crate clap;

use clap::{Arg, App};
use std::collections::HashMap;

const PRICES_CODE_COLUMN: usize = 0;
const PRICES_PRICE_COLUMN: usize = 1;
const PRICES_DATE_COLUMN: usize = 2;

const CONCOM_CODE_COLUMN: usize = 1;
const CONCOM_PRICE_COLUMN: usize = 3;
const CONCOM_DISCOUNT_PERCENT_COLUMN: usize = 4;
const CONCOM_DATE_COLUMN: usize = 5;

fn main() {
    let matches = App::new("modicocom")
        .version("1.0")
        .arg(Arg::with_name("PRICES")
            .help("Sets the input prices file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("CONCOM")
            .help("Sets the input concom file to use")
            .required(true)
            .index(2))
        .get_matches();
    let prices = load_csv(matches.value_of("PRICES").unwrap());
    let mut concom = load_csv(matches.value_of("CONCOM").unwrap());
    let mut price_map = HashMap::new();
    for row in &prices {
        price_map.insert(&row[PRICES_CODE_COLUMN],
                         (&row[PRICES_PRICE_COLUMN], &row[PRICES_DATE_COLUMN]));
    }
    let mut newcc = csv::Writer::from_file("newcc.csv")
        .expect("Error opening newcc output file")
        .delimiter(b';')
        .quote(b'"');
    let mut newt10 = csv::Writer::from_file("newt10.csv")
        .expect("Error opening newt10 output file")
        .delimiter(b';')
        .quote(b'"');
    for (i, row) in concom.iter_mut().enumerate() {
        if let Some(&(ref new_price, ref new_date)) = price_map.get(&row[CONCOM_CODE_COLUMN]) {
            row[CONCOM_PRICE_COLUMN] = new_price.to_string();
            row[CONCOM_DATE_COLUMN] = new_date.to_string();
        }
        newcc.encode(row.clone()).expect("Error writing to newcc");
        let price: f64 = row[CONCOM_PRICE_COLUMN]
            .replace(",", ".")
            .parse()
            .expect(&format!("Error parsing price `{}` on row {} with product code {}.",
                             row[CONCOM_PRICE_COLUMN],
                             i,
                             row[CONCOM_CODE_COLUMN]));
        let discount_percent: f64 = row[CONCOM_DISCOUNT_PERCENT_COLUMN]
            .replace(",", ".")
            .parse()
            .expect(&format!("Error parsing discount percent `{}` on row {} with product code \
                              {}.",
                             row[CONCOM_DISCOUNT_PERCENT_COLUMN],
                             i,
                             row[CONCOM_CODE_COLUMN]));
        newt10.encode((row[CONCOM_CODE_COLUMN].clone(), price * (100f64 - (discount_percent / 100f64))))
            .expect("Error writing to newt10");
    }
}

fn load_csv(file_name: &str) -> Vec<Vec<String>> {
    let f = csv::Reader::from_file(file_name)
        .expect(&format!("Error opening input file {}", file_name))
        .delimiter(b';')
        .records()
        .map(|r| r.unwrap())
        .collect::<Vec<_>>();
    f.iter()
        .map(|row| row.iter().map(|c| c.trim().to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
