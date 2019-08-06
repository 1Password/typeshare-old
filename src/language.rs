use syn;
use std::{error::Error, fs, io::Write};

const COMMENT_PREFIX: &'static str = "= \" ";
const COMMENT_SUFFIX: &'static str = "\"";

const OPTION_PREFIX: &'static str = "Option < ";
const OPTION_SUFFIX: &'static str = " >";

const VEC_PREFIX: &'static str = "Vec < ";
const VEC_SUFFIX: &'static str = " >";

pub trait Language {
    fn process(&mut self, w: &mut dyn Write, filename: &str) -> Result<(), Box<dyn Error>> {
        let source = fs::read_to_string(filename)?;
        let source = syn::parse_file(&source)?;

        self.begin(w)?;

        for item in source.items.iter() {
            match item {
                syn::Item::Struct(s) => self.process_struct(w, &s)?,
                syn::Item::Enum(e) => self.process_enum(w, &e)?,
                syn::Item::Fn(_) => {},
                _ => {},
            }
        }

        self.end(w)?;
        Ok(())
    }

    fn process_struct(&mut self, w: &mut dyn Write, s: &syn::ItemStruct)  -> std::io::Result<()> {
        for a in s.attrs.iter() {
            let s = a.tts.to_string();
            if s.starts_with(COMMENT_PREFIX) {
                self.write_comment(w, 0, remove_prefix_suffix(&s, COMMENT_PREFIX, COMMENT_SUFFIX))?;
            }
        }

        self.write_begin_struct(w, &s.ident.to_string())?;
        for f in s.fields.iter() {
            self.process_field(w, &f)?;
        }
        self.write_end_struct(w, &s.ident.to_string())?;
        Ok(())
    }

    fn process_enum(&mut self, w: &mut dyn Write, e: &syn::ItemEnum) -> std::io::Result<()> {
        self.write_begin_enum(w, &e.ident.to_string())?;
        for v in e.variants.iter() {
            self.process_enum_variant(w, &v)?;
        }
        self.write_end_enum(w, &e.ident.to_string())?;
        Ok(())
    }

    fn process_field(&mut self, w: &mut dyn Write, f: &syn::Field) -> std::io::Result<()> {
        for a in f.attrs.iter() {
            let s = a.tts.to_string();
            if s.starts_with(COMMENT_PREFIX) {
                self.write_comment(w, 1, remove_prefix_suffix(&s, COMMENT_PREFIX, COMMENT_SUFFIX))?;
            }
        }

        let mut ty: &str = &type_as_string(&f.ty);
        let optional = ty.starts_with(OPTION_PREFIX);
        if optional {
            ty = remove_prefix_suffix(&ty, OPTION_PREFIX, OPTION_SUFFIX);
        }

        let ident = f.ident.as_ref().map_or("???".to_string(), |id| id.to_string());
        if ty.starts_with(VEC_PREFIX) {
            let ty = &remove_prefix_suffix(&ty, VEC_PREFIX, VEC_SUFFIX);
            self.write_vec_field(w, &ident, optional, ty)?;
        } else {
            self.write_field(w, &ident, optional, ty)?;
        }

        Ok(())
    }

    fn process_enum_variant(&mut self, w: &mut dyn Write, v: &syn::Variant) -> std::io::Result<()> {
        match v.fields {
            syn::Fields::Named(_) => {
                panic!("Can't handle complex enum");
            },
            syn::Fields::Unnamed(_) => {

            },
            syn::Fields::Unit => {}
        }

        writeln!(w, "\t{} = \"{}\"", v.ident, v.ident.to_string())?;
        Ok(())
    }


    //------

    fn begin(&mut self, _w: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }

    fn end(&mut self, _w: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }

    fn write_comment(&mut self, w: &mut dyn Write, _indent: usize, comment: &str) -> std::io::Result<()>  {
        writeln!(w, "COMMENT: {}", comment)?;
        Ok(())
    }

    fn write_begin_struct(&mut self, w: &mut dyn Write, s: &str) -> std::io::Result<()>  {
        writeln!(w, "BEGIN STRUCT: {}", s)?;
        Ok(())
    }

    fn write_end_struct(&mut self, w: &mut dyn Write, s: &str) -> std::io::Result<()>  {
        writeln!(w, "END STRUCT: {}", s)?;
        Ok(())
    }

    fn write_begin_enum(&mut self, w: &mut dyn Write, s: &str) -> std::io::Result<()>  {
        writeln!(w, "BEGIN ENUM: {}", s)?;
        Ok(())
    }

    fn write_end_enum(&mut self, w: &mut dyn Write, s: &str) -> std::io::Result<()>  {
        writeln!(w, "END ENUM: {}", s)?;
        Ok(())
    }

    fn write_field(&mut self, w: &mut dyn Write, ident: &str, _optional: bool, _ty: &str) -> std::io::Result<()> {
        writeln!(w, "FIELD: {}", ident)?;
        Ok(())
    }

    fn write_vec_field(&mut self, w: &mut dyn Write, ident: &str, _optional: bool, _ty: &str) ->  std::io::Result<()>  {
        writeln!(w, "VEC FIELD: {}", ident)?;
        Ok(())
    }
}

fn type_as_string(ty: &syn::Type) -> String {
    use quote::ToTokens;

    let mut tokens = proc_macro2::TokenStream::new();
    ty.to_tokens(&mut tokens);
    tokens.to_string()
}

fn remove_prefix_suffix<'a>(src: &'a str, prefix: &'static str, suffix: &'static str) -> &'a str {
    if src.starts_with(prefix) && src.ends_with(suffix) {
        return &src[prefix.len()..src.len() - suffix.len()];
    }
    
    src
}
