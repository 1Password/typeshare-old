use typeshare::language::Generator;
use typeshare::typescript;

#[test]
fn test_can_generate_simple_struct() {
    let mut out: Vec<u8> = Vec::new();
    let mut lang = typescript::TypeScript {};
    let mut g = Generator::new(&mut lang, &mut out);

    let source = "

pub struct Person {
    pub name: String,
    pub age: u8,
}
   
";
    g.process_source(source.to_string());

    let result = String::from_utf8(out).unwrap();

    let expected = "// 
// Generated
// 

export interface Person {
	name: string;
	age: number;
}

";

    assert_eq!(expected, &result);
    println!("{}", result);
}
