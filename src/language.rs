use proc_macro2::{Ident, Span};
use std::{error::Error, fs, io::Write};
use syn;

use inflector::Inflector;

const COMMENT_PREFIX: &str = "= \" ";
const COMMENT_SUFFIX: &str = "\"";

const OPTION_PREFIX: &str = "Option < ";
const OPTION_SUFFIX: &str = " >";

const VEC_PREFIX: &str = "Vec < ";
const VEC_SUFFIX: &str = " >";

/// Identifier used in Rust structs, enums, and fields. It includes the `original` name and the `renamed` value after the transformation based on `serde` attributes.
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

/// Rust struct.
pub struct RustStruct {
    pub id: Id,
    pub fields: Vec<RustField>,
    pub comments: Vec<String>,
}

/// Rust field defintion.
pub struct RustField {
    pub id: Id,
    pub ty: String,
    pub is_optional: bool,
    pub is_vec: bool,
    pub comments: Vec<String>,
}

/// Definition of constant enums.
pub struct RustConstEnum {
    pub id: Id,
    pub comments: Vec<String>,
    pub ty: Option<syn::Lit>,
    pub consts: Vec<RustConst>,
}

pub struct RustConst {
    pub id: Id,
    pub comments: Vec<String>,
    pub value: Option<syn::ExprLit>,
}

pub trait Language {
    fn begin_file(&mut self, _w: &mut dyn Write, _params: &Params) -> std::io::Result<()> {
        Ok(())
    }

    fn end_file(&mut self, _w: &mut dyn Write, _params: &Params) -> std::io::Result<()> {
        Ok(())
    }

    fn write_struct(&mut self, w: &mut dyn Write, params: &Params, rs: &RustStruct) -> std::io::Result<()>;
    fn write_const_enum(&mut self, w: &mut dyn Write, params: &Params, e: &RustConstEnum) -> std::io::Result<()>;
}

pub struct Params {
    pub use_marker: bool,
    pub swift_prefix: String,
}

pub struct Generator<'l> {
    params: Params,
    language: &'l mut dyn Language,
    serde_rename_all: Option<String>,

    structs: Vec<RustStruct>,
    const_enums: Vec<RustConstEnum>,
}

impl<'l> Generator<'l> {
    pub fn new(language: &'l mut dyn Language, params: Params) -> Self {
        Self {
            params,
            language,
            serde_rename_all: None,

            structs: Vec::new(),
            const_enums: Vec::new(),
        }
    }

    pub fn process_file(&mut self, filename: &str, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let source = fs::read_to_string(filename)?;
        self.process_source(source, w)?;
        Ok(())
    }

    pub fn process_source(&mut self, source: String, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        let source = syn::parse_file(&source)?;
        for item in source.items.iter() {
            match item {
                syn::Item::Struct(s) => self.parse_struct(&s)?,
                syn::Item::Enum(e) => self.parse_enum(&e)?,
                syn::Item::Fn(_) => {}
                _ => {}
            }
        }

        self.write(w)?;
        Ok(())
    }

    pub fn write(&mut self, w: &mut dyn Write) -> Result<(), Box<dyn Error>> {
        self.language.begin_file(w, &self.params)?;

        for s in &self.structs {
            self.language.write_struct(w, &self.params, &s)?;
        }

        for e in &self.const_enums {
            self.language.write_const_enum(w, &self.params, &e)?;
        }

        self.language.end_file(w, &self.params)?;
        Ok(())
    }

    fn parse_struct(&mut self, s: &syn::ItemStruct) -> std::io::Result<()> {
        if self.params.use_marker && !has_typeshare_marker(&s.attrs) {
            return Ok(());
        }

        self.serde_rename_all = serde_rename_all(&s.attrs);

        let mut rs = RustStruct {
            id: get_ident(Some(&s.ident), &s.attrs, &self.serde_rename_all),
            fields: Vec::new(),
            comments: Vec::new(),
        };
        self.parse_comment_attrs(&mut rs.comments, &s.attrs)?;

        for f in s.fields.iter() {
            self.parse_field(&mut rs, &f)?;
        }

        self.serde_rename_all = None;
        self.structs.push(rs);
        Ok(())
    }

    fn parse_field(&mut self, rs: &mut RustStruct, f: &syn::Field) -> std::io::Result<()> {
        let mut ty: &str = &type_as_string(&f.ty);
        let is_optional = ty.starts_with(OPTION_PREFIX);
        if is_optional {
            ty = remove_prefix_suffix(&ty, OPTION_PREFIX, OPTION_SUFFIX);
        }

        let is_vec = ty.starts_with(VEC_PREFIX);
        if is_vec {
            ty = &remove_prefix_suffix(&ty, VEC_PREFIX, VEC_SUFFIX);
        }

        let mut rf = RustField {
            id: get_ident(f.ident.as_ref(), &f.attrs, &self.serde_rename_all),
            ty: ty.to_owned(),
            is_optional,
            is_vec,
            comments: Vec::new(),
        };
        self.parse_comment_attrs(&mut rf.comments, &f.attrs)?;

        rs.fields.push(rf);
        Ok(())
    }

    fn parse_enum(&mut self, e: &syn::ItemEnum) -> std::io::Result<()> {
        if self.params.use_marker && !has_typeshare_marker(&e.attrs) {
            return Ok(());
        }

        self.serde_rename_all = serde_rename_all(&e.attrs);
        if is_const_enum(e) {
            self.parse_const_enum(e)?;
        } else {
            self.parse_algebraic_enum(e)?;
        }
        self.serde_rename_all = None;
        Ok(())
    }

