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
    /// Function type with return type
    Function {
        /// Return type of the function
        return_type: Box<Type>,
    },
}

impl Type {
    /// Check if this type is compatible with another type for assignment
    /// 
    /// This is used for type checking in assignments, function calls, etc.
    /// Returns true if a value of type `other` can be assigned to a variable of type `self`.
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

        // Functions are compatible if their return types are compatible
        if let (Type::Function { return_type: rt1 }, Type::Function { return_type: rt2 }) = (self, other) {
            return rt1.is_compatible_with(rt2);
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
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Function { return_type } => {
                write!(f, "function -> {}", return_type)
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
        assert!(!Type::Int.is_compatible_with(&Type::Float));
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
