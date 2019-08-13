use typeshare::language::Generator;
use typeshare::typescript;

#[test]
fn can_generate_simple_struct_with_a_comment() {
    let mut out: Vec<u8> = Vec::new();
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang, &mut out);

    let source = "
/// This is a comment.
pub struct Person {
    pub name: String,
    pub age: u8,
    pub info: Option<String>,
    pub emails: Vec<String>,
}
   
";
    assert!(g.process_source(source.to_string()).is_ok(), "must be able to process the source");
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

    assert_eq!(expected, &result);
    println!("{}", result);
}

#[test]
fn can_handle_serde_rename() {
    let mut out: Vec<u8> = Vec::new();
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang, &mut out);

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
    assert!(g.process_source(source.to_string()).is_ok(), "must be able to process the source");
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

    assert_eq!(expected, &result);
    println!("{}", result);
}

#[test]
fn can_generate_simple_enum() {
    let mut out: Vec<u8> = Vec::new();
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang, &mut out);

    let source = r##"
/// This is a comment.
pub enum Colors {
    Red = 0,
    Blue = 1,
    Green = 2,
}
   
"##;
    assert!(g.process_source(source.to_string()).is_ok(), "must be able to process the source");
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

    assert_eq!(expected, &result);
    println!("{}", result);
}
