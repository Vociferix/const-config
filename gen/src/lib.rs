#![allow(dead_code)]
#![allow(unused)]

pub(crate) mod phf;
mod value;

#[cfg(feature = "toml")]
pub mod toml;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "yaml")]
pub mod yaml;

#[cfg(feature = "ron")]
pub mod ron;

#[cfg(feature = "json5")]
pub mod json5;

#[cfg(feature = "s-expr")]
pub mod s_expr;

#[cfg(feature = "hjson")]
pub mod hjson;

#[cfg(feature = "csv")]
pub mod csv;

#[cfg(feature = "cbor")]
pub mod cbor;

#[cfg(feature = "msgpack")]
pub mod msgpack;

#[cfg(feature = "pickle")]
pub mod pickle;

#[cfg(feature = "bson")]
pub mod bson;

#[cfg(feature = "flexbuffers")]
pub mod flexbuffers;

pub use value::*;

#[cfg(feature = "toml")]
pub fn generate_from_toml<I, O>(input_path: I, output_path: O) -> Result<(), toml::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, toml::generate)
}

#[cfg(feature = "json")]
pub fn generate_from_json<I, O>(input_path: I, output_path: O) -> Result<(), json::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, json::generate)
}

#[cfg(feature = "yaml")]
pub fn generate_from_yaml<I, O>(input_path: I, output_path: O) -> Result<(), yaml::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, yaml::generate)
}

#[cfg(feature = "ron")]
pub fn generate_from_ron<I, O>(input_path: I, output_path: O) -> Result<(), ron::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, ron::generate)
}

#[cfg(feature = "json5")]
pub fn generate_from_json5<I, O>(input_path: I, output_path: O) -> Result<(), json5::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, json5::generate)
}

#[cfg(feature = "s-expr")]
pub fn generate_from_s_expr<I, O>(input_path: I, output_path: O) -> Result<(), s_expr::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, s_expr::generate)
}

#[cfg(feature = "hjson")]
pub fn generate_from_hjson<I, O>(input_path: I, output_path: O) -> Result<(), hjson::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, hjson::generate)
}

#[cfg(feature = "csv")]
pub fn generate_from_csv<I, O>(input_path: I, output_path: O) -> Result<(), csv::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, csv::generate)
}

#[cfg(feature = "cbor")]
pub fn generate_from_cbor<I, O>(input_path: I, output_path: O) -> Result<(), cbor::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, cbor::generate)
}

#[cfg(feature = "msgpack")]
pub fn generate_from_msgpack<I, O>(input_path: I, output_path: O) -> Result<(), msgpack::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, msgpack::generate)
}

#[cfg(feature = "pickle")]
pub fn generate_from_pickle<I, O>(input_path: I, output_path: O) -> Result<(), pickle::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, pickle::generate)
}

#[cfg(feature = "bson")]
pub fn generate_from_bson<I, O>(input_path: I, output_path: O) -> Result<(), bson::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, bson::generate)
}

#[cfg(feature = "flexbuffers")]
pub fn generate_from_flexbuffers<I, O>(
    input_path: I,
    output_path: O,
) -> Result<(), flexbuffers::Error>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    gen_fs(input_path, output_path, flexbuffers::generate)
}

pub fn generate<O>(value: Value, mut output: O) -> std::io::Result<()>
where
    O: std::io::Write,
{
    write!(output, "const {{\n")?;
    do_indent(&mut output, 1)?;
    generate_impl(value, &mut output, 1)?;
    write!(output, "\n}}")
}

fn gen_fs<I, O, F, E>(input_path: I, output_path: O, f: F) -> Result<(), E>
where
    I: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
    F: FnOnce(
        std::io::BufReader<std::fs::File>,
        std::io::BufWriter<std::fs::File>,
    ) -> Result<(), E>,
    E: From<std::io::Error>,
{
    let input = std::io::BufReader::new(std::fs::File::open(input_path)?);
    let output = std::io::BufWriter::new(std::fs::File::create(output_path)?);
    f(input, output)
}

fn do_indent<O>(output: &mut O, indent: usize) -> std::io::Result<()>
where
    O: std::io::Write,
{
    for _ in 0..indent {
        write!(output, "    ")?;
    }
    Ok(())
}

