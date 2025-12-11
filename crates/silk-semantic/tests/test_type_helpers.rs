//! Tests for Type helper methods used in type checking

use silk_semantic::types::Type;

#[test]
fn test_is_numeric() {
    assert!(Type::Int.is_numeric());
    assert!(Type::Float.is_numeric());
    assert!(Type::Unknown.is_numeric());
    assert!(!Type::Str.is_numeric());
    assert!(!Type::Bool.is_numeric());
    assert!(!Type::None.is_numeric());
    assert!(!Type::List(Box::new(Type::Int)).is_numeric());
}

#[test]
fn test_is_comparable() {
    assert!(Type::Int.is_comparable());
    assert!(Type::Float.is_comparable());
    assert!(Type::Str.is_comparable());
    assert!(Type::Unknown.is_comparable());
    assert!(!Type::Bool.is_comparable());
    assert!(!Type::None.is_comparable());
    assert!(!Type::List(Box::new(Type::Int)).is_comparable());
}

#[test]
fn test_is_indexable() {
    assert!(Type::List(Box::new(Type::Int)).is_indexable());
    assert!(Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Int)
    }
    .is_indexable());
    assert!(Type::Tuple(vec![Type::Int, Type::Str]).is_indexable());
    assert!(Type::Str.is_indexable());
    assert!(Type::Unknown.is_indexable());
    assert!(!Type::Int.is_indexable());
    assert!(!Type::Float.is_indexable());
    assert!(!Type::Bool.is_indexable());
}

#[test]
fn test_is_iterable() {
    assert!(Type::List(Box::new(Type::Int)).is_iterable());
    assert!(Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Int)
    }
    .is_iterable());
    assert!(Type::Set(Box::new(Type::Int)).is_iterable());
    assert!(Type::Tuple(vec![Type::Int, Type::Str]).is_iterable());
    assert!(Type::Str.is_iterable());
    assert!(Type::Unknown.is_iterable());
    assert!(!Type::Int.is_iterable());
    assert!(!Type::Float.is_iterable());
    assert!(!Type::Bool.is_iterable());
}

#[test]
fn test_expected_index_type_list() {
    let list_type = Type::List(Box::new(Type::Int));
    assert_eq!(list_type.expected_index_type(), Type::Int);
}

#[test]
fn test_expected_index_type_tuple() {
    let tuple_type = Type::Tuple(vec![Type::Int, Type::Str]);
    assert_eq!(tuple_type.expected_index_type(), Type::Int);
}

#[test]
fn test_expected_index_type_str() {
    assert_eq!(Type::Str.expected_index_type(), Type::Int);
}

#[test]
fn test_expected_index_type_dict() {
    let dict = Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Int),
    };
    assert_eq!(dict.expected_index_type(), Type::Str);
}

#[test]
fn test_expected_index_type_non_indexable() {
    assert_eq!(Type::Int.expected_index_type(), Type::Unknown);
    assert_eq!(Type::Float.expected_index_type(), Type::Unknown);
}

#[test]
fn test_index_result_type_list() {
    let list_type = Type::List(Box::new(Type::Int));
    assert_eq!(list_type.index_result_type(), Type::Int);

    let nested_list = Type::List(Box::new(Type::List(Box::new(Type::Str))));
    assert_eq!(
        nested_list.index_result_type(),
        Type::List(Box::new(Type::Str))
    );
}

#[test]
fn test_index_result_type_dict() {
    let dict = Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Float),
    };
    assert_eq!(dict.index_result_type(), Type::Float);
}

#[test]
fn test_index_result_type_str() {
    assert_eq!(Type::Str.index_result_type(), Type::Str);
}

#[test]
fn test_index_result_type_tuple() {
    let tuple_type = Type::Tuple(vec![Type::Int, Type::Str]);
    assert_eq!(tuple_type.index_result_type(), Type::Unknown);
}

#[test]
fn test_index_result_type_non_indexable() {
    assert_eq!(Type::Int.index_result_type(), Type::Unknown);
}

#[test]
fn test_can_operate_with_arithmetic_same_numeric() {
    assert!(Type::Int.can_operate_with(&Type::Int, true));
    assert!(Type::Float.can_operate_with(&Type::Float, true));
}

#[test]
fn test_can_operate_with_arithmetic_mixed_numeric() {
    assert!(Type::Int.can_operate_with(&Type::Float, true));
    assert!(Type::Float.can_operate_with(&Type::Int, true));
}

#[test]
fn test_can_operate_with_arithmetic_non_numeric() {
    assert!(!Type::Int.can_operate_with(&Type::Str, true));
    assert!(!Type::Str.can_operate_with(&Type::Float, true));
    assert!(!Type::Bool.can_operate_with(&Type::Int, true));
}

#[test]
fn test_can_operate_with_arithmetic_unknown() {
    assert!(Type::Unknown.can_operate_with(&Type::Int, true));
    assert!(Type::Str.can_operate_with(&Type::Unknown, true));
    assert!(Type::Unknown.can_operate_with(&Type::Unknown, true));
}

#[test]
fn test_can_operate_with_comparison_same_types() {
    assert!(Type::Int.can_operate_with(&Type::Int, false));
    assert!(Type::Str.can_operate_with(&Type::Str, false));
    assert!(Type::Bool.can_operate_with(&Type::Bool, false));
    assert!(Type::Float.can_operate_with(&Type::Float, false));
}

#[test]
fn test_can_operate_with_comparison_unknown() {
    assert!(Type::Unknown.can_operate_with(&Type::Bool, false));
    assert!(Type::Int.can_operate_with(&Type::Unknown, false));
}

#[test]
fn test_requires_exact_match_collections() {
    assert!(Type::List(Box::new(Type::Int)).requires_exact_match());
    assert!(Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Int)
    }
    .requires_exact_match());
    assert!(Type::Set(Box::new(Type::Int)).requires_exact_match());
    assert!(Type::Tuple(vec![Type::Int]).requires_exact_match());
}

#[test]
fn test_requires_exact_match_primitives() {
    assert!(!Type::Int.requires_exact_match());
    assert!(!Type::Str.requires_exact_match());
    assert!(!Type::Float.requires_exact_match());
    assert!(!Type::Bool.requires_exact_match());
    assert!(!Type::None.requires_exact_match());
    assert!(!Type::Unknown.requires_exact_match());
}

#[test]
fn test_helper_methods_integration() {
    // Test that list[int] is indexable, iterable, but not numeric
    let list_int = Type::List(Box::new(Type::Int));
    assert!(list_int.is_indexable());
    assert!(list_int.is_iterable());
    assert!(!list_int.is_numeric());
    assert!(!list_int.is_comparable());
    assert_eq!(list_int.expected_index_type(), Type::Int);
    assert_eq!(list_int.index_result_type(), Type::Int);
}
