#[cfg(test)]
mod tests;

use anyhow::Result;
pub mod deserializer;
mod jsonish;
use std::collections::HashMap;

use baml_types::{BamlValue, BamlValueWithMeta, FieldType, ResponseCheck, JinjaExpression};
use deserializer::coercer::{ParsingContext, TypeCoercer};

pub use deserializer::types::BamlValueWithFlags;
use internal_baml_core::ir::TypeValue;
use internal_baml_jinja::types::OutputFormatContent;

use baml_types::CompletionState;
use deserializer::deserialize_flags::Flag;
use serde::{Serialize, Serializer, ser::SerializeMap};
use crate::deserializer::score::WithScore;
use jsonish::Value;

#[derive(Clone, Debug)]
pub struct ResponseBamlValue(pub BamlValueWithMeta<(Vec<Flag>, Vec<ResponseCheck>, Option<CompletionState>)>);

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
