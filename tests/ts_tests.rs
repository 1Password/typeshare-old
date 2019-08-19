use text_diff;
use typeshare::language::Generator;
use typeshare::typescript;

#[test]
fn can_generate_simple_struct_with_a_comment() {
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang);

    let source = "
/// This is a comment.
pub struct Person {
    pub name: String,
    pub age: u8,
    pub info: Option<String>,
    pub emails: Vec<String>,
}
   
";

    let mut out: Vec<u8> = Vec::new();
    assert!(g.process_source(source.to_string(), &mut out).is_ok(), "must be able to process the source");
    let result = String::from_utf8(out).unwrap();

    let expected = "// 
// Generated
// 

// This is a comment.
export interface Person {
	name: string;
	age: number;
	info?: string;
	emails: string[];
}

";

    if expected != result.as_str() {
        text_diff::print_diff(expected, &result, " ");
    }
    assert_eq!(expected, &result);
}

#[test]
fn can_handle_serde_rename() {
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang);

    let source = r##"
/// This is a comment.
pub struct Person {
    pub name: String,
    pub age: u8,
    #[serde(rename="extraSpecialFieldOne")]
    pub extra_special_field1: i32,
    #[serde(rename="extraSpecialFieldTwo")]
    pub extra_special_field2: Option<Vec<String>>,
}
   
"##;

    let mut out: Vec<u8> = Vec::new();
    assert!(g.process_source(source.to_string(), &mut out).is_ok(), "must be able to process the source");
    let result = String::from_utf8(out).unwrap();

    let expected = "// 
// Generated
// 

// This is a comment.
export interface Person {
	name: string;
	age: number;
	extraSpecialFieldOne: number;
	extraSpecialFieldTwo?: string[];
}

";

    if expected != result.as_str() {
        text_diff::print_diff(expected, &result, " ");
    }
    assert_eq!(expected, &result);
}

#[test]
fn can_handle_serde_rename_all() {
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang);

    let source = r##"
/// This is a Person struct with camelCase rename
#[serde(default, rename_all = "camelCase")]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub extra_special_field1: i32,
    pub extra_special_field2: Option<Vec<String>>,
}

/// This is a Person2 struct with UPPERCASE rename
#[serde(default, rename_all = "UPPERCASE")]
pub struct Person2 {
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
}

"##;

    let mut out: Vec<u8> = Vec::new();
    assert!(g.process_source(source.to_string(), &mut out).is_ok(), "must be able to process the source");
    let result = String::from_utf8(out).unwrap();

    let expected = "// 
// Generated
// 

// This is a Person struct with camelCase rename
export interface Person {
	firstName: string;
	lastName: string;
	age: number;
	extraSpecialField1: number;
	extraSpecialField2?: string[];
}

// This is a Person2 struct with UPPERCASE rename
export interface Person2 {
	FIRST_NAME: string;
	LAST_NAME: string;
	AGE: number;
}

";

    if expected != result.as_str() {
        text_diff::print_diff(expected, &result, " ");
    }
    assert_eq!(expected, &result);
}

#[test]
fn can_generate_simple_enum() {
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang);

    let source = r##"
/// This is a comment.
pub enum Colors {
    Red = 0,
    Blue = 1,
    Green = 2,
}
   
"##;

    let mut out: Vec<u8> = Vec::new();
    assert!(g.process_source(source.to_string(), &mut out).is_ok(), "must be able to process the source");
    let result = String::from_utf8(out).unwrap();

    let expected = "// 
// Generated
// 

// This is a comment.
export enum Colors {
	Red = 0,
	Blue = 1,
	Green = 2,
}

";

    if expected != result.as_str() {
        text_diff::print_diff(expected, &result, " ");
    }
    assert_eq!(expected, &result);
}

#[test]
fn can_generate_algebraic_enum() {
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang);

    let string_enum = r##"
pub enum AdvancedColors {
    string(String),
    number(i32),
    numberArray(Vec<i32>),
    reallyCoolType(ItemDetailsFieldValue),
}"##;

    let mut out: Vec<u8> = Vec::new();
    assert!(g.process_source(string_enum.to_string(), &mut out).is_ok(), "must be able to process the source");
    let result = String::from_utf8(out).unwrap();

    let expected = "// 
// Generated
// 

export type AdvancedColors = 
	| string
	| number
	| number[]
	| ItemDetailsFieldValue;

";

    if expected != result.as_str() {
        text_diff::print_diff(expected, &result, " ");
    }
    assert_eq!(expected, &result);
}
