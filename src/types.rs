use super::styling;


pub trait ToRustValue {
    fn to_src(&self) -> String;
    fn name(&self) -> String;
}


#[derive(Debug, PartialEq)]
pub struct RustBool {
    name: String
}

impl RustBool {
    pub fn new(name: String) -> RustBool {
        RustBool { name }
    }
}

impl ToRustValue for RustBool {
    fn to_src(&self) -> String {
        self.name.clone() + ": bool"
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct RustNum {
    name: String
}

impl RustNum {
    pub fn new(name: String) -> RustNum {
        RustNum { name }
    }
}

impl ToRustValue for RustNum {
    fn to_src(&self) -> String {
        self.name.clone() + ": i32"
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct RustString {
    name: String
}

impl RustString {
    pub fn new(name: String) -> RustString {
        RustString { name }
    }
}

impl ToRustValue for RustString {
    fn to_src(&self) -> String {
        self.name.clone() + ": String"
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}


pub struct RustStruct {
    name: String,
    elements: Vec<Box<ToRustValue>>,
}

impl RustStruct {
    pub fn new(name: String, elements: Vec<Box<ToRustValue>>) -> RustStruct {
        let name = styling::capitalize_string(&name);
        RustStruct { name, elements }
    }
}

impl ToRustValue for RustStruct {
    fn to_src(&self) -> String {
        let mut result: String = "\n#[derive(Debug)]\nstruct ".to_owned() + &self.name.clone() + " {\n";
        for element in self.elements.iter() {
            result += "    ";
            result.push_str(&element.clone().to_src());
            result += ",\n";
        }
        result + "}\n"
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct RustArray {
    name: String,
    element_type: String,
}

impl RustArray {
    pub fn new(name: String, element_type: String) -> RustArray {
        let element_type = styling::plural_to_singular(&element_type);
        let element_type = styling::snake_case_to_capitalized(&element_type);
        RustArray { name, element_type }
    }
}

impl ToRustValue for RustArray {
    fn to_src(&self) -> String {
        self.name.clone() + ": Vec<" + &self.element_type + ">"
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct RustElement {
    name: String,
    type_name: String,
}

impl RustElement {
    pub fn new(name: String) -> RustElement {
        let type_name = styling::capitalize_string(&name);
        RustElement { name, type_name }
    }
}

impl ToRustValue for RustElement {
    fn to_src(&self) -> String {
        self.name.clone() + ": " + &self.type_name
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct NullableType {
    name: String
}

impl NullableType {
    pub fn new(name: String) -> NullableType {
        NullableType { name }
    }
}

impl ToRustValue for NullableType {
    fn to_src(&self) -> String {
        self.name.clone() + ": Option<String>"
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

mod test {
    use super::{RustNum, RustArray, RustStruct, RustElement, RustString, RustBool, ToRustValue};

    mod primitives {
        use super::*;

        #[test]
        fn test_boolean_to_bool() {
            let value = RustBool::new("boolean".to_owned());
            assert_eq!("boolean: bool".to_owned(), value.to_src());
        }

        #[test]
        fn test_num_to_i32() {
            let value = RustNum::new("total_commits".to_owned());
            assert_eq!("total_commits: i32".to_owned(), value.to_src());
        }

        #[test]
        fn test_string_to_string() {
            let value = RustString::new("name".to_owned());
            assert_eq!("name: String".to_owned(), value.to_src());
        }
    }

    mod objects {
        use super::*;

        #[test]
        fn test_single_level_object_to_struct() {
            let vector: Vec<Box<ToRustValue>> = vec![Box::new(RustString::new("name".into())), Box::new(RustBool::new("is_good".into()))];
            let value = RustStruct::new("MyObject".to_owned(), vector);
            assert_eq!("\n#[derive(Debug)]\nstruct MyObject {\n    name: String,\n    is_good: bool,\n}\n".to_owned(), value.to_src());
        }

        #[test]
        fn test_element() {}
    }

    mod arrays {
        use super::*;

        #[test]
        fn test_basic_array() {
            let value = RustArray::new("commits".to_owned(), "GitCommits".to_owned());
            assert_eq!("commits: Vec<GitCommit>", value.to_src());
        }
    }
}
