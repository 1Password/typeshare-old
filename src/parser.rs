use syn;

use proc_macro2::{TokenStream, TokenTree};
use std::collections::HashMap;

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
    pub comments: Vec<String>,
    pub attrs: RustAttrs,
    pub fields: Vec<RustField>,
}

impl RustStruct {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            comments: Vec::new(),
            attrs: RustAttrs::new(),
            fields: Vec::new(),
        }
    }
}

/// Rust field defintion.
pub struct RustField {
    pub id: Id,
    pub comments: Vec<String>,
    pub attrs: RustAttrs,
    pub ty: String,
    pub is_optional: bool,
    pub is_vec: bool,
}

impl RustField {
    pub fn new(id: Id, ty: &str, is_optional: bool, is_vec: bool) -> Self {
        Self {
            id,
            comments: Vec::new(),
            attrs: RustAttrs::new(),
            ty: ty.to_owned(),
            is_optional,
            is_vec,
        }
    }
}

/// Definition of constant enums.
pub struct RustConstEnum {
    pub id: Id,
    pub comments: Vec<String>,
    pub attrs: RustAttrs,
    pub ty: Option<syn::Lit>,
    pub consts: Vec<RustConst>,
}

/// Single constant in an enum.
pub struct RustConst {
    pub id: Id,
    pub comments: Vec<String>,
    pub attrs: RustAttrs,
    pub value: Option<syn::ExprLit>,
}

pub struct RustAttrs {
    attrs: HashMap<String, RustAttrValues>,
}

impl RustAttrs {
    pub fn new() -> Self {
        Self { attrs: HashMap::new() }
    }

    pub fn comments() -> Option<Vec<String>> {
        None
    }

    pub fn get_value(macro_name: &str, attr_name: &str) -> Option<String> {
        None
    }

    pub fn parse(&mut self, attrs: &[syn::Attribute]) {
        for a in attrs {
            let meta = a.parse_meta();
            println!("{:?}", meta);
            // if let Some(segment) = a.path.segments.iter().next() {
            //     let key = segment.ident.to_string();
            //     println!("key: {}", key);
            //     if let Some(v) = self.attrs.get_mut(&key) {
            //         v.parse(&mut a.tts.into_iter());
            //     } else {
            //         let mut v = RustAttrValues::new();
            //         v.parse(&mut a.tts.into_iter());
            //         self.attrs.insert(key, v);
            //     }
            // } else {
            // }
        }
    }
}

pub struct RustAttrValues {
    list: Vec<String>,
    map: HashMap<String, String>,
}

impl RustAttrValues {
    fn new() -> Self {
        Self {
            list: Vec::new(),
            map: HashMap::new(),
        }
    }

    fn parse(&mut self, iter: &mut dyn Iterator<Item = TokenTree>) {
        if let Some(token) = iter.next() {
            match token {
                TokenTree::Punct(p) => {
                    println!("Punct {:?}", p);
                }
                TokenTree::Group(g) => {
                    println!("Group: {:?}", g);
                }
                TokenTree::Ident(g) => {
                    println!("Ident: {:?}", g);
                }
                TokenTree::Literal(g) => {
                    println!("Literal: {:?}", g);
                }
            }
        }
    }
}
