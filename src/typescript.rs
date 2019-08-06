use std::io::Write;

use crate::language::Language;

pub struct TypeScript {
}

fn typescript_type(s: &str) -> &str {
    match s {
        "String" => "string",
        "i8" | "i16" | "i32" | "i64" => "number",
        "u8" | "u16" | "u32" | "u64" => "number",
        "isize" => "number",
        "usize" => "number",
        "bool" => "boolean",
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

    fn write_comment(&mut self, w: &mut dyn Write, indent: usize, comment: &str) -> std::io::Result<()>  {
        writeln!(w, "{}// {}", "\t".repeat(indent), comment)?;
        Ok(())
    }

    fn write_begin_struct(&mut self, w: &mut dyn Write, name: &str) -> std::io::Result<()>  {
        writeln!(w, "export interface {} {{", name)?;
        Ok(())
    }

    fn write_end_struct(&mut self, w: &mut dyn Write, _name: &str) -> std::io::Result<()>  {
        writeln!(w, "}}\n")?;
        Ok(())
    }

    fn write_begin_enum(&mut self, w: &mut dyn Write, name: &str) -> std::io::Result<()>  {
        writeln!(w, "export enum {} {{", name)?;
        Ok(())
    }

    fn write_end_enum(&mut self, w: &mut dyn Write, _name: &str) -> std::io::Result<()>  {
        writeln!(w, "}}")?;
        Ok(())
    }

    fn write_field(&mut self, w: &mut dyn Write, ident: &str, optional: bool, ty: &str) ->  std::io::Result<()>  {
        writeln!(w, "\t{}{}: {};", ident, option_symbol(optional), typescript_type(ty))?;
        Ok(())
    }

    fn write_vec_field(&mut self, w: &mut dyn Write, ident: &str, optional: bool, ty: &str) ->  std::io::Result<()>  {
        writeln!(w, "\t{}{}: {}[];", ident, option_symbol(optional), typescript_type(ty))?;
        Ok(())
    }
}
 
fn option_symbol(optional: bool) -> &'static str {
    match optional {
        true => "?",
        false => "",
    }
}