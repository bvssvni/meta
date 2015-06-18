extern crate piston_meta;

use piston_meta::*;

fn main() {
    let text = "say \"Hello world!\"";
    let rules = "1 \"rule\" [\"say\" w! t?\"foo\"]";
    // Parse rules with meta language and convert to rules for parsing text.
    let rules = bootstrap::convert(
        &parse(&bootstrap::rules(), rules).unwrap(),
        &mut vec![] // stores ignored meta data
    ).unwrap();
    let data = parse(&rules, text).unwrap();
    assert_eq!(data.len(), 1);
    if let &MetaData::String(_, ref hello) = &data[0].1 {
        println!("{}", hello);
    }
}
