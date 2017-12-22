#[macro_use]
extern crate serde_json;

use std::fs::File;
use std::path::Path;

use serde_json::Value;

use types::*;

mod styling;
mod types;

pub struct Converter {
    file_name: String,
    values: Value,
}

impl Converter {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Converter {
        let file_name_dirty = file_path.as_ref().file_stem().unwrap().to_str().unwrap().to_owned();
        // Tidy up file name
        let file_name = styling::snake_case_to_capitalized(&file_name_dirty);
        let file = File::open(file_path).unwrap();
        let values: Value = serde_json::from_reader(file).unwrap();

        Converter { file_name, values }
    }

    pub fn convert(self) -> Result<String, String> {
        let result = convert_full_object(self.file_name, &self.values).map(|x| x.iter().map(|x| x.to_src()).collect());
        result.map(|x: String| x.trim().to_owned())
    }
}


/// Function to convert the entire JSON object, this will be a file
fn convert_full_object(file_name: String, root_element: &Value) -> Result<Vec<RustStruct>, String> {
    // First check it is an object root element otherwise we cannot work
    if let Some(root_map) = root_element.as_object() {
        let mut result = Vec::new();
        let mut elements: Vec<Box<ToRustValue>> = Vec::new();
        for (inner_name, val) in root_map.iter() {
            if val.is_object() {
                elements.push(Box::new(RustElement::new(inner_name.to_owned())));
                let new_obj = convert_full_object(inner_name.to_owned(), val);
                if new_obj.is_ok() {
                    for elem in new_obj.unwrap() {
                        result.push(elem);
                    }
                }
            } else if val.is_array() {
                let arr = val.as_array().unwrap();
                let first = arr.get(0).unwrap();
                if first.is_object() {
                    elements.push(Box::new(RustArray::new(inner_name.to_owned(), inner_name.to_owned())));
                    let new_obj = convert_full_object(styling::plural_to_singular(inner_name), &first);
                    if new_obj.is_ok() {
                        for elem in new_obj.unwrap() {
                            result.push(elem);
                        }
                    }
                } else {
                    let output = convert_simple_element(inner_name.to_owned(), val);
                    elements.push(output);
                }
            } else {
                let output = convert_simple_element(inner_name.to_owned(), val);
                elements.push(output);
            }
        }
        result.push(RustStruct::new(file_name, elements));
        Ok(result)
    } else {
        Err("Nope".to_owned())
    }
}

// TODO Look into impl traits here
fn convert_simple_element(name: String, values: &Value) -> Box<ToRustValue> {
    match values {
        &Value::Bool(_) => Box::new(RustBool::new(name)),
        &Value::Number(_) => Box::new(RustNum::new(name)),
        &Value::String(_) => Box::new(RustString::new(name)),
        &Value::Array(ref arr) => {
            let first_element = arr.get(0).unwrap();
            let elem_type = match first_element {
                &Value::String(_) => "String",
                &Value::Bool(_) => "bool",
                &Value::Number(_) => "i32",
                _ => unimplemented!()
            };
            Box::new(RustArray::new(name.clone(), elem_type.to_owned()))
        }
        &Value::Null => Box::new(NullableType::new(name)),
        _ => {
            println!("Have not covered this value yet {:?}", values);
            unimplemented!()
        }
    }
}


mod test {
    use super::*;
    use super::types::*;

    mod conversion {
        use super::*;

        #[test]
        fn test_boolean_convert() {
            let value = Value::Bool(true);
            let result = convert_simple_element("my_bool".to_owned(), &value);
            let expected = RustBool::new("my_bool".to_owned());
            assert_eq!(expected.name(), result.name());
        }

        #[test]
        fn test_num_convert() {
            let value = json!(12);
            let name = "a_number".to_owned();
            let result = convert_simple_element(name.clone(), &value);
            let expected = RustNum::new(name);
            assert_eq!(expected.name(), result.name());
        }

        #[test]
        fn test_string_convert() {
            let value = json!("something");
            let name = "a_string".to_owned();
            let result = convert_simple_element(name.clone(), &value);
            let expected = RustString::new(name);
            assert_eq!(expected.name(), result.name());
        }

        #[test]
        fn test_struct_convert() {
            let value = json!({"str": "string", "num": 10, "bool": false});
            let name = "a_struct".to_owned();
            let result = convert_full_object(name.clone(), &value).unwrap();
            let mut elements: Vec<Box<ToRustValue>> = Vec::new();
            elements.push(Box::new(RustString::new("str".to_owned())));
            elements.push(Box::new(RustNum::new("num".to_owned())));
            elements.push(Box::new(RustBool::new("bool".to_owned())));
            let expected = RustStruct::new(name, elements);
            assert_eq!(1, result.len());
            let actual = result.get(0).unwrap();
            assert_eq!(expected.name(), actual.name());
            assert_eq!(expected.to_src(), actual.to_src());
        }

