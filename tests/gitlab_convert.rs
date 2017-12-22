extern crate genetype;

use genetype::Converter;

#[test]
fn gitlab_push_json_convert() {
    const PATH: &str = "tests/resources/gitlab_push.json";
    let converted = Converter::new(PATH).convert().expect("Failed to convert");
    let expected = include_str!("resources/push_hook.rs");
    println!("{}", converted);
    assert_eq!(converted, expected);
}