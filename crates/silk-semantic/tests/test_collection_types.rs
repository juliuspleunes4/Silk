use silk_semantic::types::Type;

#[test]
fn test_list_type_display() {
    let list_int = Type::List(Box::new(Type::Int));
    assert_eq!(list_int.to_string(), "list[int]");

    let list_str = Type::List(Box::new(Type::Str));
    assert_eq!(list_str.to_string(), "list[str]");

    let list_unknown = Type::List(Box::new(Type::Unknown));
    assert_eq!(list_unknown.to_string(), "list[<unknown>]");
}

#[test]
fn test_list_type_compatibility() {
    let list_int1 = Type::List(Box::new(Type::Int));
    let list_int2 = Type::List(Box::new(Type::Int));
    let list_str = Type::List(Box::new(Type::Str));
    let list_unknown = Type::List(Box::new(Type::Unknown));

    // Same element types are compatible
    assert!(list_int1.is_compatible_with(&list_int2));

    // Different element types are not compatible
    assert!(!list_int1.is_compatible_with(&list_str));

    // Unknown is compatible with everything
    assert!(list_int1.is_compatible_with(&list_unknown));
    assert!(list_unknown.is_compatible_with(&list_int1));
}

#[test]
fn test_nested_list_type_display() {
    let nested = Type::List(Box::new(Type::List(Box::new(Type::Int))));
    assert_eq!(nested.to_string(), "list[list[int]]");
}

#[test]
fn test_dict_type_display() {
    let dict_str_int = Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Int),
    };
    assert_eq!(dict_str_int.to_string(), "dict[str, int]");

    let dict_int_str = Type::Dict {
        key_type: Box::new(Type::Int),
        value_type: Box::new(Type::Str),
    };
    assert_eq!(dict_int_str.to_string(), "dict[int, str]");

    let dict_unknown = Type::Dict {
        key_type: Box::new(Type::Unknown),
        value_type: Box::new(Type::Unknown),
    };
    assert_eq!(dict_unknown.to_string(), "dict[<unknown>, <unknown>]");
}

#[test]
fn test_dict_type_compatibility() {
    let dict1 = Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Int),
    };
    let dict2 = Type::Dict {
        key_type: Box::new(Type::Str),
        value_type: Box::new(Type::Int),
    };
    let dict3 = Type::Dict {
        key_type: Box::new(Type::Int),
        value_type: Box::new(Type::Str),
    };
    let dict_unknown = Type::Dict {
        key_type: Box::new(Type::Unknown),
        value_type: Box::new(Type::Unknown),
    };

    // Same key/value types are compatible
    assert!(dict1.is_compatible_with(&dict2));

    // Different key/value types are not compatible
    assert!(!dict1.is_compatible_with(&dict3));

    // Unknown is compatible with everything
    assert!(dict1.is_compatible_with(&dict_unknown));
    assert!(dict_unknown.is_compatible_with(&dict1));
}

#[test]
fn test_set_type_display() {
    let set_int = Type::Set(Box::new(Type::Int));
    assert_eq!(set_int.to_string(), "set[int]");

    let set_str = Type::Set(Box::new(Type::Str));
    assert_eq!(set_str.to_string(), "set[str]");
}

#[test]
fn test_set_type_compatibility() {
    let set_int1 = Type::Set(Box::new(Type::Int));
    let set_int2 = Type::Set(Box::new(Type::Int));
    let set_str = Type::Set(Box::new(Type::Str));

    // Same element types are compatible
    assert!(set_int1.is_compatible_with(&set_int2));

    // Different element types are not compatible
    assert!(!set_int1.is_compatible_with(&set_str));
}

#[test]
fn test_tuple_type_display() {
    let tuple_empty = Type::Tuple(vec![]);
    assert_eq!(tuple_empty.to_string(), "tuple[]");

    let tuple_single = Type::Tuple(vec![Type::Int]);
    assert_eq!(tuple_single.to_string(), "tuple[int]");

    let tuple_homogeneous = Type::Tuple(vec![Type::Int, Type::Int, Type::Int]);
    assert_eq!(tuple_homogeneous.to_string(), "tuple[int, int, int]");

    let tuple_heterogeneous = Type::Tuple(vec![Type::Int, Type::Str, Type::Float]);
    assert_eq!(tuple_heterogeneous.to_string(), "tuple[int, str, float]");
}

#[test]
fn test_tuple_type_compatibility() {
    let tuple1 = Type::Tuple(vec![Type::Int, Type::Str]);
    let tuple2 = Type::Tuple(vec![Type::Int, Type::Str]);
    let tuple3 = Type::Tuple(vec![Type::Str, Type::Int]);
    let tuple_different_length = Type::Tuple(vec![Type::Int, Type::Str, Type::Float]);

    // Same element types and length are compatible
    assert!(tuple1.is_compatible_with(&tuple2));

    // Different element types are not compatible
    assert!(!tuple1.is_compatible_with(&tuple3));

    // Different lengths are not compatible
    assert!(!tuple1.is_compatible_with(&tuple_different_length));
}

#[test]
fn test_tuple_empty_compatibility() {
    let empty1 = Type::Tuple(vec![]);
    let empty2 = Type::Tuple(vec![]);
    let non_empty = Type::Tuple(vec![Type::Int]);

    // Empty tuples are compatible with each other
    assert!(empty1.is_compatible_with(&empty2));

    // Empty tuple is not compatible with non-empty
    assert!(!empty1.is_compatible_with(&non_empty));
}

#[test]
fn test_nested_tuple_display() {
    let nested = Type::Tuple(vec![
        Type::Tuple(vec![Type::Int, Type::Int]),
        Type::Tuple(vec![Type::Str, Type::Str]),
    ]);
    assert_eq!(
        nested.to_string(),
        "tuple[tuple[int, int], tuple[str, str]]"
    );
}

#[test]
fn test_collection_as_str() {
    assert_eq!(Type::List(Box::new(Type::Int)).as_str(), "list");
    assert_eq!(
        Type::Dict {
            key_type: Box::new(Type::Str),
            value_type: Box::new(Type::Int)
        }
        .as_str(),
        "dict"
    );
    assert_eq!(Type::Set(Box::new(Type::Int)).as_str(), "set");
    assert_eq!(Type::Tuple(vec![Type::Int]).as_str(), "tuple");
}
