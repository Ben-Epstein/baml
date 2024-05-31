use anyhow::Result;
use internal_baml_jinja::types::{Class, Enum, Name, OutputFormatContent};

#[macro_use]
pub mod macros;

mod test_class;
mod test_enum;
mod test_lists;
mod test_unions;

use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
};

use baml_types::BamlValue;
use internal_baml_core::{
    internal_baml_diagnostics::SourceFile,
    ir::{repr::IntermediateRepr, ClassWalker, EnumWalker, FieldType, IRHelper, TypeValue},
    validate,
};
use serde_json::json;

use crate::from_str;

fn load_test_ir(file_content: &str) -> IntermediateRepr {
    let mut schema = validate(
        &PathBuf::from("./baml_src"),
        vec![SourceFile::from((
            PathBuf::from("./baml_src/example.baml"),
            file_content.to_string(),
        ))],
    );
    schema.diagnostics.to_result().unwrap();

    IntermediateRepr::from_parser_database(&schema.db, schema.configuration).unwrap()
}

fn render_output_format(
    ir: &IntermediateRepr,
    output: &FieldType,
    env_values: &HashMap<String, String>,
) -> Result<OutputFormatContent> {
    let (enums, classes) = relevant_data_models(ir, output, env_values)?;
    return Ok(OutputFormatContent::new(enums, classes, output.clone()));
}

