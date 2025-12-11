//! Type system for Silk semantic analysis

use std::fmt;

/// Represents a type in the Silk type system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Integer type (int)
    Int,
    /// Floating point type (float)
    Float,
    /// String type (str)
    Str,
    /// Boolean type (bool)
    Bool,
    /// None type
    None,
    /// Any type - allows any value (for gradual typing)
    Any,
    /// Unknown type - type hasn't been inferred yet
    Unknown,
    /// Function type with parameter and return types
    Function {
        /// Parameter types (name, type) - None means no parameters stored yet
        params: Option<Vec<(String, Type)>>,
        /// Return type of the function
        return_type: Box<Type>,
    },
    /// List type with element type
    List(Box<Type>),
    /// Dictionary type with key and value types
    Dict {
        /// Type of dictionary keys
        key_type: Box<Type>,
        /// Type of dictionary values
        value_type: Box<Type>,
    },
    /// Set type with element type
    Set(Box<Type>),
    /// Tuple type with element types (heterogeneous)
    Tuple(Vec<Type>),
}

impl Type {
    /// Check if this type is compatible with another type for assignment
    ///
    /// This is used for type checking in assignments, function calls, etc.
    /// Returns true if a value of type `self` can be assigned to a variable of type `other`.
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        // Same types are always compatible
        if self == other {
            return true;
        }

        // Any is compatible with everything
        if matches!(self, Type::Any) || matches!(other, Type::Any) {
            return true;
        }

        // Unknown is compatible with everything (for gradual typing)
        if matches!(self, Type::Unknown) || matches!(other, Type::Unknown) {
            return true;
        }

        // Special case: int can be assigned to float (widening conversion)
        if matches!(self, Type::Int) && matches!(other, Type::Float) {
            return true;
        }

        // Functions are compatible if their return types are compatible
        if let (
            Type::Function {
                return_type: rt1, ..
            },
            Type::Function {
                return_type: rt2, ..
            },
        ) = (self, other)
        {
            return rt1.is_compatible_with(rt2);
        }

        // Lists are compatible if their element types are compatible
        if let (Type::List(elem1), Type::List(elem2)) = (self, other) {
            return elem1.is_compatible_with(elem2);
        }

        // Dicts are compatible if both key and value types are compatible
        if let (
            Type::Dict {
                key_type: k1,
                value_type: v1,
            },
            Type::Dict {
                key_type: k2,
                value_type: v2,
            },
        ) = (self, other)
        {
            return k1.is_compatible_with(k2) && v1.is_compatible_with(v2);
        }

        // Sets are compatible if their element types are compatible
        if let (Type::Set(elem1), Type::Set(elem2)) = (self, other) {
            return elem1.is_compatible_with(elem2);
        }

        // Tuples are compatible if they have the same length and each element type is compatible
        if let (Type::Tuple(elems1), Type::Tuple(elems2)) = (self, other) {
            if elems1.len() != elems2.len() {
                return false;
            }
            return elems1
                .iter()
                .zip(elems2.iter())
                .all(|(t1, t2)| t1.is_compatible_with(t2));
        }

