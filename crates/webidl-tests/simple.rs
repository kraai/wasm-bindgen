use wasm_bindgen_test::*;

include!(concat!(env!("OUT_DIR"), "/simple.rs"));

#[wasm_bindgen_test]
fn method() {
    let pi = Method::new(3.14159).unwrap();
    let e = Method::new(2.71828).unwrap();
    assert!(pi.my_cmp(&pi));
    assert!(!pi.my_cmp(&e));
    assert!(!e.my_cmp(&pi));
    assert!(e.my_cmp(&e));
}

#[wasm_bindgen_test]
fn property() {
    let x = Property::new(3.14159).unwrap();
    assert_eq!(x.value(), 3.14159);
    assert_ne!(x.value(), 2.71828);
    x.set_value(2.71828);
    assert_ne!(x.value(), 3.14159);
    assert_eq!(x.value(), 2.71828);
}

#[wasm_bindgen_test]
fn named_constructor() {
    let x = NamedConstructor::new(3.14159).unwrap();
    assert_eq!(x.value(), 3.14159);
    assert_ne!(x.value(), 0.);
}

#[wasm_bindgen_test]
fn static_method() {
    assert_eq!(StaticMethod::swap(3.14159), 0.);
    assert_eq!(StaticMethod::swap(2.71828), 3.14159);
    assert_ne!(StaticMethod::swap(2.71828), 3.14159);
    assert_eq!(StaticMethod::swap(3.14159), 2.71828);
    assert_ne!(StaticMethod::swap(3.14159), 2.71828);
}

#[wasm_bindgen_test]
fn static_property() {
    assert_eq!(StaticProperty::value(), 0.);
    StaticProperty::set_value(3.14159);
    assert_eq!(StaticProperty::value(), 3.14159);
    assert_ne!(StaticProperty::value(), 2.71828);
    StaticProperty::set_value(2.71828);
    assert_eq!(StaticProperty::value(), 2.71828);
    assert_ne!(StaticProperty::value(), 3.14159);
}

#[wasm_bindgen_test]
fn one_method_using_an_undefined_import_doesnt_break_all_other_methods() {
    let f = UndefinedMethod::new().unwrap();
    assert!(f.ok_method());
}

#[wasm_bindgen_test]
fn nullable_method() {
    let f = NullableMethod::new().unwrap();
    assert!(f.opt(Some(15)) == Some(16));
    assert!(f.opt(None) == None);
}

#[wasm_bindgen_test]
fn global_method() {
    let f = GlobalMethod::new().unwrap();
    assert_eq!(f.m(), 123);
}

#[wasm_bindgen_test]
fn indexing() {
    let f = Indexing::new().unwrap();
    assert_eq!(f.get(123), -1);
    f.set(123, 456);
    assert_eq!(f.get(123), 456);
    f.delete(123);
    assert_eq!(f.get(123), -1);
}

#[wasm_bindgen_test]
fn optional_and_union_arguments() {
    let f = OptionalAndUnionArguments::new().unwrap();
    assert_eq!(f.m_using_a("abc"), "string, abc, boolean, true, number, 123, number, 456");
    assert_eq!(f.m_using_a_and_b("abc", false), "string, abc, boolean, false, number, 123, number, 456");
    assert_eq!(f.m_using_dom_str_and_bool_and_i16("abc", false, 5), "string, abc, boolean, false, number, 5, number, 456");
    assert_eq!(f.m_using_dom_str_and_bool_and_dom_str("abc", false, "5"), "string, abc, boolean, false, string, 5, number, 456");
    assert_eq!(f.m_using_dom_str_and_bool_and_i16_and_opt_i64("abc", false, 5, Some(10)), "string, abc, boolean, false, number, 5, bigint, 10");
    assert_eq!(f.m_using_dom_str_and_bool_and_i16_and_opt_bool("abc", false, 5, Some(true)), "string, abc, boolean, false, number, 5, boolean, true");
    assert_eq!(f.m_using_dom_str_and_bool_and_dom_str_and_opt_i64("abc", false, "5", Some(10)), "string, abc, boolean, false, string, 5, bigint, 10");
    assert_eq!(f.m_using_dom_str_and_bool_and_dom_str_and_opt_bool("abc", false, "5", Some(true)), "string, abc, boolean, false, string, 5, boolean, true");
}

#[wasm_bindgen_test]
fn unforgeable_is_structural() {
    let f = Unforgeable::new().unwrap();
    assert_eq!(f.uno(), 1);
    assert_eq!(f.dos(), 2);
}

#[wasm_bindgen_test]
fn partial_interface() {
    let f = PartialInterface::new().unwrap();
    assert_eq!(f.un(), 1);
    assert_eq!(f.deux(), 2);
    assert_eq!(f.trois(), 3);
    assert_eq!(f.quatre(), 4);
}

#[wasm_bindgen_test]
fn mixin() {
    let f = MixinFoo::new(1).unwrap();
    assert_eq!(f.bar(), 1);
    MixinFoo::set_default_bar(7);
    f.add_to_bar(MixinFoo::default_bar());
    assert_eq!(f.bar(), 8);
}
