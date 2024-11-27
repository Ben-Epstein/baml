use crate::BamlMediaType;
use crate::Constraint;

mod builder;

pub type FieldType = TypeWithMeta<TypeMetadata>;

/// FieldType represents the type of either a class field or a function arg.
#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub enum TypeWithMeta<T> {
    Primitive(TypeValue, T),
    Enum(String, T),
    Literal(LiteralValue, T),
    Class(String, T),
    List(Box<TypeWithMeta<T>>, T),
    Map(Box<TypeWithMeta<T>>, Box<TypeWithMeta<T>>, T),
    Union(Vec<TypeWithMeta<T>>, T),
    Tuple(Vec<TypeWithMeta<T>>, T),
    Optional(Box<TypeWithMeta<T>>, T),
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub enum TypeValue {
    String,
    Int,
    Float,
    Bool,
    // Char,
    Null,
    Media(BamlMediaType),
}
impl TypeValue {
    pub fn from_str(s: &str) -> Option<TypeValue> {
        match s {
            "string" => Some(TypeValue::String),
            "int" => Some(TypeValue::Int),
            "float" => Some(TypeValue::Float),
            "bool" => Some(TypeValue::Bool),
            "null" => Some(TypeValue::Null),
            "image" => Some(TypeValue::Media(BamlMediaType::Image)),
            "audio" => Some(TypeValue::Media(BamlMediaType::Audio)),
            _ => None,
        }
    }
}

impl std::fmt::Display for TypeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeValue::String => write!(f, "string"),
            TypeValue::Int => write!(f, "int"),
            TypeValue::Float => write!(f, "float"),
            TypeValue::Bool => write!(f, "bool"),
            TypeValue::Null => write!(f, "null"),
            TypeValue::Media(BamlMediaType::Image) => write!(f, "image"),
            TypeValue::Media(BamlMediaType::Audio) => write!(f, "audio"),
        }
    }
}

/// Subset of [`crate::BamlValue`] allowed for literal type definitions.
#[derive(serde::Serialize, Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum LiteralValue {
    String(String),
    Int(i64),
    Bool(bool),
}

impl LiteralValue {
    pub fn literal_base_type(&self) -> FieldType {
        match self {
            Self::String(_) => FieldType::string(),
            Self::Int(_) => FieldType::int(),
            Self::Bool(_) => FieldType::bool(),
        }
    }
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::String(str) => write!(f, "\"{str}\""),
            LiteralValue::Int(int) => write!(f, "{int}"),
            LiteralValue::Bool(bool) => write!(f, "{bool}"),
        }
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct StreamingBehavior {
    pub done: bool,
    pub needed: bool,
    pub state: bool,
}

impl Default for StreamingBehavior {
    fn default() -> Self {
        StreamingBehavior {
            done: false,
            needed: false,
            state: false,
        }
    }
}

#[derive(serde::Serialize, Debug, Clone, PartialEq)]
pub struct TypeMetadata {
    pub constraints: Vec<Constraint>,
    pub streaming_behavior: StreamingBehavior,
}
impl Default for TypeMetadata {
    fn default() -> Self {
        TypeMetadata {
            constraints: Vec::new(),
            streaming_behavior: StreamingBehavior::default(),
        }
    }
}


// Impl display for FieldType
impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Enum(name, _) | FieldType::Class(name, _) => {
                write!(f, "{}", name)
            }
            FieldType::Primitive(t, _) => write!(f, "{}", t),
            FieldType::Literal(v, _) => write!(f, "{}", v),
            FieldType::Union(choices, _) => {
                write!(
                    f,
                    "({})",
                    choices
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            }
            FieldType::Tuple(choices, _) => {
                write!(
                    f,
                    "({})",
                    choices
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            FieldType::Map(k, v, _) => write!(f, "map<{}, {}>", k.to_string(), v.to_string()),
            FieldType::List(t, _) => write!(f, "{}[]", t.to_string()),
            FieldType::Optional(t, _) => write!(f, "{}?", t.to_string()),
        }
    }
}

