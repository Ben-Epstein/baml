// This module helps resolve baml values with attached streaming state
// in the context of the streaming behavior associated with their types.

use crate::deserializer::coercer::ParsingError;
use crate::jsonish::CompletionState;
use crate::{BamlValueWithFlags, Flag};
use internal_baml_core::ir::repr::IntermediateRepr;
use internal_baml_core::ir::IRHelper;

use baml_types::{BamlValueWithMeta, FieldType, ResponseCheck, StreamingBehavior};

/// For a given baml value, traverse its nodes, comparing the completion state
/// of each node against the streaming behavior of the node's type.
pub fn validate_streaming_state(
    ir: &IntermediateRepr,
    baml_value: &BamlValueWithFlags,
    field_type: &FieldType,
) -> Result<BamlValueWithMeta<Option<CompletionState>>, ParsingError> {
    let baml_value_with_meta_flags: BamlValueWithMeta<Vec<Flag>> = baml_value.clone().into();
    let typed_baml_value: BamlValueWithMeta<(Vec<Flag>, FieldType)> = ir.distribute_type_with_meta(baml_value_with_meta_flags, field_type.clone()).unwrap();
    todo!()
}
