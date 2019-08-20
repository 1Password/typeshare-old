use clap::{App, Arg};
use language::{Generator, Language};

mod java;
mod language;
mod swift;
mod typescript;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const ARG_TYPE: &str = "TYPE";
const ARG_MARKER: &str = "MARKER";
const ARG_SWIFT_PREFIX: &str = "SWIFTPREFIX";
const ARG_JAVA_PACKAGE: &str = "JAVAPACKAGE";

fn main() {
    let options = App::new("typeshare")
        .version(VERSION)
        .arg(
            Arg::with_name(ARG_TYPE)
                .short("t")
                .long("type")
                .help("Output type: java|swift|typescript")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_MARKER)
                .short("m")
                .long("use-marker")
                .help("Only process structs and enums marked with #[typeshare] attribute")
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_SWIFT_PREFIX)
                .short("sp")
                .long("swift-prefix")
                .help("Prefix for generated Swift types")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_JAVA_PACKAGE)
                .short("jp")
                .long("java-package")
                .help("JAVA package name")
                .takes_value(true)
                .required(false),
        )
        .arg(Arg::with_name("input.rs").help("Sets the input file to use").required(true).index(1))
        .get_matches();

    let filename = options.value_of("input.rs").unwrap();

    let mut lang: Box<dyn Language> = match options.value_of(ARG_TYPE) {
        Some("java") => Box::new(java::Java {}),
        Some("swift") => Box::new(swift::Swift::new()),
        Some("ts") => Box::new(typescript::TypeScript {}),
        Some("typescript") => Box::new(typescript::TypeScript {}),
        _ => Box::new(typescript::TypeScript {}),
    };

    let params = language::Params {
        use_marker: options.is_present(ARG_MARKER),
        swift_prefix: options.value_of(ARG_SWIFT_PREFIX).unwrap_or("").to_string(),
        java_package: options.value_of(ARG_JAVA_PACKAGE).unwrap_or("").to_string(),
    };

    let mut generator = Generator::new(lang.as_mut(), params);

    let mut out = std::io::stdout();
    generator.process_file(filename, &mut out).expect("failed to process");
}