impl FieldType {
    pub fn is_primitive(&self) -> bool {
        match self {
            FieldType::Primitive(_, _) => true,
            FieldType::Optional(t, _) => t.is_primitive(),
            FieldType::List(t, _) => t.is_primitive(),
            _ => false,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            FieldType::Optional(_, _) => true,
            FieldType::Primitive(TypeValue::Null, _) => true,
            FieldType::Union(types, _) => types.iter().any(FieldType::is_optional),
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            FieldType::Primitive(TypeValue::Null, _) => true,
            FieldType::Optional(t, _) => t.is_null(),
            _ => false,
        }
    }

    /// BAML does not support class-based subtyping. Nonetheless some builtin
    /// BAML types are subtypes of others, and we need to be able to test this
    /// when checking the types of values.
    ///
    /// For examples of pairs of types and their subtyping relationship, see
    /// this module's test suite.
    ///
    /// Consider renaming this to `is_assignable_to`.
    pub fn is_subtype_of(&self, other: &FieldType) -> bool {
        if self == other {
            true
        } else {
            if let FieldType::Union(items, _) = other {
                if items.iter().any(|item| self.is_subtype_of(item)) {
                    return true;
                }
            }
            match (self, other) {
                (FieldType::Primitive(TypeValue::Null, _), FieldType::Optional(_, _)) => true,
                (FieldType::Optional(self_item, _), FieldType::Optional(other_item, _)) => {
                    self_item.is_subtype_of(other_item)
                }
                (_, FieldType::Optional(t, _)) => self.is_subtype_of(t),
                (FieldType::Optional(_, _), _) => false,

                // Handle types that nest other types.
                (FieldType::List(self_item, _), FieldType::List(other_item, _)) => {
                    self_item.is_subtype_of(other_item)
                }
                (FieldType::List(_, _), _) => false,

                (FieldType::Map(self_k, self_v, _), FieldType::Map(other_k, other_v, _)) => {
                    other_k.is_subtype_of(self_k) && (**self_v).is_subtype_of(other_v)
                }
                (FieldType::Map(_, _, _), _) => false,

                (
                    FieldType::Literal(LiteralValue::Bool(_), _),
                    FieldType::Primitive(TypeValue::Bool, _),
                ) => true,
                (FieldType::Literal(LiteralValue::Bool(_), meta), _) => {
                    self.is_subtype_of(&FieldType::Primitive(TypeValue::Bool, meta.clone()))
                }
                (
                    FieldType::Literal(LiteralValue::Int(_), _),
                    FieldType::Primitive(TypeValue::Int, _),
                ) => true,
                (FieldType::Literal(LiteralValue::Int(_), meta), _) => {
                    self.is_subtype_of(&FieldType::Primitive(TypeValue::Int, meta.clone()))
                }
                (
                    FieldType::Literal(LiteralValue::String(_), _),
                    FieldType::Primitive(TypeValue::String, _),
                ) => true,
                (FieldType::Literal(LiteralValue::String(_), meta), _) => {
                    self.is_subtype_of(&FieldType::Primitive(TypeValue::String, meta.clone()))
                }

                (FieldType::Union(self_items, _), _) => self_items
                    .iter()
                    .all(|self_item| self_item.is_subtype_of(other)),

                (FieldType::Tuple(self_items, _), FieldType::Tuple(other_items, _)) => {
                    self_items.len() == other_items.len()
                        && self_items
                            .iter()
                            .zip(other_items)
                            .all(|(self_item, other_item)| self_item.is_subtype_of(other_item))
                }
                (FieldType::Tuple(_, _), _) => false,

                (FieldType::Primitive(_, _), _) => false,
                (FieldType::Enum(_, _), _) => false,
                (FieldType::Class(_, _), _) => false,
            }
        }
    }

    pub fn meta(&self) -> &TypeMetadata {
        match self {
            TypeWithMeta::Primitive(_,meta) => meta,
            TypeWithMeta::Enum(_, meta) => meta,
            TypeWithMeta::Literal(_, meta) => meta,
            TypeWithMeta::Class(_, meta) => meta,
            TypeWithMeta::List(_, meta) => meta,
            TypeWithMeta::Map(_, _, meta) => meta,
            TypeWithMeta::Union(_, meta) => meta,
            TypeWithMeta::Tuple(_, meta) => meta,
            TypeWithMeta::Optional(_, meta) => meta,
        }
    }

