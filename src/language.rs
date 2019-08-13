use std::{error::Error, fs, io::Write};
use syn;

const COMMENT_PREFIX: &str = "= \" ";
const COMMENT_SUFFIX: &str = "\"";

const OPTION_PREFIX: &str = "Option < ";
const OPTION_SUFFIX: &str = " >";

const VEC_PREFIX: &str = "Vec < ";
const VEC_SUFFIX: &str = " >";

#[derive(Clone)]
pub struct Id {
    pub original: String,
    pub renamed: String,
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.original == self.renamed {
            write!(f, "({})", self.original)
        } else {
            write!(f, "({}, {})", self.original, self.renamed)
        }
    }
}

pub trait Language {
    fn begin(&mut self, _w: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }

    fn end(&mut self, _w: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }

    fn write_comment(&mut self, w: &mut dyn Write, _indent: usize, comment: &str) -> std::io::Result<()>;
    fn write_begin_struct(&mut self, w: &mut dyn Write, id: &Id) -> std::io::Result<()>;
    fn write_end_struct(&mut self, w: &mut dyn Write, id: &Id) -> std::io::Result<()>;
    fn write_begin_enum(&mut self, w: &mut dyn Write, id: &Id) -> std::io::Result<()>;
    fn write_end_enum(&mut self, w: &mut dyn Write, id: &Id) -> std::io::Result<()>;
    fn write_field(&mut self, w: &mut dyn Write, ident: &Id, _optional: bool, _ty: &str) -> std::io::Result<()>;
    fn write_vec_field(&mut self, w: &mut dyn Write, ident: &Id, _optional: bool, _ty: &str) -> std::io::Result<()>;
    fn write_const_enum_variant(&mut self, w: &mut dyn Write, ident: &Id, value: &str) -> std::io::Result<()>;
    fn lit_value(&self, l: &syn::ExprLit) -> String;
}

pub struct Generator<'l, 'w> {
    language: &'l mut dyn Language,
    pub writer: &'w mut dyn Write,
}

impl<'l, 'w> Generator<'l, 'w> {
    pub fn new(language: &'l mut dyn Language, writer: &'w mut dyn Write) -> Self {
        Self { language, writer }
    }

    pub fn process_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let source = fs::read_to_string(filename)?;
        self.process_source(source)
    }

    pub fn process_source(&mut self, source: String) -> Result<(), Box<dyn Error>> {
        let source = syn::parse_file(&source)?;

        self.language.begin(self.writer)?;

        for item in source.items.iter() {
            match item {
                syn::Item::Struct(s) => self.process_struct(&s)?,
                syn::Item::Enum(e) => self.process_enum(&e)?,
                syn::Item::Fn(_) => {}
                _ => {}
            }
        }

        self.language.end(self.writer)?;
        Ok(())
    }

    fn process_struct(&mut self, s: &syn::ItemStruct) -> std::io::Result<()> {
        self.process_comment_attrs(0, &s.attrs)?;
        let ident = get_ident(Some(&s.ident), &s.attrs);
        self.language.write_begin_struct(self.writer, &ident)?;
        for f in s.fields.iter() {
            self.process_field(&f)?;
        }
        self.language.write_end_struct(self.writer, &ident)?;
        Ok(())
    }

    fn process_field(&mut self, f: &syn::Field) -> std::io::Result<()> {
        self.process_comment_attrs(1, &f.attrs)?;

        let mut ty: &str = &type_as_string(&f.ty);
        let optional = ty.starts_with(OPTION_PREFIX);
        if optional {
            ty = remove_prefix_suffix(&ty, OPTION_PREFIX, OPTION_SUFFIX);
        }

        let ident = get_ident(f.ident.as_ref(), &f.attrs);

        if ty.starts_with(VEC_PREFIX) {
            let ty = &remove_prefix_suffix(&ty, VEC_PREFIX, VEC_SUFFIX);
            self.language.write_vec_field(self.writer, &ident, optional, ty)?;
        } else {
            self.language.write_field(self.writer, &ident, optional, ty)?;
        }

        Ok(())
    }

    fn process_enum(&mut self, e: &syn::ItemEnum) -> std::io::Result<()> {
        self.process_comment_attrs(0, &e.attrs)?;
        if is_const_enum(e) {
            self.process_const_enum(e)?;
        } else {
            self.process_algebraic_enum(e)?;
        }

        Ok(())
    }

    fn process_const_enum(&mut self, e: &syn::ItemEnum) -> std::io::Result<()> {
        let ident = get_ident(Some(&e.ident), &e.attrs);
        self.language.write_begin_enum(self.writer, &ident)?;
        for v in e.variants.iter() {
            self.process_const_enum_variant(&v)?;
        }
        self.language.write_end_enum(self.writer, &ident)?;
        Ok(())
    }

    fn process_algebraic_enum(&mut self, _e: &syn::ItemEnum) -> std::io::Result<()> {
        Ok(())
    }

    fn process_const_enum_variant(&mut self, v: &syn::Variant) -> std::io::Result<()> {
        self.process_comment_attrs(1, &v.attrs)?;
        let value = {
            if let Some(d) = &v.discriminant {
                // if d.0 != syn::Token![=] {
                //     panic!("unexpected token");
                // }

                match &d.1 {
                    syn::Expr::Lit(l) => self.language.lit_value(l),
                    _ => {
                        panic!("unexpected expr");
                    }
                }
            } else {
                "".to_string()
            }
        };

        self.language.write_const_enum_variant(self.writer, &get_ident(Some(&v.ident), &v.attrs), &value)?;
        Ok(())
    }

    //----

    fn process_comment_attrs(&mut self, indent: usize, attrs: &[syn::Attribute]) -> std::io::Result<()> {
        for a in attrs.iter() {
            let s = a.tts.to_string();
            if s.starts_with(COMMENT_PREFIX) {
                self.language.write_comment(self.writer, indent, remove_prefix_suffix(&s, COMMENT_PREFIX, COMMENT_SUFFIX))?;
            }
        }

        Ok(())
    }
}

