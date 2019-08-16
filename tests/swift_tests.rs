use text_diff;
use typeshare::language::Generator;
use typeshare::swift;

#[test]
fn can_generate_simple_struct_with_a_comment() {
	let mut lang = swift::Swift::new();
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


public extension Person {
	init(data: Data) throws {
		let decoded = try JSONDecoder().decode(Person.self, from: data)
		self.init(name: decoded.name, age: decoded.age, info: decoded.info, emails: decoded.emails)
	}
}

";

	if expected != result.as_str() {
		text_diff::print_diff(expected, &result, " ");
	}

	assert_eq!(expected, &result);
}

#[test]
fn can_handle_serde_rename() {
	let mut lang = swift::Swift::new();
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


public extension Person {
	init(data: Data) throws {
		let decoded = try JSONDecoder().decode(Person.self, from: data)
		self.init(name: decoded.name, age: decoded.age, extraSpecialFieldOne: decoded.extraSpecialFieldOne, extraSpecialFieldTwo: decoded.extraSpecialFieldTwo)
	}
}

";

	if expected != result.as_str() {
		text_diff::print_diff(expected, &result, " ");
	}

	assert_eq!(expected, &result);
}

#[test]
fn can_generate_simple_enum() {
	let mut lang = swift::Swift::new();
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

	if expected != result.as_str() {
		text_diff::print_diff(expected, &result, " ");
	}

	assert_eq!(expected, &result);
}

#[test]
fn can_generate_bare_string_enum() {
	let mut lang = swift::Swift::new();
	let mut g = Generator::new(&mut lang);

	let source = r##"
/// This is a comment.
pub enum Colors {
	Red,
	Blue,
	Green,
}
   
"##;
	let mut out: Vec<u8> = Vec::new();
	assert!(g.process_source(source.to_string(), &mut out).is_ok(), "must be able to process the source");
	let result = String::from_utf8(out).unwrap();

	let expected = r#"/// 
/// Generated
/// 

import Foundation

/// This is a comment.
public enum Colors: String, Codable {
	case Red = "Red"
	case Blue = "Blue"
	case Green = "Green"
}

"#;

	if expected != result.as_str() {
		text_diff::print_diff(expected, &result, " ");
	}
	assert_eq!(expected, &result);
}