    pub fn with_meta(self, meta: TypeMetadata) -> Self {
        match self {
            TypeWithMeta::Primitive(t, _) => TypeWithMeta::Primitive(t, meta),
            TypeWithMeta::Enum(t, _) => TypeWithMeta::Enum(t, meta),
            TypeWithMeta::Literal(t, _) => TypeWithMeta::Literal(t, meta),
            TypeWithMeta::Class(t, _) => TypeWithMeta::Class(t, meta),
            TypeWithMeta::List(t, _) => TypeWithMeta::List(t, meta),
            TypeWithMeta::Map(k, v, _) => TypeWithMeta::Map(k, v, meta),
            TypeWithMeta::Union(t, _) => TypeWithMeta::Union(t, meta),
            TypeWithMeta::Tuple(t, _) => TypeWithMeta::Tuple(t, meta),
            TypeWithMeta::Optional(t, _) => TypeWithMeta::Optional(t, meta),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mk_int() -> FieldType {
        FieldType::Primitive(TypeValue::Int, TypeMetadata::default())
    }
    fn mk_bool() -> FieldType {
        FieldType::Primitive(TypeValue::Bool, TypeMetadata::default())
    }
    fn mk_str() -> FieldType {
        FieldType::Primitive(TypeValue::String, TypeMetadata::default())
    }

    fn mk_optional(ft: FieldType) -> FieldType {
        FieldType::Optional(Box::new(ft), TypeMetadata::default())
    }

    fn mk_list(ft: FieldType) -> FieldType {
        FieldType::List(Box::new(ft), TypeMetadata::default())
    }

    fn mk_tuple(ft: Vec<FieldType>) -> FieldType {
        FieldType::Tuple(ft, TypeMetadata::default())
    }
    fn mk_union(ft: Vec<FieldType>) -> FieldType {
        FieldType::Union(ft, TypeMetadata::default())
    }
    fn mk_str_map(ft: FieldType) -> FieldType {
        FieldType::Map(Box::new(mk_str()), Box::new(ft), TypeMetadata::default())
    }

    #[test]
    fn subtype_trivial() {
        assert!(mk_int().is_subtype_of(&mk_int()))
    }

    #[test]
    fn subtype_union() {
        let i = mk_int();
        let u = mk_union(vec![mk_int(), mk_str()]);
        assert!(i.is_subtype_of(&u));
        assert!(!u.is_subtype_of(&i));

        let u3 = mk_union(vec![mk_int(), mk_bool(), mk_str()]);
        assert!(i.is_subtype_of(&u3));
        assert!(u.is_subtype_of(&u3));
        assert!(!u3.is_subtype_of(&u));
    }

    #[test]
    fn subtype_optional() {
        let i = mk_int();
        let o = mk_optional(mk_int());
        assert!(i.is_subtype_of(&o));
        assert!(!o.is_subtype_of(&i));
    }

    #[test]
    fn subtype_list() {
        let l_i = mk_list(mk_int());
        let l_o = mk_list(mk_optional(mk_int()));
        assert!(l_i.is_subtype_of(&l_o));
        assert!(!l_o.is_subtype_of(&l_i));
    }

    #[test]
    fn subtype_tuple() {
        let x = mk_tuple(vec![mk_int(), mk_optional(mk_int())]);
        let y = mk_tuple(vec![mk_int(), mk_int()]);
        assert!(y.is_subtype_of(&x));
        assert!(!x.is_subtype_of(&y));
    }

    #[test]
    fn subtype_map_of_list_of_unions() {
        let x = mk_str_map(mk_list(FieldType::Class(
            "Foo".to_string(),
            TypeMetadata::default(),
        )));
        let y = mk_str_map(mk_list(mk_union(vec![
            mk_str(),
            mk_int(),
            FieldType::Class("Foo".to_string(), TypeMetadata::default()),
        ])));
        assert!(x.is_subtype_of(&y));
    }

    #[test]
    fn subtype_media() {
        let x = FieldType::Primitive(
            TypeValue::Media(BamlMediaType::Audio),
            TypeMetadata::default(),
        );
        assert!(x.is_subtype_of(&x));
    }
}
