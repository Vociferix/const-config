#![allow(dead_code)]
#![allow(unused)]

pub(crate) mod phf;

#[cfg(feature = "toml")]
pub mod toml;

#[cfg(feature = "json")]
pub mod json;

#[cfg(feature = "yaml")]
pub mod yaml;

#[derive(Debug, Clone)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    UInt(u64),
    Int(i64),
    Float(f64),
    Time(Time),
    DateTime(DateTime),
    Str(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
    Map(Vec<(Value, Value)>),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub nanosecond: u32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DateTime {
    pub date: Date,
    pub time: Option<OffsetTime>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct OffsetTime {
    pub time: Time,
    pub offset_minutes: Option<i16>,
}

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

pub(crate) fn generate<O>(value: Value, mut output: O) -> std::io::Result<()>
where
    O: std::io::Write,
{
    write!(output, "const {{\n")?;
    generate_impl(value, &mut output, 1)?;
    write!(output, "}}")
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
                write!(output, "}},\n")?;
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
        Value::Array(value) => {
            if value.is_empty() {
                write!(
                    output,
                    "::const_config::Value::<'static>::Array(const {{ &[] }})"
                )
            } else {
                write!(
                    output,
                    "::const_config::Value::<'static>::Array(const {{ &[\n"
                )?;

                for elem in value {
                    do_indent(output, indent + 1)?;
                    generate_impl(elem, output, indent + 1)?;
                    write!(output, ",\n")?;
                }

                do_indent(output, indent)?;
                write!(output, "] }})")
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
                write!(output, "const {{ &[\n")?;
                for entry in value {
                    do_indent(output, indent + 2)?;
                    write!(output, "(")?;
                    write_str(output, &entry.0)?;
                    write!(output, ", ")?;
                    generate_impl(entry.1, output, indent + 2)?;
                    write!(output, "),\n")?;
                }
                do_indent(output, indent + 1)?;
                write!(output, "] }},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{ &[\n")?;
                for param in &phf.params {
                    do_indent(output, indent + 2)?;
                    write!(output, "{},\n", *param)?;
                }
                do_indent(output, indent + 1)?;
                write!(output, "] }},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{ &[\n")?;
                for value in &phf.values {
                    do_indent(output, indent + 2)?;
                    write!(output, "{},\n", *value)?;
                }
                do_indent(output, indent + 1)?;
                write!(output, "] }},\n")?;

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
                write!(output, "const {{ &[\n")?;
                for entry in value {
                    do_indent(output, indent + 2)?;
                    write!(output, "(\n")?;
                    do_indent(output, indent + 3)?;
                    generate_impl(entry.0, output, indent + 3)?;
                    write!(output, ",\n")?;
                    do_indent(output, indent + 3)?;
                    generate_impl(entry.1, output, indent + 2)?;
                    write!(output, ",\n")?;
                    do_indent(output, indent + 2)?;
                    write!(output, "),\n")?;
                }
                do_indent(output, indent + 1)?;
                write!(output, "] }},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{ &[\n")?;
                for param in &phf.params {
                    do_indent(output, indent + 2)?;
                    write!(output, "{},\n", *param)?;
                }
                do_indent(output, indent + 1)?;
                write!(output, "] }},\n")?;

                do_indent(output, indent + 1)?;
                write!(output, "const {{ &[\n")?;
                for value in &phf.values {
                    do_indent(output, indent + 2)?;
                    write!(output, "{},\n", *value)?;
                }
                do_indent(output, indent + 1)?;
                write!(output, "] }},\n")?;

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
