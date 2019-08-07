use std::io::Write;

use crate::language::Language;

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
        self.write_comment(w, 0, "")?;
        self.write_comment(w, 0, "Generated")?;
        self.write_comment(w, 0, "\n")?;
        Ok(())
    }

    fn write_comment(
        &mut self,
        w: &mut dyn Write,
        indent: usize,
        comment: &str,
    ) -> std::io::Result<()> {
        writeln!(w, "{}// {}", "\t".repeat(indent), comment)?;
        Ok(())
    }

    fn write_begin_struct(&mut self, w: &mut dyn Write, name: &str) -> std::io::Result<()> {
        writeln!(w, "export interface {} {{", name)?;
        Ok(())
    }

    fn write_end_struct(&mut self, w: &mut dyn Write, _name: &str) -> std::io::Result<()> {
        writeln!(w, "}}\n")?;
        Ok(())
    }

    fn write_begin_enum(&mut self, w: &mut dyn Write, name: &str) -> std::io::Result<()> {
        writeln!(w, "export enum {} {{", name)?;
        Ok(())
    }

    fn write_end_enum(&mut self, w: &mut dyn Write, _name: &str) -> std::io::Result<()> {
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_field(
        &mut self,
        w: &mut dyn Write,
        ident: &str,
        optional: bool,
        ty: &str,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "\t{}{}: {};",
            ident,
            option_symbol(optional),
            typescript_type(ty)
        )?;
        Ok(())
    }

    fn write_vec_field(
        &mut self,
        w: &mut dyn Write,
        ident: &str,
        optional: bool,
        ty: &str,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "\t{}{}: {}[];",
            ident,
            option_symbol(optional),
            typescript_type(ty)
        )?;
        Ok(())
    }

    fn lit_value(&self, l: &syn::ExprLit) -> String {
        match &l.lit {
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
}

fn bool_literal(b: bool) -> String {
    match b {
        false => "false".to_string(),
        true => "true".to_string(),
    }
}

fn option_symbol(optional: bool) -> &'static str {
    match optional {
        true => "?",
        false => "",
    }
}
