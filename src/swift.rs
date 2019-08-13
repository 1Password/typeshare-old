use std::io::Write;

use crate::language::Id;
use crate::language::Language;

pub struct Swift {
    init_params: Vec<String>,
    init_fields: Vec<String>,
}

impl Swift {
    pub fn new() -> Self {
        Swift {
            init_params: Vec::new(),
            init_fields: Vec::new(),
        }
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

fn swift_lit_type(lit: Option<&syn::Lit>) -> &'static str {
    match lit {
        Some(syn::Lit::Int(_)) => "Int",
        Some(syn::Lit::Str(_)) => "String",
        Some(syn::Lit::ByteStr(_)) => "[UInt8]",
        Some(syn::Lit::Byte(_)) => "UInt8",
        Some(syn::Lit::Char(_)) => "Int8",
        Some(syn::Lit::Float(_)) => "Float",
        Some(syn::Lit::Bool(_)) => "Bool",
        Some(syn::Lit::Verbatim(_)) => " ERROR ",
        None => "",
    }
}

impl Language for Swift {
    fn begin(&mut self, w: &mut dyn Write) -> std::io::Result<()> {
        self.write_comment(w, 0, "")?;
        self.write_comment(w, 0, "Generated")?;
        self.write_comment(w, 0, "\n")?;

        writeln!(w, "import Foundation\n")?;
        Ok(())
    }

    fn write_comment(&mut self, w: &mut dyn Write, indent: usize, comment: &str) -> std::io::Result<()> {
        writeln!(w, "{}/// {}", "\t".repeat(indent), comment)?;
        Ok(())
    }

    fn write_begin_struct(&mut self, w: &mut dyn Write, id: &Id) -> std::io::Result<()> {
        writeln!(w, "public struct {}: Codable {{", id.original)?;
        Ok(())
    }

    fn write_end_struct(&mut self, w: &mut dyn Write, _id: &Id) -> std::io::Result<()> {
        writeln!(w, "\n\tpublic init({}) {{", self.init_params.join(", "))?;
        for f in self.init_fields.iter() {
            writeln!(w, "\t\tself.{} = {}", f, f)?;
        }
        writeln!(w, "\t}}")?;
        writeln!(w, "}}\n")?;

        self.init_fields.truncate(0);
        self.init_params.truncate(0);

        Ok(())
    }

    fn write_begin_enum(&mut self, w: &mut dyn Write, id: &Id, enum_type: Option<&syn::Lit>) -> std::io::Result<()> {
        writeln!(w, "public enum {}: {}, Codable {{", id.original, swift_lit_type(enum_type))?;
        Ok(())
    }

    fn write_end_enum(&mut self, w: &mut dyn Write, _id: &Id) -> std::io::Result<()> {
        writeln!(w, "}}\n")?;
        Ok(())
    }

    fn write_field(&mut self, w: &mut dyn Write, ident: &Id, optional: bool, ty: &str) -> std::io::Result<()> {
        writeln!(w, "\tpublic let {}: {}{}", ident.renamed, swift_type(ty), option_symbol(optional))?;
        self.init_fields.push(ident.renamed.clone());
        self.init_params.push(format!("{}: {}{}", ident.renamed, swift_type(ty), option_symbol(optional)));
        Ok(())
    }

    fn write_vec_field(&mut self, w: &mut dyn Write, ident: &Id, optional: bool, ty: &str) -> std::io::Result<()> {
        writeln!(w, "\tpublic let {}: [{}]{}", ident.renamed, swift_type(ty), option_symbol(optional))?;
        self.init_fields.push(ident.renamed.clone());
        self.init_params.push(format!("{}: [{}]{}", ident.renamed, swift_type(ty), option_symbol(optional)));
        Ok(())
    }

    fn write_const_enum_variant(&mut self, w: &mut dyn Write, ident: &Id, value: &str) -> std::io::Result<()> {
        let mut printed_value = value.to_string();
        if printed_value == "" {
            printed_value = format!(r##""{}""##, &ident.renamed);
        }

        writeln!(w, "\tcase {} = {}", ident.renamed, &printed_value)?;

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