        // Otherwise, types must match exactly
        false
    }

    /// Get the string representation of this type
    pub fn as_str(&self) -> &'static str {
        match self {
            Type::Int => "int",
            Type::Float => "float",
            Type::Str => "str",
            Type::Bool => "bool",
            Type::None => "None",
            Type::Any => "Any",
            Type::Unknown => "<unknown>",
            Type::Function { .. } => "function",
            Type::List(_) => "list",
            Type::Dict { .. } => "dict",
            Type::Set(_) => "set",
            Type::Tuple(_) => "tuple",
        }
    }

    /// Parse a type from a string (for type annotations)
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "str" => Some(Type::Str),
            "bool" => Some(Type::Bool),
            "None" => Some(Type::None),
            "Any" => Some(Type::Any),
            // Collection types without parameters (element type is Unknown)
            "list" => Some(Type::List(Box::new(Type::Unknown))),
            "dict" => Some(Type::Dict {
                key_type: Box::new(Type::Unknown),
                value_type: Box::new(Type::Unknown),
            }),
            "set" => Some(Type::Set(Box::new(Type::Unknown))),
            "tuple" => Some(Type::Tuple(vec![])),
            _ => None,
        }
    }

    /// Check if this is a built-in type
    pub fn is_builtin(&self) -> bool {
        matches!(
            self,
            Type::Int | Type::Float | Type::Str | Type::Bool | Type::None | Type::Any
        )
    }

    /// Check if this type can be used in numeric operations (+ - * / etc)
    ///
    /// Returns true for Int, Float, and Unknown (for gradual typing)
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::Unknown)
    }

    /// Check if this type can be used in comparison operations (< > <= >=)
    ///
    /// Returns true for Int, Float, Str, and Unknown
    pub fn is_comparable(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::Str | Type::Unknown)
    }

    /// Check if this type can be indexed/subscripted
    ///
    /// Returns true for List, Dict, Tuple, Str, and Unknown
    pub fn is_indexable(&self) -> bool {
        matches!(
            self,
            Type::List(_) | Type::Dict { .. } | Type::Tuple(_) | Type::Str | Type::Unknown
        )
    }

    /// Check if this type can be iterated over (for loops)
    ///
    /// Returns true for List, Dict, Set, Tuple, Str, and Unknown
    pub fn is_iterable(&self) -> bool {
        matches!(
            self,
            Type::List(_)
                | Type::Dict { .. }
                | Type::Set(_)
                | Type::Tuple(_)
                | Type::Str
                | Type::Unknown
        )
    }

    /// Get the index type for this container type
    ///
    /// For lists/tuples/strings: returns int
    /// For dicts: returns the key type
    /// For others: returns Unknown
    pub fn expected_index_type(&self) -> Type {
        match self {
            Type::List(_) | Type::Tuple(_) | Type::Str => Type::Int,
            Type::Dict { key_type, .. } => (**key_type).clone(),
            _ => Type::Unknown,
        }
    }

    /// Get the element/value type when indexing this container
    ///
    /// For lists: returns element type
    /// For dicts: returns value type
    /// For tuples: returns Unknown (varies by index)
    /// For strings: returns str
    pub fn index_result_type(&self) -> Type {
        match self {
            Type::List(elem_type) => (**elem_type).clone(),
            Type::Dict { value_type, .. } => (**value_type).clone(),
            Type::Tuple(_) => Type::Unknown, // Could be any element type
            Type::Str => Type::Str,
            _ => Type::Unknown,
        }
    }

    /// Check if two types can be used together in a binary operation
    ///
    /// This is a stricter check than is_compatible_with, used for operations
    /// Returns true if the operation makes sense (doesn't check result type)
    pub fn can_operate_with(&self, other: &Type, is_arithmetic: bool) -> bool {
        // Unknown is always allowed (gradual typing)
        if matches!(self, Type::Unknown) || matches!(other, Type::Unknown) {
            return true;
        }

        if is_arithmetic {
            // Arithmetic operations: both must be numeric
            // Int and Float can be mixed
            self.is_numeric() && other.is_numeric()
        } else {
            // Other operations (comparisons, etc): types should be compatible
            self.is_compatible_with(other) || other.is_compatible_with(self)
        }
    }

    /// Check if this type requires exact type matching (no coercion)
    ///
    /// Returns true for collection types where element types matter
    pub fn requires_exact_match(&self) -> bool {
        matches!(
            self,
            Type::List(_) | Type::Dict { .. } | Type::Set(_) | Type::Tuple(_)
        )
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Function { return_type, .. } => {
                write!(f, "function() -> {}", return_type)
            }
            Type::List(element_type) => {
                write!(f, "list[{}]", element_type)
            }
            Type::Dict {
                key_type,
                value_type,
            } => {
                write!(f, "dict[{}, {}]", key_type, value_type)
            }
            Type::Set(element_type) => {
                write!(f, "set[{}]", element_type)
            }
            Type::Tuple(elements) => {
                if elements.is_empty() {
                    write!(f, "tuple[]")
                } else {
                    write!(
                        f,
                        "tuple[{}]",
                        elements
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
            _ => write!(f, "{}", self.as_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_compatibility_same() {
        assert!(Type::Int.is_compatible_with(&Type::Int));
        assert!(Type::Float.is_compatible_with(&Type::Float));
        assert!(Type::Str.is_compatible_with(&Type::Str));
        assert!(Type::Bool.is_compatible_with(&Type::Bool));
        assert!(Type::None.is_compatible_with(&Type::None));
    }

    #[test]
    fn test_type_compatibility_different() {
        // Int can widen to Float (safe conversion)
        assert!(Type::Int.is_compatible_with(&Type::Float));
        // But Float cannot narrow to Int (loses precision)
        assert!(!Type::Float.is_compatible_with(&Type::Int));
        // Other types are still incompatible
        assert!(!Type::Str.is_compatible_with(&Type::Int));
        assert!(!Type::Bool.is_compatible_with(&Type::Str));
    }

    #[test]
    fn test_type_compatibility_any() {
        assert!(Type::Any.is_compatible_with(&Type::Int));
        assert!(Type::Int.is_compatible_with(&Type::Any));
        assert!(Type::Any.is_compatible_with(&Type::Str));
    }

    #[test]
    fn test_type_compatibility_unknown() {
        assert!(Type::Unknown.is_compatible_with(&Type::Int));
        assert!(Type::Int.is_compatible_with(&Type::Unknown));
        assert!(Type::Unknown.is_compatible_with(&Type::Unknown));
    }

    #[test]
    fn test_type_as_str() {
        assert_eq!(Type::Int.as_str(), "int");
        assert_eq!(Type::Float.as_str(), "float");
        assert_eq!(Type::Str.as_str(), "str");
        assert_eq!(Type::Bool.as_str(), "bool");
        assert_eq!(Type::None.as_str(), "None");
        assert_eq!(Type::Any.as_str(), "Any");
        assert_eq!(Type::Unknown.as_str(), "<unknown>");
    }

    #[test]
    fn test_type_from_str() {
        assert_eq!(Type::from_str("int"), Some(Type::Int));
        assert_eq!(Type::from_str("float"), Some(Type::Float));
        assert_eq!(Type::from_str("str"), Some(Type::Str));
        assert_eq!(Type::from_str("bool"), Some(Type::Bool));
        assert_eq!(Type::from_str("None"), Some(Type::None));
        assert_eq!(Type::from_str("Any"), Some(Type::Any));
        assert_eq!(Type::from_str("CustomType"), None);
    }

    #[test]
    fn test_type_is_builtin() {
        assert!(Type::Int.is_builtin());
        assert!(Type::Float.is_builtin());
        assert!(Type::Str.is_builtin());
        assert!(Type::Bool.is_builtin());
        assert!(Type::None.is_builtin());
        assert!(Type::Any.is_builtin());
        assert!(!Type::Unknown.is_builtin());
    }

    #[test]
    fn test_type_display() {
        assert_eq!(format!("{}", Type::Int), "int");
        assert_eq!(format!("{}", Type::Float), "float");
        assert_eq!(format!("{}", Type::Str), "str");
    }
}
