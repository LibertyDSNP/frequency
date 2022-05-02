use crate::avro;

const PRIMITIVE_EXAMPLE: &[(&str, bool)] = &[
    (r#""null""#, true),
    (r#"{"type": "null"}"#, true),
    (r#""boolean""#, true),
    (r#"{"type": "boolean"}"#, true),
    (r#""string""#, true),
    (r#"{"type": "string"}"#, true),
    (r#""bytes""#, true),
    (r#"{"type": "bytes"}"#, true),
    (r#""int""#, true),
    (r#"{"type": "int"}"#, true),
    (r#""long""#, true),
    (r#"{"type": "long"}"#, true),
    (r#""float""#, true),
    (r#"{"type": "float"}"#, true),
    (r#""double""#, true),
    (r#"{"type": "double"}"#, true),
    (r#""true""#, false),
    (r#"true"#, false),
    (r#"{"no_type": "test"}"#, false),
    (r#"{"type": "panther"}"#, false),
];

const VALID_EXAMPLES: &[(&str, bool)] = &[
    (r#"{"type": "fixed", "name": "Test", "size": 1}"#, true),
    (
        r#"{
                "type": "fixed",
                "name": "MyFixed",
                "namespace": "org.apache.hadoop.avro",
                "size": 1
            }"#,
        true,
    ),
];

#[test]
fn test_fingerprint_raw() {
    for (raw_schema, expected) in PRIMITIVE_EXAMPLE {
        let schema_result = avro::fingerprint_raw_schema(raw_schema);
        if *expected {
            assert!(
                schema_result.is_ok(),
                "schema {} was supposed to be valid; error: {:?}",
                raw_schema,
                schema_result.err()
            );
        } else {
            assert!(
                schema_result.is_err(),
                "schema {} was supposed to be invalid; error: {:?}",
                raw_schema,
                schema_result.err()
            );
        }
    }
}

#[test]
/// Test that the string generated by an Avro Schema object is, in fact, a valid Avro schema.
fn test_valid_cast_to_string_after_parse() {
    for (raw_schema, expected) in VALID_EXAMPLES {
        let schema_result = avro::fingerprint_raw_schema(raw_schema);
        if *expected {
            assert!(
                schema_result.is_ok(),
                "schema {} was supposed to be valid; error: {:?}",
                raw_schema,
                schema_result.err()
            );
            let schema_res = schema_result.unwrap();
            let translate_schema = avro::translate_schema(schema_res.1);
            assert!(
                translate_schema.is_ok(),
                "schema {} was supposed to be valid; error: {:?}",
                raw_schema,
                translate_schema.err()
            );
            let translated_schema = translate_schema.unwrap();
            assert_eq!(translated_schema, schema_res.0);
        } else {
            assert!(
                schema_result.is_err(),
                "schema {} was supposed to be invalid; error: {:?}",
                raw_schema,
                schema_result.err()
            );
        }
    }
}
