use std::io::Write;

use crate::language::{Language, RustAlgebraicEnum, RustConstEnum, RustStruct};

pub struct TypeScript {}

fn typescript_type(s: &str) -> &str {
    match s {
        "str" | "String" => "string",
        "i8" | "i16" | "i32" | "i64" | "i128" => "number",
        "u8" | "u16" | "u32" | "u64" => "number",
        "f32" | "f64" => "number",
        "isize" => "number",
        "usize" => "number",
        "bool" => "boolean",
        "char" => "string",
        _ => s,
    }
}

impl Language for TypeScript {
    fn begin(&mut self, w: &mut dyn Write) -> std::io::Result<()> {
        write_comment(w, 0, "")?;
        write_comment(w, 0, "Generated")?;
        write_comment(w, 0, "\n")?;
        Ok(())
    }

    fn write_struct(&mut self, w: &mut dyn Write, rs: &RustStruct) -> std::io::Result<()> {
        write_comments(w, 0, &rs.comments)?;
        writeln!(w, "export interface {} {{", rs.id.original)?;

        for rf in rs.fields.iter() {
            write_comments(w, 1, &rf.comments)?;
            if rf.is_vec {
                writeln!(w, "\t{}{}: {}[];", rf.id.renamed, option_symbol(rf.is_optional), typescript_type(&rf.ty))?;
            } else {
                writeln!(w, "\t{}{}: {};", rf.id.renamed, option_symbol(rf.is_optional), typescript_type(&rf.ty))?;
            }
        }

        writeln!(w, "}}\n")?;
        Ok(())
    }

    fn write_const_enum(&mut self, w: &mut dyn Write, e: &RustConstEnum) -> std::io::Result<()> {
        write_comments(w, 0, &e.comments)?;
        writeln!(w, "export enum {} {{", e.id.original)?;

        for c in e.cases.iter() {
            let mut printed_value = lit_value(&c.value).to_string();
            if printed_value == "" {
                printed_value = format!(r##""{}""##, &c.id.renamed);
            }

            write_comments(w, 1, &c.comments)?;
            writeln!(w, "\t{} = {},", c.id.original, &printed_value)?;
        }

        writeln!(w, "}}\n")?;
        Ok(())
    }

    fn write_algebraic_enum(&mut self, w: &mut dyn Write, e: &RustAlgebraicEnum) -> std::io::Result<()> {
        write_comments(w, 0, &e.comments)?;
        write!(w, "export type {} = ", e.id.original)?;

        for case in e.cases.iter() {
            write_comments(w, 1, &case.comments)?;

            if case.value.is_vec {
                write!(w, "\n\t| {}[]", typescript_type(&case.value.ty))?;
            } else {
                write!(w, "\n\t| {}", typescript_type(&case.value.ty))?;
            }
        }
        write!(w, ";\n\n")?;
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
    writeln!(w, "{}// {}", "\t".repeat(indent), comment)?;
    Ok(())
}

fn write_comments(w: &mut dyn Write, indent: usize, comments: &Vec<String>) -> std::io::Result<()> {
    for c in comments {
        write_comment(w, indent, &c)?;
    }

    Ok(())
}
