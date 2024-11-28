use anyhow::Result;
use baml_types::{BamlMap, Constraint, ConstraintLevel};
use internal_baml_core::{ir::FieldType, ir::TypeValue};

use crate::deserializer::{
    coercer::{run_user_checks, DefaultValue, TypeCoercer},
    deserialize_flags::{DeserializerConditions, Flag},
    types::BamlValueWithFlags,
};

use super::{
    array_helper, coerce_array::coerce_array, coerce_map::coerce_map,
    coerce_optional::coerce_optional, coerce_union::coerce_union, ir_ref::IrRef, ParsingContext,
    ParsingError,
};

impl TypeCoercer for FieldType {
    fn coerce(
        &self,
        ctx: &ParsingContext,
        target: &FieldType,
        value: Option<&crate::jsonish::Value>,
    ) -> Result<BamlValueWithFlags, ParsingError> {
        match value {
            Some(crate::jsonish::Value::AnyOf(candidates, primitive)) => {
                log::debug!(
                    "scope: {scope} :: coercing to: {name} (current: {current})",
                    name = target.to_string(),
                    scope = ctx.display_scope(),
                    current = value.map(|v| v.r#type()).unwrap_or("<null>".into())
                );
                if matches!(target, FieldType::Primitive(TypeValue::String, _)) {
                    self.coerce(
                        ctx,
                        target,
                        Some(&crate::jsonish::Value::String(primitive.clone())),
                    )
                } else {
                    array_helper::coerce_array_to_singular(
                        ctx,
                        target,
                        &candidates.iter().collect::<Vec<_>>(),
                        &|val| self.coerce(ctx, target, Some(val)),
                    )
                }
            }
            Some(crate::jsonish::Value::Markdown(_t, v)) => {
                log::debug!(
                    "scope: {scope} :: coercing to: {name} (current: {current})",
                    name = target.to_string(),
                    scope = ctx.display_scope(),
                    current = value.map(|v| v.r#type()).unwrap_or("<null>".into())
                );
                self.coerce(ctx, target, Some(v)).and_then(|mut v| {
                    v.add_flag(Flag::ObjectFromMarkdown(
                        if matches!(target, FieldType::Primitive(TypeValue::String, _)) {
                            1
                        } else {
                            0
                        },
                    ));

                    Ok(v)
                })
            }
            Some(crate::jsonish::Value::FixedJson(v, fixes)) => {
                log::debug!(
                    "scope: {scope} :: coercing to: {name} (current: {current})",
                    name = target.to_string(),
                    scope = ctx.display_scope(),
                    current = value.map(|v| v.r#type()).unwrap_or("<null>".into())
                );
                let mut v = self.coerce(ctx, target, Some(v))?;
                v.add_flag(Flag::ObjectFromFixedJson(fixes.to_vec()));
                Ok(v)
            }
            _ => match self {
                FieldType::Primitive(p, _) => p.coerce(ctx, target, value),
                FieldType::Enum(e, _) => IrRef::Enum(e).coerce(ctx, target, value),
                FieldType::Literal(l, _) => l.coerce(ctx, target, value),
                FieldType::Class(c, _) => IrRef::Class(c).coerce(ctx, target, value),
                FieldType::List(_, _) => coerce_array(ctx, self, value),
                FieldType::Union(_, _) => coerce_union(ctx, self, value),
                FieldType::Optional(_, _) => coerce_optional(ctx, self, value),
                FieldType::Map(_, _, _) => coerce_map(ctx, self, value),
                FieldType::Tuple(_, _) => Err(ctx.error_internal("Tuple not supported")),
                //// TODO: Do I need to restore constraint validation here?
                // FieldType::Constrained { base, .. } => {
                //     let mut coerced_value = base.coerce(ctx, base, value)?;
                //     let constraint_results = run_user_checks(&coerced_value.clone().into(), &self)
                //         .map_err(|e| ParsingError {
                //             reason: format!("Failed to evaluate constraints: {:?}", e),
                //             scope: ctx.scope.clone(),
                //             causes: Vec::new(),
                //         })?;
                //     validate_asserts(&constraint_results)?;
                //     let check_results = constraint_results
                //         .into_iter()
                //         .filter_map(|(maybe_check, result)| {
                //             maybe_check
                //                 .as_check()
                //                 .map(|(label, expr)| (label, expr, result))
                //         })
                //         .collect();
                //     coerced_value.add_flag(Flag::ConstraintResults(check_results));
                //     Ok(coerced_value)
                // }
            },
        }
    }
}

pub fn validate_asserts(constraints: &Vec<(Constraint, bool)>) -> Result<(), ParsingError> {
    let failing_asserts = constraints
        .iter()
        .filter_map(
            |(
                Constraint {
                    level,
                    expression,
                    label,
                },
                result,
            )| {
                if !result && ConstraintLevel::Assert == *level {
                    Some((label, expression))
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();
    let causes = failing_asserts
        .into_iter()
        .map(|(label, expr)| ParsingError {
            causes: vec![],
            reason: format!(
                "Failed: {}{}",
                label.as_ref().map_or("".to_string(), |l| format!("{} ", l)),
                expr.0
            ),
            scope: vec![],
        }).collect::<Vec<_>>();
    if causes.len() > 0 {
        Err(ParsingError {
            causes: vec![],
            reason: "Assertions failed.".to_string(),
            scope: vec![],
        })
    } else {
        Ok(())
    }
}

impl DefaultValue for FieldType {
    fn default_value(&self, error: Option<&ParsingError>) -> Option<BamlValueWithFlags> {
        let get_flags = || {
            DeserializerConditions::new().with_flag(error.map_or(Flag::DefaultFromNoValue, |e| {
                Flag::DefaultButHadUnparseableValue(e.clone())
            }))
        };

        match self {
            FieldType::Enum(e, _) => None,
            FieldType::Literal(_, _) => None,
            FieldType::Class(_, _) => None,
            FieldType::List(_, _) => Some(BamlValueWithFlags::List(get_flags(), Vec::new())),
            FieldType::Union(items, _) => items.iter().find_map(|i| i.default_value(error)),
            FieldType::Primitive(TypeValue::Null, _) | FieldType::Optional(_, _) => {
                Some(BamlValueWithFlags::Null(get_flags()))
            }
            FieldType::Map(_, _, _) => Some(BamlValueWithFlags::Map(get_flags(), BamlMap::new())),
            FieldType::Tuple(v, _) => {
                let default_values: Vec<_> = v.iter().map(|f| f.default_value(error)).collect();
                if default_values.iter().all(Option::is_some) {
                    Some(BamlValueWithFlags::List(
                        get_flags(),
                        default_values.into_iter().map(Option::unwrap).collect(),
                    ))
                } else {
                    None
                }
            }
            FieldType::Primitive(_, _) => None,
        }
    }
}
