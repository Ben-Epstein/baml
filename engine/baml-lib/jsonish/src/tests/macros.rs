use internal_baml_core::ir::repr::IntermediateRepr;
use baml_types::{BamlValueWithMeta, CompletionState, FieldType, JinjaExpression, ResponseCheck, StreamingBehavior};
use crate::deserializer::semantic_streaming::validate_streaming_state;
use crate::{ResponseBamlValue, BamlValueWithFlags};
use crate::deserializer::deserialize_flags::Flag;
use anyhow::Result;

macro_rules! test_failing_deserializer {
    ($name:ident, $file_content:expr, $raw_string:expr, $target_type:expr) => {
        #[test_log::test]
        fn $name() {
            let ir = load_test_ir($file_content);
            let target = render_output_format(&ir, &$target_type, &Default::default()).unwrap();

            let result = from_str(&target, &$target_type, $raw_string, false);

            assert!(
                result.is_err(),
                "Failed not to parse: {:?}",
                result.unwrap()
            );
        }
    };
}

/// Arguments
///
/// - `name`: The name of the test function to generate.
/// - `file_content`: A BAML schema used for the test.
/// - `raw_string`: An example payload coming from an LLM to parse.
/// - `target_type`: The type to try to parse `raw_string` into.
/// - `json`: The expected JSON encoding that the parser should return.
///
/// Example
///
/// ```rust
/// test_deserializer!(
///     my_test,
///     "schema_content",
///     "raw_payload",
///     MyType,
///     { "expected": "json" }
/// );
/// ```
macro_rules! test_deserializer {
    ($name:ident, $file_content:expr, $raw_string:expr, $target_type:expr, $($json:tt)+) => {
        #[test_log::test]
        fn $name() {
            let ir = load_test_ir($file_content);
            let target = render_output_format(&ir, &$target_type, &Default::default()).unwrap();

            let result = from_str(
                &target,
                &$target_type,
                $raw_string,
                false,
            );

            assert!(result.is_ok(), "Failed to parse: {:?}", result);

            let value = result.unwrap();
            log::trace!("Score: {}", value.score());
            let value: BamlValue = value.into();
            log::info!("{}", value);
            let json_value = json!(value);

            let expected = serde_json::json!($($json)+);

            assert_json_diff::assert_json_eq!(json_value, expected);
        }
    };
}

macro_rules! test_deserializer_with_expected_score {
    ($name:ident, $file_content:expr, $raw_string:expr, $target_type:expr, $target_score:expr) => {
        #[test_log::test]
        fn $name() {
            let ir = load_test_ir($file_content);
            let target = render_output_format(&ir, &$target_type, &Default::default()).unwrap();

            let result = from_str(&target, &$target_type, $raw_string, false);

            assert!(result.is_ok(), "Failed to parse: {:?}", result);

            let value = result.unwrap();
            dbg!(&value);
            log::trace!("Score: {}", value.score());
            assert_eq!(value.score(), $target_score);
        }
    };
}

macro_rules! test_partial_deserializer {
    ($name:ident, $file_content:expr, $raw_string:expr, $target_type:expr, $($json:tt)+) => {
        #[test_log::test]
        fn $name() {
            let ir = load_test_ir($file_content);
            let target = render_output_format(&ir, &$target_type, &Default::default()).unwrap();

            let result = from_str(
                &target,
                &$target_type,
                $raw_string,
                true,
            );

            assert!(result.is_ok(), "Failed to parse: {:?}", result);

            let value = result.unwrap();
            log::trace!("Score: {}", value.score());
            let value: BamlValue = value.into();
            log::info!("{}", value);
            let json_value = json!(value);

            let expected = serde_json::json!($($json)+);

            assert_json_diff::assert_json_eq!(json_value, expected);
        }
    };
}

fn parsed_value_to_response(
    ir: &IntermediateRepr,
    baml_value: &BamlValueWithFlags,
    field_type: &FieldType,
) -> Result<ResponseBamlValue> {
    let meta_flags: BamlValueWithMeta<Vec<Flag>> = baml_value.clone().into();
    let baml_value_with_meta: BamlValueWithMeta<Vec<(String, JinjaExpression, bool)>> =
        baml_value.clone().into();

    let value_with_response_checks: BamlValueWithMeta<Vec<ResponseCheck>> = baml_value_with_meta.map_meta(|cs| {
        cs.iter()
            .map(|(label, expr, result)| {
                let status = (if *result { "succeeded" } else { "failed" }).to_string();
                ResponseCheck {
                    name: label.clone(),
                    expression: expr.0.clone(),
                    status,
                }
            })
            .collect()
    });

    let baml_value_with_streaming =
        validate_streaming_state(ir, &baml_value, field_type).map_err(|s| anyhow::anyhow!("TODO"))?;
    let response_value = meta_flags
        .zip_meta(value_with_response_checks)
        .ok_or(anyhow::anyhow!("TODO"))?
        .zip_meta(baml_value_with_streaming)
        .ok_or(anyhow::anyhow!("TODO"))?
        .map_meta(|((x, y), z)| (x.clone(), y.clone(), z.clone()));
    Ok(ResponseBamlValue(response_value))
}
