use clap::{App, Arg};
use language::{Generator, Language};

mod java;
mod language;
mod swift;
mod typescript;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("typeshare")
        .version(VERSION)
        .arg(
            Arg::with_name("TYPE")
                .short("t")
                .long("type")
                .help("Output type: java|swift|ts")
                .takes_value(true)
                .required(false),
        )
        .arg(Arg::with_name("input.rs").help("Sets the input file to use").required(true).index(1))
        .get_matches();

    let filename = matches.value_of("input.rs").unwrap();
    let mut out = std::io::stdout();

    let mut lang: Box<dyn Language> = match matches.value_of("TYPE") {
        Some("java") => Box::new(java::Java {}),
        Some("swift") => Box::new(swift::Swift::new()),
        Some("ts") => Box::new(typescript::TypeScript {}),
        _ => Box::new(typescript::TypeScript {}),
    };

    let mut generator = Generator::new(lang.as_mut(), &mut out);
    generator.process_file(filename).expect("failed to process");
}
