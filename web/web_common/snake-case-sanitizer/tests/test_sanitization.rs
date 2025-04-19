use snake_case_sanitizer::Sanitizer;

const EXPECTED_CASES: [(&str, &str); 14] = [
    ("snake_case", "snake_case"),
    ("snakeCase", "snake_case"),
    ("snake-case", "snake_case"),
    ("herbivory_(percent)", "herbivory_percent"),
    ("wikidataID", "wikidata_id"),
    ("wikidata.ID", "wikidata_id"),
    ("wikidata'ID", "wikidata_id"),
    ("snake___case", "snake_case"),
    ("temperature_(°C)", "temperature_celsius"),
    ("herbivory(percent)", "herbivory_percent"),
    ("herbivory(percent)_", "herbivory_percent"),
    ("_herbivory(percent)_", "herbivory_percent"),
    ("_herbivory(percent)))))))))", "herbivory_percent"),
    ("Amount_in_$", "amount_in_dollar"),
];

#[test]
fn test_sanitization() {
    let sanitizer = Sanitizer::default()
        .include_defaults()
        .remove_leading_underscores()
        .remove_trailing_underscores();
    for (input, expected) in EXPECTED_CASES.iter() {
        assert_eq!(sanitizer.to_snake_case(input).unwrap(), *expected);
    }
}
