use crate::tasks::c12_object_oriented_programming::{
    Converter,
    IntToHexConverter,
    StringToIntConverter,
    demonstrate_cringify,
    demonstrate_zoo,
};

#[test]
fn test_zoo_demo() {
    demonstrate_zoo();
}

#[test]
fn test_cringify_demo() {
    demonstrate_cringify();
}

#[test]
fn test_string_to_int_converter() {
    let result = StringToIntConverter::convert("123".to_string());
    assert_eq!(result, 123);
}

#[test]
fn test_int_to_hex_converter() {
    let result = IntToHexConverter::convert(255);
    assert_eq!(result, "FF");

    let result2 = IntToHexConverter::convert(16);
    assert_eq!(result2, "10");
}
