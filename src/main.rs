use clap::{App, Arg};

mod language;
mod typescript;
mod swift;
use language::Language;

fn main() {
    let matches = App::new("rust_ftw_gen")
        .version("v0.0.1")
        .arg(
            Arg::with_name("TYPE")
                .short("t")
                .long("type")
                .help("Output type: java|swift|ts")
                .required(false)
        )
        .arg(
            Arg::with_name("input.rs")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("input.rs").unwrap();
    let mut out = std::io::stdout();

    let mut lang: Box<dyn Language> = match matches.value_of("TYPE") {
        Some("java") => Box::new(typescript::TypeScript{}),
        Some("swift") => Box::new(swift::Swift::new()),
        Some("ts") => Box::new(typescript::TypeScript{}),
        _ => Box::new(swift::Swift::new()),
    };

    lang.process(&mut out, filename).expect("failed to process");
}
