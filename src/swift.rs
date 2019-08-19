use std::io::Write;

use crate::language::{Language, RustAlgebraicEnum, RustConstEnum, RustStruct};

pub struct Swift {}

impl Swift {
    pub fn new() -> Self {
        Swift {}
    }
}

fn swift_type(s: &str) -> &str {
    match s {
        "String" => "String",
        "i8" => "Int8",
        "i16" => "Int16",
        "i32" => "Int32",
        "i64" => "Int64",
        "u8" => "UInt8",
        "u16" => "UInt16",
        "u32" => "UInt32",
        "u64" => "UInt64",
        "isize" => "Int",
        "usize" => "UInt",
        "bool" => "Bool",
        _ => s,
    }
}

fn swift_lit_type(lit: &Option<syn::Lit>) -> &'static str {
    match lit {
        Some(syn::Lit::Int(_)) => "Int",
        Some(syn::Lit::Str(_)) => "String",
        Some(syn::Lit::ByteStr(_)) => "[UInt8]",
        Some(syn::Lit::Byte(_)) => "UInt8",
        Some(syn::Lit::Char(_)) => "Int8",
        Some(syn::Lit::Float(_)) => "Float",
        Some(syn::Lit::Bool(_)) => "Bool",
        Some(syn::Lit::Verbatim(_)) => " ERROR ",
        None => "String", // Should be used when we have a bare enum
    }
}

impl Language for Swift {
    fn begin(&mut self, w: &mut dyn Write) -> std::io::Result<()> {
        write_comment(w, 0, "")?;
        write_comment(w, 0, "Generated")?;
        write_comment(w, 0, "\n")?;

        writeln!(w, "import Foundation\n")?;
        Ok(())
    }

    fn write_struct(&mut self, w: &mut dyn Write, rs: &RustStruct) -> std::io::Result<()> {
        write_comments(w, 0, &rs.comments)?;
        writeln!(w, "public struct {}: Codable {{", rs.id.original)?;

        for f in rs.fields.iter() {
            write_comments(w, 1, &f.comments)?;
            if f.is_vec {
                writeln!(w, "\tpublic let {}: [{}]{}", f.id.renamed, swift_type(&f.ty), option_symbol(f.is_optional))?;
            } else {
                writeln!(w, "\tpublic let {}: {}{}", f.id.renamed, swift_type(&f.ty), option_symbol(f.is_optional))?;
            }
        }

        let mut init_params: Vec<String> = Vec::new();
        for f in rs.fields.iter() {
            if f.is_vec {
                init_params.push(format!("{}: [{}]{}", f.id.renamed, swift_type(&f.ty), option_symbol(f.is_optional)));
            } else {
                init_params.push(format!("{}: {}{}", f.id.renamed, swift_type(&f.ty), option_symbol(f.is_optional)));
            }
        }

        writeln!(w, "\n\tpublic init({}) {{", init_params.join(", "))?;
        for f in rs.fields.iter() {
            writeln!(w, "\t\tself.{} = {}", f.id.renamed, f.id.renamed)?;
        }
        writeln!(w, "\t}}")?;
        writeln!(w, "}}\n")?;

        write_struct_convenience_methods(w, rs)?;

        Ok(())
    }

    fn write_const_enum(&mut self, w: &mut dyn Write, e: &RustConstEnum) -> std::io::Result<()> {
        write_comments(w, 0, &e.comments)?;
        writeln!(w, "public enum {}: {}, Codable {{", e.id.original, swift_lit_type(&e.ty))?;

        for c in e.cases.iter() {
            write_comments(w, 1, &c.comments)?;
            let mut printed_value = lit_value(&c.value).to_string();
            if printed_value == "" {
                printed_value = format!(r##""{}""##, &c.id.renamed);
            }

            writeln!(w, "\tcase {} = {}", c.id.renamed, &printed_value)?;
        }

        writeln!(w, "}}\n")?;
        Ok(())
    }

    fn write_algebraic_enum(&mut self, _w: &mut dyn Write, _e: &RustAlgebraicEnum) -> std::io::Result<()> {
        Ok(())
    }
}

fn bool_literal(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}

fn option_symbol(optional: bool) -> &'static str {
    if optional {
        "?"
    } else {
        ""
    }
}

fn write_struct_convenience_methods(w: &mut dyn Write, rs: &RustStruct) -> std::io::Result<()> {
    let data_init_params = rs
        .fields
        .iter()
        .map(|f| format!("{param}: decoded.{param}", param = f.id.renamed))
        .collect::<Vec<String>>()
        .join(", ");

    writeln!(
        w,
        "
public extension {struct} {{
\tinit(data: Data) throws {{
\t\tlet decoded = try JSONDecoder().decode({struct}.self, from: data)
\t\tself.init({params})
\t}}
}}
",
        struct = rs.id.original, params = data_init_params
    )?;

    Ok(())
}

fn lit_value(l: &Option<syn::ExprLit>) -> String {
    if l.is_none() {
        return "".to_string();
    }

    match &l.as_ref().unwrap().lit {
        syn::Lit::Str(s) => format!(r##""{}""##, s.value()),
        // syn::Lit::ByteStr(s) => format!("[{}]", &s.value().as_slice()),
        syn::Lit::Byte(s) => format!("{}", s.value()),
        syn::Lit::Char(s) => format!("{}", s.value()),
        syn::Lit::Int(s) => format!("{}", s.value()),
        syn::Lit::Float(s) => format!("{}", s.value()),
        syn::Lit::Bool(s) => format!(r##""{}""##, bool_literal(s.value)),
        // syn::Lit::Verbatim(s) => format!(r##""{}""##, s.to_string()),
        _ => "nope???".to_string(),
    }
}

fn write_comment(w: &mut dyn Write, indent: usize, comment: &str) -> std::io::Result<()> {
    writeln!(w, "{}/// {}", "\t".repeat(indent), comment)?;
    Ok(())
}

fn write_comments(w: &mut dyn Write, indent: usize, comments: &Vec<String>) -> std::io::Result<()> {
    for c in comments {
        write_comment(w, indent, &c)?;
    }

    Ok(())
}
