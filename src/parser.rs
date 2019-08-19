use syn;

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
    pub comments: Vec<String>,
    pub typeshare_attrs: HashMap<String, String>,
    pub fields: Vec<RustField>,
}

/// Rust field defintion.
pub struct RustField {
    pub id: Id,
    pub comments: Vec<String>,
    pub ty: String,
    pub is_optional: bool,
    pub is_vec: bool,
}

/// Definition of constant enums.
pub struct RustConstEnum {
    pub id: Id,
    pub comments: Vec<String>,
    pub typeshare_attrs: HashMap<String, String>,
    pub ty: Option<syn::Lit>,
    pub consts: Vec<RustConst>,
}

/// Single constant in an enum.
pub struct RustConst {
    pub id: Id,
    pub comments: Vec<String>,
    pub value: Option<syn::ExprLit>,
}

pub struct RustAttrs {
    attrs: HashMap<String, RustAttrValues>,
}

pub struct RustAttrValues {
    values: HashMap<String, String>,
}

impl RustAttrs {
    fn new() -> Self {
        Self {
            attrs: HashMap::new(),
        }
    }

    fn add_values(s: &str) {
        if let Some(segment) = a.path.segments.iter().next() {
            if segment.ident != Ident::new("serde", Span::call_site()) {
                continue;
            }

    }
}
