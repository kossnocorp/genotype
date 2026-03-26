mod generated_quote {
    include!(concat!(
        env!("GENOTYPE_CLI_MANIFEST_DIR"),
        "/examples/ordered-float-derives/dist/rs/src/quote.rs"
    ));
}

fn main() {
    let _quote = generated_quote::Quote {
        value: ordered_float::OrderedFloat(10.0_f64),
        precision: ordered_float::OrderedFloat(0.5_f32),
        weight: ordered_float::OrderedFloat(1.25_f64),
    };
}