fn is_const_enum(e: &syn::ItemEnum) -> bool {
    for v in e.variants.iter() {
        match v.fields {
            syn::Fields::Named(_) => {
                panic!("Can't handle complex enum");
            }
            syn::Fields::Unnamed(_) => return false,
            syn::Fields::Unit => {}
        }
    }

    true
}

fn type_as_string(ty: &syn::Type) -> String {
    use quote::ToTokens;

    let mut tokens = proc_macro2::TokenStream::new();
    ty.to_tokens(&mut tokens);
    tokens.to_string()
}

fn get_ident(ident: Option<&proc_macro2::Ident>, attrs: &[syn::Attribute]) -> Id {
    let original = ident.map_or("???".to_string(), |id| id.to_string().replace("r#", ""));
    match serde_rename(attrs) {
        Some(s) => Id { original, renamed: s },
        None => Id {
            original: original.clone(),
            renamed: original,
        },
    }
}

fn serde_rename(attrs: &[syn::Attribute]) -> Option<String> {
    const RENAME_PREFIX: &str = r##"rename = ""##;
    const RENAME_SUFFIX: &str = r##"""##;

    for a in attrs.iter() {
        let attr_as_string = a.tts.to_string();
        let values = parse_attr(&attr_as_string)?;

        for v in values {
            if v.starts_with(RENAME_PREFIX) && v.ends_with(RENAME_SUFFIX) {
                return Some(remove_prefix_suffix(&v, RENAME_PREFIX, RENAME_SUFFIX).to_string());
            }
        }
    }

    None
}

fn parse_attr<'x>(attr: &'x str) -> Option<Vec<&'x str>> {
    const ATTR_PREFIX: &str = "( ";
    const ATTR_SUFFIX: &str = " )";

    if attr.starts_with(ATTR_PREFIX) && attr.ends_with(ATTR_SUFFIX) {
        let attr = remove_prefix_suffix(attr, ATTR_PREFIX, ATTR_SUFFIX);
        return Some(attr.split(" , ").collect());
    }

    None
}

fn remove_prefix_suffix<'a>(src: &'a str, prefix: &'static str, suffix: &'static str) -> &'a str {
    if src.starts_with(prefix) && src.ends_with(suffix) {
        return &src[prefix.len()..src.len() - suffix.len()];
    }
    src
}
