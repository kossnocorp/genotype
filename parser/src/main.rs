use parser::{GenotypeParser, Rule};
use pest::Parser;
use std::fs;

// #[derive(Parser)]
// #[grammar = "genotype.pest"]
// pub struct GenotypeParser;

pub mod parser;

fn main() {
    let unparsed_file = fs::read_to_string("numbers.csv").expect("cannot read file");

    let file = GenotypeParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut field_sum: f64 = 0.0;
    let mut record_count: u64 = 0;

    for record in file.into_inner() {
        match record.as_rule() {
            Rule::record => {
                record_count += 1;

                for field in record.into_inner() {
                    field_sum += field.as_str().parse::<f64>().unwrap();
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("Sum of fields: {}", field_sum);
    println!("Number of records: {}", record_count);
}
