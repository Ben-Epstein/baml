// This module helps resolve baml values with attached streaming state
// in the context of the streaming behavior associated with their types.

use crate::deserializer::coercer::ParsingError;
use crate::jsonish::CompletionState;
use crate::{BamlValueWithFlags, Flag};
use internal_baml_core::ir::repr::{IntermediateRepr, Walker};
use internal_baml_core::ir::{Field, IRHelper};

use baml_types::{BamlValueWithMeta, FieldType, TypeValue, ResponseCheck, StreamingBehavior};

use std::collections::HashSet;

pub enum StreamingError {
    ExpectedClass,
    IncompleteDoneValue,
}

/// For a given baml value, traverse its nodes, comparing the completion state
/// of each node against the streaming behavior of the node's type.
pub fn validate_streaming_state(
    ir: &IntermediateRepr,
    baml_value: &BamlValueWithFlags,
    field_type: &FieldType,
) -> Result<BamlValueWithMeta<Option<CompletionState>>, StreamingError> {
    let baml_value_with_meta_flags: BamlValueWithMeta<Vec<Flag>> = baml_value.clone().into();
    let typed_baml_value: BamlValueWithMeta<(Vec<Flag>, FieldType)> = ir
        .distribute_type_with_meta(baml_value_with_meta_flags, field_type.clone())
        .unwrap();
    let baml_value_with_streaming_state_and_behavior = typed_baml_value
        .map_meta(|(flags, r#type)| (completion_state(&flags), r#type));

    process_node(ir, baml_value_with_streaming_state_and_behavior)
}

/// Consider a node's type, streaming state, and streaming behavior annotations. Return
/// an error if streaming state doesn't meet the streaming requirements. Also attach
/// the streaming state to the node as metadata, if this was requested by the user
/// vial `@streaming::state`.
/// 
/// This function descends into child nodes, when the argument is a compound value.
fn process_node(
    ir: &IntermediateRepr,
    value: BamlValueWithMeta<(CompletionState, &FieldType)>,
) -> Result<BamlValueWithMeta<Option<CompletionState>>, StreamingError> {
    let (completion_state, field_type) = value.meta();
    let (base_type, (_, streaming_behavior)) = ir.distribute_metadata(field_type);

    if required_done(ir, base_type) && !(completion_state == &CompletionState::Complete) {
        return Err(StreamingError::IncompleteDoneValue)
    }

    let new_meta = if streaming_behavior.state {
        Some(completion_state.clone())
    } else {
        None
    };

    let mut new_value = match value {
        BamlValueWithMeta::String(s,_) => BamlValueWithMeta::String(s, new_meta),
        BamlValueWithMeta::Int(i,_) => BamlValueWithMeta::Int(i, new_meta), 
        BamlValueWithMeta::Float(f,_) => BamlValueWithMeta::Float(f, new_meta),
        BamlValueWithMeta::Bool(b,_) => BamlValueWithMeta::Bool(b, new_meta),
        BamlValueWithMeta::List(items, _) => BamlValueWithMeta::List( 
            items.into_iter().filter_map(|item| process_node(ir, item).ok()).collect(),
            new_meta
        ),
        BamlValueWithMeta::Class(class_name, fields, _) => {
            let needed_fields: HashSet<String> = needed_fields(ir, field_type)?;
            let new_fields = fields.into_iter().filter_map(|(field_name, field_value)| {
                process_node(ir, field_value).ok()
            }).collect::<Vec<_>>();
            todo!()
        }
        _ => todo!()
    };

    // let mut value_meta = new_value.meta_mut();
    // *value_meta = new_meta;
    Ok(new_value)
    
}

/// For a given type, assume that it is a class, and list the fields of that
/// class that were marked `@streaming::needed`.
/// The parameter must have already been passed through `distribute_metadata`,
/// it's an error to call this function with undistributed metadata.
fn needed_fields(ir: &IntermediateRepr, field_type: &FieldType) -> Result<HashSet<String>, StreamingError> {
    match field_type {
        FieldType::Class(class_name) => {
            let class = ir.find_class(class_name).map_err(|_| StreamingError::ExpectedClass)?;
            let needed_fields = class.walk_fields().filter_map(|field: Walker<'_, &Field>| if field.streaming_needed() { Some(field.name().clone())} else { None }).collect();
            Ok(needed_fields)
        },
        _ => Err(StreamingError::ExpectedClass) // TODO: Handle type aliases?.
    }
}

/// Whether a type must be complete before being included as a node
/// in a streamed value.
fn required_done(ir: &IntermediateRepr, field_type: &FieldType) -> bool {
    let (base_type, (_, streaming_behavior)) = ir.distribute_metadata(field_type);
    let type_implies_done = match base_type {
        FieldType::Primitive(tv) => match tv {
            TypeValue::String => false,
            TypeValue::Int => true,
            TypeValue::Float => true,
            TypeValue::Media(_) => true,
            TypeValue::Bool => true,
            TypeValue::Null => true,
        },
        FieldType::Optional(_) => false, // TODO: Think so? Or depends on Optional's base?
        FieldType::Literal(_) => true,
        FieldType::List(_) => false,
        FieldType::Map(_,_) => false,
        FieldType::Enum(_) => true,
        FieldType::Tuple(_) => false,
        FieldType::Class(_) => false,
        FieldType::Union(_) => false,
        FieldType::WithMetadata{..} => {
            unreachable!("distribute_metadata always consumes `WithMetadata`.")
        }
    };
    type_implies_done || streaming_behavior.done
}



fn completion_state(flags: &Vec<Flag>) -> CompletionState {
    if flags.iter().any(|f| matches!(f, Flag::Incomplete)) {
        CompletionState::Incomplete
    } else {
        CompletionState::Complete
    }
}

fn streaming_behavior(ir: &IntermediateRepr, r#type: &FieldType) -> StreamingBehavior {
    let (_base_type, (_constraints, streaming_behavior)) = ir.distribute_metadata(r#type);
    streaming_behavior
}
