use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input configuration or data file. Defaults to stdin.
    #[arg(short, long, value_name = "FILE")]
    infile: Option<std::path::PathBuf>,

    /// Output file for generated Rust code. Defaults to stdout.
    #[arg(short, long, value_name = "FILE")]
    outfile: Option<std::path::PathBuf>,

    /// Format of input file.
    #[arg(short, long, value_enum)]
    format: Format,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "yaml")]
    Yaml,
    #[cfg(feature = "ron")]
    Ron,
    #[cfg(feature = "json5")]
    Json5,
    #[cfg(feature = "s-expr")]
    SExpr,
    #[cfg(feature = "hjson")]
    Hjson,
    #[cfg(feature = "csv")]
    Csv,
    #[cfg(feature = "cbor")]
    Cbor,
    #[cfg(feature = "msgpack")]
    Msgpack,
    #[cfg(feature = "pickle")]
    Pickle,
    #[cfg(feature = "bson")]
    Bson,
    #[cfg(feature = "flexbuffers")]
    Flexbuffers,
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Cli::parse();

    let input: Box<dyn std::io::Read> = if let Some(infile) = args.infile.as_ref() {
        Box::new(std::io::BufReader::new(std::fs::File::open(infile)?))
    } else {
        Box::new(std::io::stdin().lock())
    };

    let output: Box<dyn std::io::Write> = if let Some(outfile) = args.outfile.as_ref() {
        Box::new(std::io::BufWriter::new(std::fs::File::create(outfile)?))
    } else {
        Box::new(std::io::stdout().lock())
    };

    match args.format {
        #[cfg(feature = "toml")]
        Format::Toml => const_config_gen::toml::generate(input, output)?,
        #[cfg(feature = "json")]
        Format::Json => const_config_gen::json::generate(input, output)?,
        #[cfg(feature = "yaml")]
        Format::Yaml => const_config_gen::yaml::generate(input, output)?,
        #[cfg(feature = "ron")]
        Format::Ron => const_config_gen::ron::generate(input, output)?,
        #[cfg(feature = "json5")]
        Format::Json5 => const_config_gen::json5::generate(input, output)?,
        #[cfg(feature = "s-expr")]
        Format::SExpr => const_config_gen::s_expr::generate(input, output)?,
        #[cfg(feature = "hjson")]
        Format::Hjson => const_config_gen::hjson::generate(input, output)?,
        #[cfg(feature = "csv")]
        Format::Csv => const_config_gen::csv::generate(input, output)?,
        #[cfg(feature = "cbor")]
        Format::Cbor => const_config_gen::cbor::generate(input, output)?,
        #[cfg(feature = "msgpack")]
        Format::Msgpack => const_config_gen::msgpack::generate(input, output)?,
        #[cfg(feature = "pickle")]
        Format::Pickle => const_config_gen::pickle::generate(input, output)?,
        #[cfg(feature = "bson")]
        Format::Bson => const_config_gen::bson::generate(input, output)?,
        #[cfg(feature = "flexbuffers")]
        Format::Flexbuffers => const_config_gen::flexbuffers::generate(input, output)?,
    }

    Ok(())
}