fn find_existing_class_field<'a>(
    class_name: &str,
    field_name: &str,
    class_walker: &Result<ClassWalker<'a>>,
    env_values: &HashMap<String, String>,
) -> Result<(Name, FieldType, Option<String>)> {
    let Ok(class_walker) = class_walker else {
        anyhow::bail!("Class {} does not exist", class_name);
    };

    let Some(field_walker) = class_walker.find_field(field_name) else {
        anyhow::bail!("Class {} does not have a field: {}", class_name, field_name);
    };

    let name = Name::new_with_alias(field_name.to_string(), field_walker.alias(env_values)?);
    let desc = field_walker.description(env_values)?;
    let r#type = field_walker.r#type();
    Ok((name, r#type.clone(), desc))
}

fn find_enum_value(
    enum_name: &str,
    value_name: &str,
    enum_walker: &Result<EnumWalker<'_>>,
    env_values: &HashMap<String, String>,
) -> Result<Option<(Name, Option<String>)>> {
    if enum_walker.is_err() {
        anyhow::bail!("Enum {} does not exist", enum_name);
    }

    let value_walker = match enum_walker {
        Ok(e) => e.find_value(value_name),
        Err(_) => None,
    };

    let value_walker = match value_walker {
        Some(v) => v,
        None => return Ok(None),
    };

    if value_walker.skip(env_values)? {
        return Ok(None);
    }

    let name = Name::new_with_alias(value_name.to_string(), value_walker.alias(env_values)?);
    let desc = value_walker.description(env_values)?;

    Ok(Some((name, desc)))
}

fn relevant_data_models<'a>(
    ir: &'a IntermediateRepr,
    output: &'a FieldType,
    env_values: &HashMap<String, String>,
) -> Result<(Vec<Enum>, Vec<Class>)> {
    let mut checked_types = HashSet::new();
    let mut enums = Vec::new();
    let mut classes = Vec::new();
    let mut start: Vec<baml_types::FieldType> = vec![output.clone()];

    while !start.is_empty() {
        let output = start.pop().unwrap();
        match &output {
            FieldType::Enum(enm) => {
                if checked_types.insert(output.to_string()) {
                    let walker = ir.find_enum(enm);

                    let real_values = walker
                        .as_ref()
                        .map(|e| e.walk_values().map(|v| v.name().to_string()))
                        .ok();
                    let values = real_values
                        .into_iter()
                        .flatten()
                        .into_iter()
                        .map(|value| {
                            let meta = find_enum_value(enm, &value, &walker, env_values)?;
                            Ok(meta.map(|m| m))
                        })
                        .filter_map(|v| v.transpose())
                        .collect::<Result<Vec<_>>>()?;

                    enums.push(Enum {
                        name: Name::new_with_alias(enm.to_string(), walker?.alias(env_values)?),
                        values,
                    });
                }
            }
            FieldType::List(inner) | FieldType::Optional(inner) => {
                if !checked_types.contains(&inner.to_string()) {
                    start.push(inner.as_ref().clone());
                }
            }
            FieldType::Map(k, v) => {
                if checked_types.insert(output.to_string()) {
                    if !checked_types.contains(&k.to_string()) {
                        start.push(k.as_ref().clone());
                    }
                    if !checked_types.contains(&v.to_string()) {
                        start.push(v.as_ref().clone());
                    }
                }
            }
            FieldType::Tuple(options) | FieldType::Union(options) => {
                if checked_types.insert((&output).to_string()) {
                    for inner in options {
                        if !checked_types.contains(&inner.to_string()) {
                            start.push(inner.clone());
                        }
                    }
                }
            }
            FieldType::Class(cls) => {
                if checked_types.insert(output.to_string()) {
                    let walker = ir.find_class(&cls);

                    let real_fields = walker
                        .as_ref()
                        .map(|e| e.walk_fields().map(|v| v.name().to_string()))
                        .ok();

                    let fields = real_fields.into_iter().flatten().into_iter().map(|field| {
                        let meta = find_existing_class_field(&cls, &field, &walker, env_values)?;
                        Ok(meta)
                    });

                    let fields = fields.collect::<Result<Vec<_>>>()?;

                    for (_, t, _) in fields.iter().as_ref() {
                        if !checked_types.contains(&t.to_string()) {
                            start.push(t.clone());
                        }
                    }

                    classes.push(Class {
                        name: Name::new_with_alias(cls.to_string(), walker?.alias(env_values)?),
                        fields,
                    });
                }
            }
            FieldType::Primitive(_) => {}
        }
    }

    Ok((enums, classes))
}

const EMPTY_FILE: &str = r#"
"#;

test_deserializer!(
    test_string_from_string,
    EMPTY_FILE,
    r#"hello"#,
    FieldType::Primitive(TypeValue::String),
    "hello"
);

test_deserializer!(
    test_string_from_string_with_quotes,
    EMPTY_FILE,
    r#""hello""#,
    FieldType::Primitive(TypeValue::String),
    "\"hello\""
);

test_deserializer!(
    test_string_from_object,
    EMPTY_FILE,
    r#"{"hi":    "hello"}"#,
    FieldType::Primitive(TypeValue::String),
    r#"{"hi":    "hello"}"#
);

test_deserializer!(
    test_string_from_obj_and_string,
    EMPTY_FILE,
    r#"The output is: {"hello": "world"}"#,
    FieldType::Primitive(TypeValue::String),
    "The output is: {\"hello\": \"world\"}"
);

test_deserializer!(
    test_string_from_list,
    EMPTY_FILE,
    r#"["hello", "world"]"#,
    FieldType::Primitive(TypeValue::String),
    "[\"hello\", \"world\"]"
);

test_deserializer!(
    test_string_from_int,
    EMPTY_FILE,
    r#"1"#,
    FieldType::Primitive(TypeValue::String),
    "1"
);

test_deserializer!(
    test_string_from_string21,
    EMPTY_FILE,
    r#"Some preview text

    JSON Output:
    
    [
      {
        "blah": "blah"
      },
      {
        "blah": "blah"
      },
      {
        "blah": "blah"
      }
    ]"#,
    FieldType::Primitive(TypeValue::String),
    r#"Some preview text

    JSON Output:
    
    [
      {
        "blah": "blah"
      },
      {
        "blah": "blah"
      },
      {
        "blah": "blah"
      }
    ]"#
);

test_deserializer!(
    test_string_from_string22,
    EMPTY_FILE,
    r#"Hello there.
    
    JSON Output:
    ```json
    [
      {
        "id": "hi"
      },
      {
        "id": "hi"
      },
      {
        "id": "hi"
      }
    ]
    ```
    "#,
    FieldType::Primitive(TypeValue::String),
    r#"Hello there.
    
    JSON Output:
    ```json
    [
      {
        "id": "hi"
      },
      {
        "id": "hi"
      },
      {
        "id": "hi"
      }
    ]
    ```
    "#
);

const FOO_FILE: &str = r#"
class Foo {
  id string
}
"#;

// This fails becaus
test_deserializer!(
    test_string_from_string23,
    FOO_FILE,
    r#"Hello there. Here is {{playername}

  JSON Output:

    {
      "id": "{{hi} there"
    }

  "#,
    FieldType::Class("Foo".to_string()),
    json!({"id": r#"{}"# })
);

// also fails -- if you are in an object and you are casting to a string, dont do that.
// TODO: find all the json blobs here correctly
test_deserializer!(
    test_string_from_string24,
    FOO_FILE,
    r#"Hello there. Here is {playername}

    JSON Output:

      {
        "id": "{{hi} there",
      }

    "#,
    FieldType::Class("Foo".to_string()),
    json!({"id": r#"{{hi} there"# })
);