fn generate_impl<O>(value: Value, output: &mut O, indent: usize) -> std::io::Result<()>
where
    O: std::io::Write,
{
    match value {
        Value::Null => write!(output, "::const_config::Value::<'static>::Null"),
        Value::Bool(value) => write!(output, "::const_config::Value::<'static>::Bool({})", value),
        Value::UInt(value) => write!(
            output,
            "::const_config::Value::<'static>::Number(::const_config::Number::UInt({}))",
            value
        ),
        Value::Int(value) => write!(
            output,
            "::const_config::Value::<'static>::Number(::const_config::Number::Int({}))",
            value
        ),
        Value::Float(value) => write!(
            output,
            "::const_config::Value::<'static>::Number(::const_config::Number::Float({:.17}))",
            value
        ),
        Value::Time(value) => {
            write!(
                output,
                "::const_config::Value::<'static>::Time(::const_config::Time {{\n"
            )?;
            do_indent(output, indent + 1)?;
            write!(output, "hour: {},\n", value.hour)?;
            do_indent(output, indent + 1)?;
            write!(output, "minute: {},\n", value.minute)?;
            do_indent(output, indent + 1)?;
            write!(output, "second: {},\n", value.second)?;
            do_indent(output, indent + 1)?;
            write!(output, "nanosecond: {},\n", value.nanosecond)?;
            do_indent(output, indent)?;
            write!(output, "}})")
        }
        Value::DateTime(value) => {
            write!(
                output,
                "::const_config::Value::<'static>::DateTime(::const_config::DateTime {{\n"
            )?;
            do_indent(output, indent + 1)?;
            write!(output, "date: ::const_config::Date {{\n")?;
            do_indent(output, indent + 2)?;
            write!(output, "year: {},\n", value.date.year)?;
            do_indent(output, indent + 2)?;
            write!(output, "month: {},\n", value.date.month)?;
            do_indent(output, indent + 2)?;
            write!(output, "day: {},\n", value.date.day)?;
            do_indent(output, indent + 1)?;
            write!(output, "}},\n")?;
            do_indent(output, indent + 1)?;
            if let Some(time) = value.time {
                write!(
                    output,
                    "time: ::core::option::Option::Some(::const_config::OffsetTime {{\n"
                )?;
                do_indent(output, indent + 2)?;
                write!(output, "time: ::const_config::Time {{\n")?;
                do_indent(output, indent + 3)?;
                write!(output, "hour: {},\n", time.time.hour)?;
                do_indent(output, indent + 3)?;
                write!(output, "minute: {},\n", time.time.minute)?;
                do_indent(output, indent + 3)?;
                write!(output, "second: {},\n", time.time.second)?;
                do_indent(output, indent + 3)?;
                write!(output, "nanosecond: {},\n", time.time.nanosecond)?;
                do_indent(output, indent + 2)?;
                write!(output, "}},\n")?;
                do_indent(output, indent + 2)?;
                if let Some(offset) = time.offset_minutes {
                    write!(
                        output,
                        "offset_minutes: ::core::option::Option::Some({}),\n",
                        offset
                    )?;
                } else {
                    write!(output, "offset_minutes: ::core::option::Option::None,\n")?;
                }
                do_indent(output, indent + 1)?;
                write!(output, "}}),\n")?;
            } else {
                write!(output, "time: ::core::option::Option::None,\n")?;
            }
            do_indent(output, indent)?;
            write!(output, "}})")
        }
        Value::Str(value) => {
            write!(output, "::const_config::Value::<'static>::Str(")?;
            write_str(output, &value)?;
            write!(output, ")")
        }
        Value::Bytes(value) => {
            write!(output, "::const_config::Value::<'static>::Bytes(")?;
            write_bytes(output, &value)?;
            write!(output, ")")
        }
        Value::Array(value) => {
            if value.is_empty() {
                write!(
                    output,
                    "::const_config::Value::<'static>::Array(const {{ &[] }})"
                )
            } else {
                write!(output, "::const_config::Value::<'static>::Array(const {{\n")?;
                do_indent(output, indent + 1)?;
                write!(output, "&[\n")?;

                for elem in value {
                    do_indent(output, indent + 2)?;
                    generate_impl(elem, output, indent + 2)?;
                    write!(output, ",\n")?;
                }

                do_indent(output, indent + 1)?;
                write!(output, "]\n")?;
                do_indent(output, indent)?;
                write!(output, "}})")
            }
        }
        Value::Object(value) => {
            if value.is_empty() {
                write!(
                    output,
                    "::const_config::Value::<'static>::Object(::const_config::Object::<'static>::new(const {{ &[] }}, const {{ &[] }}, const {{ &[] }}))"
                )
            } else {
                let phf = phf::Phf::build_object(&value);
                write!(
                    output,
                    "::const_config::Value::<'static>::Object(::const_config::Object::<'static>::new(\n"
                )?;
                do_indent(output, indent + 1)?;
                write!(output, "const {{\n")?;
                do_indent(output, indent + 2)?;
                write!(output, "&[\n")?;
                for entry in value {
                    do_indent(output, indent + 3)?;
                    write!(output, "(")?;
                    write_str(output, &entry.0)?;
                    write!(output, ", ")?;
                    generate_impl(entry.1, output, indent + 3)?;
                    write!(output, "),\n")?;
                }
                do_indent(output, indent + 2)?;
                write!(output, "]\n")?;
                do_indent(output, indent + 1)?;
                write!(output, "}},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{\n")?;
                do_indent(output, indent + 2)?;
                write!(output, "&[\n")?;
                for param in &phf.params {
                    do_indent(output, indent + 3)?;
                    write!(output, "{},\n", *param)?;
                }
                do_indent(output, indent + 2)?;
                write!(output, "]\n")?;
                do_indent(output, indent + 1)?;
                write!(output, "}},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{\n")?;
                do_indent(output, indent + 2)?;
                write!(output, "&[\n")?;
                for value in &phf.values {
                    do_indent(output, indent + 3)?;
                    write!(output, "{},\n", *value)?;
                }
                do_indent(output, indent + 2)?;
                write!(output, "]\n")?;
                do_indent(output, indent + 1)?;
                write!(output, "}},\n")?;

                do_indent(output, indent)?;
                write!(output, "))")
            }
        }
        Value::Map(value) => {
            if value.is_empty() {
                write!(
                    output,
                    "::const_config::Value::<'static>::Map(::const_config::Map::<'static>::new(const {{ &[] }}, const {{ &[] }}, const {{ &[] }}))"
                )
            } else {
                let phf = phf::Phf::build_map(&value);
                write!(
                    output,
                    "::const_config::Value::<'static>::Map(::const_config::Map::<'static>::new(\n"
                )?;
                do_indent(output, indent + 1)?;
                write!(output, "const {{\n")?;
                do_indent(output, indent + 2)?;
                write!(output, "&[\n")?;
                for entry in value {
                    do_indent(output, indent + 3)?;
                    write!(output, "(\n")?;
                    do_indent(output, indent + 4)?;
                    generate_impl(entry.0, output, indent + 4)?;
                    write!(output, ",\n")?;
                    do_indent(output, indent + 4)?;
                    generate_impl(entry.1, output, indent + 4)?;
                    write!(output, ",\n")?;
                    do_indent(output, indent + 3)?;
                    write!(output, "),\n")?;
                }
                do_indent(output, indent + 2)?;
                write!(output, "]\n")?;
                do_indent(output, indent + 1)?;
                write!(output, "}},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{\n")?;
                do_indent(output, indent + 2)?;
                write!(output, "&[\n")?;
                for param in &phf.params {
                    do_indent(output, indent + 3)?;
                    write!(output, "{},\n", *param)?;
                }
                do_indent(output, indent + 2)?;
                write!(output, "]\n")?;
                do_indent(output, indent + 1)?;
                write!(output, "}},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{\n")?;
                do_indent(output, indent + 2)?;
                write!(output, "&[\n")?;
                for value in &phf.values {
                    do_indent(output, indent + 3)?;
                    write!(output, "{},\n", *value)?;
                }
                do_indent(output, indent + 2)?;
                write!(output, "]\n")?;
                do_indent(output, indent + 1)?;
                write!(output, "}},\n")?;

                do_indent(output, indent)?;
                write!(output, "))")
            }
        }
    }
}