    fn parse_const_enum(&mut self, e: &syn::ItemEnum) -> std::io::Result<()> {
        let mut re = RustConstEnum {
            id: get_ident(Some(&e.ident), &e.attrs, &self.serde_rename_all),
            comments: Vec::new(),
            ty: get_const_enum_type(e).clone(),
            consts: Vec::new(),
        };
        self.parse_comment_attrs(&mut re.comments, &e.attrs)?;

        for v in e.variants.iter() {
            let mut rc = RustConst {
                id: get_ident(Some(&v.ident), &v.attrs, &self.serde_rename_all),
                value: get_discriminant(&v),
                comments: Vec::new(),
            };

            self.parse_comment_attrs(&mut rc.comments, &v.attrs)?;
            re.consts.push(rc);
        }

        self.const_enums.push(re);

        Ok(())
    }

    fn parse_algebraic_enum(&mut self, _e: &syn::ItemEnum) -> std::io::Result<()> {
        Ok(())
    }

    //----

    fn parse_comment_attrs(&mut self, comments: &mut Vec<String>, attrs: &[syn::Attribute]) -> std::io::Result<()> {
        for a in attrs.iter() {
            let s = a.tts.to_string();
            if s.starts_with(COMMENT_PREFIX) {
                comments.push(remove_prefix_suffix(&s, COMMENT_PREFIX, COMMENT_SUFFIX).to_owned());
            }
        }

        Ok(())
    }
}

fn get_discriminant(v: &syn::Variant) -> Option<syn::ExprLit> {
    if let Some(d) = &v.discriminant {
        match &d.1 {
            syn::Expr::Lit(l) => {
                return Some(l.clone());
            }
            _ => {
                panic!("unexpected expr");
            }
        }
    }

    None
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

fn get_const_enum_type(e: &syn::ItemEnum) -> Option<syn::Lit> {
    if is_const_enum(e) {
        if let Some(discriminant) = &e.variants.first().unwrap().into_value().discriminant {
            return match &discriminant.1 {
                syn::Expr::Lit(expr_lit) => Some(expr_lit.lit.clone()),
                _ => None,
            };
        }
    }
    None
}

fn type_as_string(ty: &syn::Type) -> String {
    use quote::ToTokens;

    let mut tokens = proc_macro2::TokenStream::new();
    ty.to_tokens(&mut tokens);
    tokens.to_string()
}

fn get_ident(ident: Option<&proc_macro2::Ident>, attrs: &[syn::Attribute], rename_all: &Option<String>) -> Id {
    let original = ident.map_or("???".to_string(), |id| id.to_string().replace("r#", ""));
    let mut renamed = match rename_all {
        None => original.clone(),
        Some(value) => match value.as_str() {
            "lowercase" => original.to_lowercase(),
            "UPPERCASE" => original.to_uppercase(),
            "PascalCase" => original.to_pascal_case(),
            "camelCase" => original.to_camel_case(),
            "snake_case" => original.to_snake_case(),
            "SCREAMING_SNAKE_CASE" => original.to_screaming_snake_case(),
            "kebab-case" => original.to_kebab_case(),
            "SCREAMING-KEBAB-CASE" => original.to_kebab_case(),
            _ => original.clone(),
        },
    };

    if let Some(s) = serde_rename(attrs) {
        renamed = s;
    }

    Id { original, renamed }
}

fn serde_rename(attrs: &[syn::Attribute]) -> Option<String> {
    const PREFIX: &str = r##"rename = ""##;
    const SUFFIX: &str = r##"""##;
    attr_value(attrs, PREFIX, SUFFIX)
}

fn serde_rename_all(attrs: &[syn::Attribute]) -> Option<String> {
    const PREFIX: &str = r##"rename_all = ""##;
    const SUFFIX: &str = r##"""##;
    attr_value(attrs, PREFIX, SUFFIX)
}

fn has_typeshare_marker(attrs: &[syn::Attribute]) -> bool {
    const TYPESHARE_MARKER: &str = "typeshare";
    let typeshare_ident = Ident::new(TYPESHARE_MARKER, Span::call_site());
    for a in attrs {
        if let Some(segment) = a.path.segments.iter().next() {
            if segment.ident == typeshare_ident {
                return true;
            }
        }
    }

    false
}

/*
    Process attributes and return value of the matching attribute, if found.
    ```
    [
    Attribute
        {
            pound_token: Pound,
            style: Outer,
            bracket_token: Bracket,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment { ident: Ident(doc), arguments: None }
                ]
            },
            tts: TokenStream [
                Punct { op: '=', spacing: Alone },
                Literal { lit: " This is a comment." }]
        },

    Attribute
        {
            pound_token: Pound,
            style: Outer,
            bracket_token: Bracket,
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment { ident: Ident(serde), arguments: None }
                ]
            }
            tts: TokenStream [
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [
                        Ident { sym: default },
                        Punct { op: ',', spacing: Alone },
                        Ident { sym: rename_all },
                        Punct { op: '=', spacing: Alone },
                        Literal { lit: "camelCase" }
                    ]
                }
            ]
        }
    ]
    ```
*/
fn attr_value(attrs: &[syn::Attribute], prefix: &'static str, suffix: &'static str) -> Option<String> {
    for a in attrs {
        if let Some(segment) = a.path.segments.iter().next() {
            if segment.ident != Ident::new("serde", Span::call_site()) {
                continue;
            }

            let attr_as_string = a.tts.to_string();
            let values = parse_attr(&attr_as_string)?;

            for v in values {
                if v.starts_with(prefix) && v.ends_with(suffix) {
                    return Some(remove_prefix_suffix(&v, prefix, suffix).to_string());
                }
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