        #[test]
        fn test_array_convert() {
            // Array must be of a single type
            let value = json!(["text", "some more text"]);
            let name = "an_array".to_owned();
            let result = convert_simple_element(name.clone(), &value);
            let expected = RustArray::new(name, "String".to_owned());
            assert_eq!(expected.name(), result.name());
            assert_eq!(expected.to_src(), result.to_src());
        }
    }

    mod full_obj {
        use super::*;

        #[test]
        fn test_bool_obj() {
            let value = json!({"aBool": true});
            let name = "Something";
            let actual = convert_full_object(name.to_owned(), &value);
            let expected = RustStruct::new(name.to_owned(), vec![Box::new(RustBool::new("aBool".to_owned()))]);
            assert!(actual.is_ok());
            let mut uw_actual = actual.unwrap();
            assert_eq!(1, uw_actual.len());
            assert_eq!(uw_actual.pop().unwrap().to_src(), expected.to_src());
        }

        #[test]
        fn test_multiple_structs() {
            let value = json!({"name": "me", "age": 33, "dob": {"year": 1974, "month":8, "day": 11}});
            let name = "Something";
            let actual = convert_full_object(name.to_owned(), &value);
            let mut expected_elems: Vec<Box<ToRustValue>> = Vec::new();
            expected_elems.push(Box::new(RustString::new("name".to_owned())));
            expected_elems.push(Box::new(RustNum::new("age".to_owned())));
            expected_elems.push(Box::new(RustElement::new("dob".to_owned())));
            let expected = RustStruct::new(name.to_owned(), expected_elems);
            assert!(actual.is_ok());
            let mut uw_actual = actual.unwrap();
            assert_eq!(2, uw_actual.len());
            assert_eq!(uw_actual.pop().unwrap().to_src(), expected.to_src());
            let mut dob_elements: Vec<Box<ToRustValue>> = Vec::new();
            dob_elements.push(Box::new(RustNum::new("year".to_owned())));
            dob_elements.push(Box::new(RustNum::new("month".to_owned())));
            dob_elements.push(Box::new(RustNum::new("day".to_owned())));
            let second_expected = RustStruct::new("Dob".to_owned(), dob_elements);
            assert_eq!(second_expected.to_src(), uw_actual.pop().unwrap().to_src());
        }

        #[test]
        fn test_array_object() {
            let value = json!({"name":"Bob", "family": [{"name": "Claire", "relation": "mother"}, {"name": "Fred", "relation": "father"}], "age": 20});
            let actual = convert_full_object("someone".to_owned(), &value);
            let mut expected_elems: Vec<Box<ToRustValue>> = Vec::new();
            expected_elems.push(Box::new(RustString::new("name".to_owned())));
            expected_elems.push(Box::new(RustString::new("relation".to_owned())));
            let expected_vec_struct = RustStruct::new("Family".to_owned(), expected_elems);
            let mut root_expected_elems: Vec<Box<ToRustValue>> = Vec::new();
            root_expected_elems.push(Box::new(RustString::new("name".to_owned())));
            root_expected_elems.push(Box::new(RustArray::new("family".to_owned(), "family".to_owned())));
            root_expected_elems.push(Box::new(RustNum::new("age".to_owned())));
            let expected_root = RustStruct::new("Someone".to_owned(), root_expected_elems);

            assert!(actual.is_ok());
            let mut uw_actual = actual.unwrap();
            assert_eq!(2, uw_actual.len());
            assert_eq!(uw_actual.pop().unwrap().to_src(), expected_root.to_src());
            assert_eq!(uw_actual.pop().unwrap().to_src(), expected_vec_struct.to_src());
        }

        #[test]
        fn test_array_of_primitive() {
            let value = json!({"name":"Bob", "family": ["Claire", "Fred"], "age": 20});
            let actual = convert_full_object("someone".to_owned(), &value);
            let mut root_expected_elems: Vec<Box<ToRustValue>> = Vec::new();
            root_expected_elems.push(Box::new(RustString::new("name".to_owned())));
            root_expected_elems.push(Box::new(RustArray::new("family".to_owned(), "String".to_owned())));
            root_expected_elems.push(Box::new(RustNum::new("age".to_owned())));
            let expected_root = RustStruct::new("Someone".to_owned(), root_expected_elems);

            assert!(actual.is_ok());
            let mut uw_actual = actual.unwrap();
            assert_eq!(1, uw_actual.len());
            assert_eq!(uw_actual.pop().unwrap().to_src(), expected_root.to_src());
        }
    }

    // TODO Need to handle array of objects and array of primitives
}