fn write_str<O>(output: &mut O, s: &str) -> std::io::Result<()>
where
    O: std::io::Write,
{
    write!(output, "\"")?;
    for ch in s.chars() {
        match ch {
            '\0' => {
                write!(output, "\\0")?;
            }
            '\t' => {
                write!(output, "\\t")?;
            }
            '\n' => {
                write!(output, "\\n")?;
            }
            '\r' => {
                write!(output, "\\r")?;
            }
            '"' => {
                write!(output, "\\\"")?;
            }
            '\\' => {
                write!(output, "\\\\")?;
            }
            _ => {
                if ch.is_ascii_control() {
                    write!(output, "\\x{:02x}", ch as u32)?;
                } else if ch.is_control() {
                    write!(output, "\\u{{{:04x}}}", ch as u32)?;
                } else {
                    write!(output, "{}", ch)?;
                }
            }
        }
    }
    write!(output, "\"")
}

fn write_bytes<O>(output: &mut O, b: &[u8]) -> std::io::Result<()>
where
    O: std::io::Write,
{
    write!(output, "b\"")?;
    for byte in b {
        match *byte {
            b'\0' => {
                write!(output, "\\0")?;
            }
            b'\t' => {
                write!(output, "\\t")?;
            }
            b'\n' => {
                write!(output, "\\n")?;
            }
            b'\r' => {
                write!(output, "\\r")?;
            }
            b'"' => {
                write!(output, "\\\"")?;
            }
            b'\\' => {
                write!(output, "\\\\")?;
            }
            _ => {
                if byte.is_ascii_control() || *byte >= 0x80 {
                    write!(output, "\\x{:02x}", *byte)?;
                } else if let Some(ch) = char::from_u32(*byte as u32) {
                    write!(output, "{}", ch)?;
                } else {
                    write!(output, "\\x{:02x}", *byte)?;
                }
            }
        }
    }
    write!(output, "\"")
}
