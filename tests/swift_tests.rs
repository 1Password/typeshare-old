use typeshare::language::Generator;
use typeshare::swift;

#[test]
fn can_generate_simple_struct_with_a_comment() {
    let mut out: Vec<u8> = Vec::new();
    let mut lang = swift::Swift::new();
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

    let expected = "/// 
/// Generated
/// 

import Foundation

/// This is a comment.
public struct Person: Codable {
	public let name: String
	public let age: UInt8
	public let info: String?
	public let emails: [String]

	public init(name: String, age: UInt8, info: String?, emails: [String]) {
		self.name = name
		self.age = age
		self.info = info
		self.emails = emails
	}
}

";

    assert_eq!(expected, &result);
    println!("{}", result);
}

#[test]
fn can_handle_serde_rename() {
    let mut out: Vec<u8> = Vec::new();
    let mut lang = swift::Swift::new();
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

    let expected = "/// 
/// Generated
/// 

import Foundation

/// This is a comment.
public struct Person: Codable {
	public let name: String
	public let age: UInt8
	public let extraSpecialFieldOne: Int32
	public let extraSpecialFieldTwo: [String]?

	public init(name: String, age: UInt8, extraSpecialFieldOne: Int32, extraSpecialFieldTwo: [String]?) {
		self.name = name
		self.age = age
		self.extraSpecialFieldOne = extraSpecialFieldOne
		self.extraSpecialFieldTwo = extraSpecialFieldTwo
	}
}

";

    assert_eq!(expected, &result);
    println!("{}", result);
}

#[test]
fn can_generate_simple_enum() {
    let mut out: Vec<u8> = Vec::new();
    let mut lang = swift::Swift::new();
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

    let expected = "/// 
/// Generated
/// 

import Foundation

/// This is a comment.
public enum Colors: Int, Codable {
	case Red = 0
	case Blue = 1
	case Green = 2
}

";

    assert_eq!(expected, &result);
    println!("{}", result);
}
