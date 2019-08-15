use crate::language::{Id, Language};
use std::io::Write;

pub struct Java {}

fn java_type(s: &str) -> &str {
    match s {
        "str" | "String" => "String",
        "i8" => "byte",
        "i16" => "short",
        "i32" => "int",
        "i64" => "long",
        "i128" => "java.math.BigInteger",
        "u8" => "byte",
        "u16" => "short",
        "u32" => "int",
        "u64" => "number",
        "f32" => "float",
        "f64" => "double",
        "isize" => "long",
        "usize" => "long",
        "bool" => "boolean",
        "char" => "char",
        _ => s,
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

impl Language for Java {
    fn begin(&mut self, w: &mut dyn Write) -> std::io::Result<()> {
        self.write_comment(w, 0, "")?;
        self.write_comment(w, 0, "Generated")?;
        self.write_comment(w, 0, "")?;

        writeln!(w, "package PACKAGE.NAME.WILL.BE.HERE;")?;
        writeln!(w)?;
        writeln!(w, "import java.util.*;")?;
        writeln!(w, "import com.fasterxml.jackson.annotation.*;")?;

        Ok(())
    }

    fn write_comment(&mut self, w: &mut dyn Write, indent: usize, comment: &str) -> std::io::Result<()> {
        writeln!(w, "{}// {}", "\t".repeat(indent), comment)?;
        Ok(())
    }

    fn write_begin_struct(&mut self, w: &mut dyn Write, id: &Id) -> std::io::Result<()> {
        writeln!(w, "public class {} {{", id.original)?;
        Ok(())
    }

    fn write_end_struct(&mut self, w: &mut dyn Write, _id: &Id) -> std::io::Result<()> {
        writeln!(w, "}}\n")?;
        Ok(())
    }

    fn write_begin_enum(&mut self, w: &mut dyn Write, id: &Id, _enum_type: Option<&syn::Lit>) -> std::io::Result<()> {
        writeln!(w, "// export enum {} {{", id.original)?;
        Ok(())
    }

    fn write_end_enum(&mut self, w: &mut dyn Write, _id: &Id) -> std::io::Result<()> {
        writeln!(w, "// }}\n")?;
        Ok(())
    }

    fn write_field(&mut self, w: &mut dyn Write, ident: &Id, _optional: bool, ty: &str) -> std::io::Result<()> {
        writeln!(w, "\tprivate {} {};", java_type(ty), ident.renamed)?;
        Ok(())
    }

    fn write_vec_field(&mut self, w: &mut dyn Write, ident: &Id, optional: bool, ty: &str) -> std::io::Result<()> {
        writeln!(w, "// \t{}{}: {}[];", ident.renamed, option_symbol(optional), java_type(ty))?;
        Ok(())
    }

    fn write_const_enum_variant(&mut self, w: &mut dyn Write, ident: &Id, value: &str) -> std::io::Result<()> {
        let mut printed_value = value.to_string();
        if printed_value == "" {
            printed_value = format!(r##""{}""##, &ident.renamed);
        }

        writeln!(w, "// \t{} = {},", ident.original, &printed_value)?;

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
