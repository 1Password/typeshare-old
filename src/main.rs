use clap::{App, Arg};

mod language;
mod swift;
mod typescript;
use language::{Generator, GeneratorParams, Language};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let options = App::new("typeshare")
        .version(VERSION)
        .arg(
            Arg::with_name("TYPE")
                .short("t")
                .long("type")
                .help("Output type: java|swift|typescript")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("MARKER")
                .short("m")
                .long("use-marker")
                .help("Only process structs and enums marked with #[typeshare] attribute")
                .takes_value(false)
                .required(false),
        )
        .arg(Arg::with_name("input.rs").help("Sets the input file to use").required(true).index(1))
        .get_matches();

    let filename = options.value_of("input.rs").unwrap();

    let mut lang: Box<dyn Language> = match options.value_of("TYPE") {
        Some("java") => Box::new(typescript::TypeScript {}),
        Some("swift") => Box::new(swift::Swift::new()),
        Some("ts") => Box::new(typescript::TypeScript {}),
        Some("typescript") => Box::new(typescript::TypeScript {}),
        _ => Box::new(typescript::TypeScript {}),
    };

    let params = GeneratorParams {
        use_marker: options.is_present("MARKER"),
    };

    let mut generator = Generator::new(lang.as_mut(), params);

    let mut out = std::io::stdout();
    generator.process_file(filename, &mut out).expect("failed to process");
}
