use super::{BamlMediaType, FieldType, TypeValue, TypeMetadata};

impl FieldType {
    pub fn string() -> Self {
        FieldType::Primitive(TypeValue::String, TypeMetadata::default())
    }

    pub fn literal_string(value: String) -> Self {
        FieldType::Literal(super::LiteralValue::String(value), TypeMetadata::default())
    }

    pub fn literal_int(value: i64) -> Self {
        FieldType::Literal(super::LiteralValue::Int(value), TypeMetadata::default())
    }

    pub fn literal_bool(value: bool) -> Self {
        FieldType::Literal(super::LiteralValue::Bool(value), TypeMetadata::default())
    }

    pub fn int() -> Self {
        FieldType::Primitive(TypeValue::Int, TypeMetadata::default())
    }

    pub fn float() -> Self {
        FieldType::Primitive(TypeValue::Float, TypeMetadata::default())
    }

    pub fn bool() -> Self {
        FieldType::Primitive(TypeValue::Bool, TypeMetadata::default())
    }

    pub fn null() -> Self {
        FieldType::Primitive(TypeValue::Null, TypeMetadata::default())
    }

    pub fn image() -> Self {
        FieldType::Primitive(TypeValue::Media(BamlMediaType::Image), TypeMetadata::default())
    }

    pub fn r#enum(name: &str) -> Self {
        FieldType::Enum(name.to_string(), TypeMetadata::default())
    }

    pub fn class(name: &str) -> Self {
        FieldType::Class(name.to_string(), TypeMetadata::default())
    }

    pub fn list(inner: FieldType) -> Self {
        FieldType::List(Box::new(inner), TypeMetadata::default())
    }

    pub fn as_list(self) -> Self {
        FieldType::List(Box::new(self), TypeMetadata::default())
    }

    pub fn map(key: FieldType, value: FieldType) -> Self {
        FieldType::Map(Box::new(key), Box::new(value), TypeMetadata::default())
    }

    pub fn union(choices: Vec<FieldType>) -> Self {
        FieldType::Union(choices, TypeMetadata::default())
    }

    pub fn tuple(choices: Vec<FieldType>) -> Self {
        FieldType::Tuple(choices, TypeMetadata::default())
    }

    pub fn optional(inner: FieldType) -> Self {
        FieldType::Optional(Box::new(inner), TypeMetadata::default())
    }

    pub fn as_optional(self) -> Self {
        FieldType::Optional(Box::new(self), TypeMetadata::default())
    }
}
