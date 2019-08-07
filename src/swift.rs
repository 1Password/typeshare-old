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

impl Language for Swift {
    fn begin(&mut self, w: &mut dyn Write) -> std::io::Result<()> {
        self.write_comment(w, 0, "")?;
        self.write_comment(w, 0, "Generated")?;
        self.write_comment(w, 0, "\n")?;

        writeln!(w, "import Foundation\n")?;
        Ok(())
    }

    fn write_comment(
        &mut self,
        w: &mut dyn Write,
        indent: usize,
        comment: &str,
    ) -> std::io::Result<()> {
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

    fn write_begin_enum(&mut self, w: &mut dyn Write, id: &Id) -> std::io::Result<()> {
        writeln!(w, "export enum {} {{", id.original)?;
        Ok(())
    }

    fn write_end_enum(&mut self, w: &mut dyn Write, _id: &Id) -> std::io::Result<()> {
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_field(
        &mut self,
        w: &mut dyn Write,
        ident: &Id,
        optional: bool,
        ty: &str,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "\tpublic let {}: {}{}",
            ident.original,
            swift_type(ty),
            option_symbol(optional)
        )?;
        self.init_fields.push(ident.to_string());
        self.init_params
            .push(format!("{}: {}", ident, swift_type(ty)));
        Ok(())
    }

    fn write_vec_field(
        &mut self,
        w: &mut dyn Write,
        ident: &Id,
        optional: bool,
        ty: &str,
    ) -> std::io::Result<()> {
        writeln!(
            w,
            "\tpublic let {}: [{}]{}",
            ident.original,
            swift_type(ty),
            option_symbol(optional)
        )?;
        self.init_fields.push(ident.to_string());
        self.init_params
            .push(format!("{}: {}", ident, swift_type(ty)));
        Ok(())
    }
}

fn option_symbol(optional: bool) -> &'static str {
    match optional {
        true => "?",
        false => "",
    }
}
