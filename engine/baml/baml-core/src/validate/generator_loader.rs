use crate::{
    ast::WithSpan,
    configuration::{Generator, GeneratorConfigValue, StringFromEnvVar},
    internal_baml_diagnostics::*,
};
use internal_baml_parser_database::ast::{self, Expression, WithDocumentation, WithName};
use std::collections::HashMap;

const LANGUAGE_KEY: &str = "language";
const OUTPUT_KEY: &str = "output";

const FIRST_CLASS_PROPERTIES: &[&str] = &[LANGUAGE_KEY, OUTPUT_KEY];

/// Load and validate Generators defined in an AST.
pub(crate) fn load_generators_from_ast(
    ast_schema: &ast::SchemaAst,
    diagnostics: &mut Diagnostics,
) -> Vec<Generator> {
    let mut generators: Vec<Generator> = Vec::new();

    for gen in ast_schema.generators() {
        if let Some(generator) = lift_generator(gen, diagnostics) {
            generators.push(generator)
        }
    }

    generators
}

fn lift_generator(
    ast_generator: &ast::GeneratorConfig,
    diagnostics: &mut Diagnostics,
) -> Option<Generator> {
    let generator_name = ast_generator.name();
    let args: HashMap<_, &Expression> = ast_generator
        .fields()
        .iter()
        .map(|arg| match &arg.value {
            Some(expr) => Some((arg.name.name.as_str(), expr)),
            None => {
                diagnostics.push_error(DatamodelError::new_config_property_missing_value_error(
                    arg.name.name.as_str(),
                    generator_name,
                    "generator",
                    ast_generator.span(),
                ));

                None
            }
        })
        .collect::<Option<HashMap<_, _>>>()?;

    if let Some(expr) = args.get(LANGUAGE_KEY) {
        if !expr.is_string() {
            diagnostics.push_error(DatamodelError::new_type_mismatch_error(
                "String",
                expr.describe_value_type(),
                &expr.to_string(),
                expr.span(),
            ))
        }
    }

    let language = match args.get(LANGUAGE_KEY) {
        Some(val) => StringFromEnvVar::coerce(val, diagnostics)?,
        None => {
            diagnostics.push_error(DatamodelError::new_generator_argument_not_found_error(
                LANGUAGE_KEY,
                &ast_generator.name(),
                ast_generator.span(),
            ));
            return None;
        }
    };

    let output = args
        .get(OUTPUT_KEY)
        .and_then(|v| StringFromEnvVar::coerce(v, diagnostics));

    let mut properties = HashMap::new();

    for prop in ast_generator.fields() {
        let is_first_class_prop = FIRST_CLASS_PROPERTIES.iter().any(|k| *k == prop.name());
        if is_first_class_prop {
            continue;
        }

        let value = match &prop.value {
            Some(val) => GeneratorConfigValue::from(val),
            None => {
                diagnostics.push_error(DatamodelError::new_config_property_missing_value_error(
                    &prop.name.name,
                    generator_name,
                    "generator",
                    prop.span,
                ));
                continue;
            }
        };

        properties.insert(prop.name.name.clone(), value);
    }

    Some(Generator {
        name: String::from(ast_generator.name()),
        language,
        output,
        config: properties,
        documentation: ast_generator.documentation().map(String::from),
    })
}
