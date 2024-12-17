#[cfg(test)]
mod tests;

use anyhow::Result;
pub mod deserializer;
mod jsonish;
use std::collections::HashMap;

use baml_types::{BamlValue, BamlValueWithMeta, FieldType, ResponseCheck, JinjaExpression, SerializeMetadata};
use deserializer::{coercer::{ParsingContext, ParsingError, TypeCoercer}, deserialize_flags::DeserializerConditions};

pub use deserializer::types::BamlValueWithFlags;
use internal_baml_core::ir::TypeValue;
use internal_baml_jinja::types::OutputFormatContent;

use baml_types::CompletionState;
use deserializer::deserialize_flags::Flag;
use deserializer::types::ParsingErrorToUiJson;
use serde::{Serialize, Serializer, ser::SerializeMap};
use crate::deserializer::score::WithScore;
use jsonish::Value;

#[derive(Clone, Debug)]
pub struct ResponseBamlValue(pub BamlValueWithMeta<(Vec<Flag>, Vec<ResponseCheck>, Option<CompletionState>)>);

impl serde::Serialize for ResponseBamlValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

pub fn from_str(
    of: &OutputFormatContent,
    target: &FieldType,
    raw_string: &str,
    allow_partials: bool,
) -> Result<BamlValueWithFlags> {
    if matches!(target, FieldType::Primitive(TypeValue::String)) {
        return Ok(BamlValueWithFlags::String(raw_string.to_string().into()));
    }

    // When the schema is just a string, i should really just return the raw_string w/o parsing it.
    let value = jsonish::parse(raw_string, jsonish::ParseOptions::default())?;
    // let schema = deserializer::schema::from_jsonish_value(&value, None);
    eprintln!("value: {value:?}");

    // See Note [Streaming Number Invalidation]
    if allow_partials {
        // invalidate_numbers_in_progress(&mut value, raw_string);
    }

    // Pick the schema that is the most specific.
    // log::info!("Parsed: {}", schema);
    log::debug!("Parsed JSONish (step 1 of parsing): {:#?}", value);
    let ctx = ParsingContext::new(of, allow_partials);
    // let res = schema.cast_to(target);
    // log::info!("Casted: {:?}", res);

    // match res {
    //     Ok(v) => Ok(v),
    //     Err(e) => anyhow::bail!("Failed to cast value: {}", e),
    // }

    // Determine the best way to get the desired schema from the parsed schema.

    // Lets try to now coerce the value into the expected schema.
    let parsed_value: BamlValueWithFlags = match target.coerce(&ctx, target, Some(&value)) {
        Ok(v) => {
            if v.conditions()
                .flags()
                .iter()
                .any(|f| matches!(f, Flag::InferedObject(jsonish::Value::String(_, _))))
            {
                anyhow::bail!("Failed to coerce value: {:?}", v.conditions().flags());
            }

            Ok::<BamlValueWithFlags, anyhow::Error>(v)
        }
        Err(e) => anyhow::bail!("Failed to coerce value: {}", e),
    }?;
    
    Ok(parsed_value)

}


impl ResponseBamlValue { 
    pub fn score(&self) -> i32 {
        self.0.iter().map(|node| node.meta().0.score()).sum()
    }

    pub fn explanation_json(&self) -> Vec<serde_json::Value> {
        let mut expl = vec![];
        self.explanation_impl(vec!["<root>".to_string()], &mut expl);
        expl.into_iter().map(|e| e.to_ui_json()).collect::<Vec<_>>()
    }

    fn explanation_impl(&self, scope: Vec<String>, expls: &mut Vec<ParsingError>) {
        self.0.iter().for_each(|node| {
            let message = match node {
                BamlValueWithMeta::String(_,_) => "error while parsing string".to_string(),
                BamlValueWithMeta::Int(_,_) => "error while parsing int".to_string(),
                BamlValueWithMeta::Float(_,_) => "error while parsing float".to_string(),
                BamlValueWithMeta::Bool(_, _) => "error while parsing bool".to_string(),
                BamlValueWithMeta::List(_, _) => "error while parsing list".to_string(),
                BamlValueWithMeta::Map(_, _) => "error while parsing map".to_string(),
                BamlValueWithMeta::Enum(enum_name, _, _) => format!("error while parsing {enum_name} enum value"),
                BamlValueWithMeta::Class(class_name,_,_) => format!("error while parsing class {class_name}"),
                BamlValueWithMeta::Null(_) => "error while parsing null".to_string(),
                BamlValueWithMeta::Media(_, _) => "error while parsing media".to_string(),
            };
            let parsing_error = ParsingError {
                scope: scope.clone(),
                reason: message,
                causes: DeserializerConditions{flags: node.meta().0.clone()}.explanation(),
            };
            if node.meta().0.len() > 0 {
                expls.push(parsing_error)
            }
        })
    }
}

impl From<ResponseBamlValue> for BamlValue {
    fn from(v: ResponseBamlValue) -> BamlValue {
        v.0.into()
    }
}

impl WithScore for ResponseBamlValue {
    fn score(&self) -> i32 {
        self.0.iter().map(|node| node.meta().0.score()).sum()
    }
}

impl SerializeMetadata for ResponseBamlValue {
    fn metadata_fields(&self) -> Vec<(String, serde_json::Value)> {
        let mut fields = Vec::new();
        let checks: Vec<(&str, &ResponseCheck)> = self.0.meta().1.iter().map(|check| (check.name.as_str(), check)).collect();
        if !checks.is_empty() {
            let checks_json = serde_json::to_value(checks).expect("Serializing checks is safe.");
            fields.push(("checks".to_string(), checks_json));
        }
        let completion_state: Option<&CompletionState> = self.0.meta().2.as_ref();
        if let Some(state) = completion_state {
            let completion_state_json = serde_json::to_value(&state).expect("Serializing completion state is safe.");
            fields.push(("completion_state".to_string(), completion_state_json));
        }
        fields
    }
}
