use crate::Value;
use std::io::{Read, Write};
use yaml_rust2::yaml::{YAMLDecodingTrap, Yaml, YamlDecoder};

#[doc(inline)]
pub use yaml_rust2::yaml::LoadError as Error;

type Result<T> = std::result::Result<T, Error>;

pub fn generate<I, O>(input: I, output: O) -> Result<()>
where
    I: Read,
    O: Write,
{
    let yaml = YamlDecoder::read(input)
        .encoding_trap(YAMLDecodingTrap::Strict)
        .decode()?;
    let value = if yaml.is_empty() {
        Value::Object(Vec::new())
    } else if yaml.len() == 1 {
        transform(&yaml[0])
    } else {
        let mut array = Vec::with_capacity(yaml.len());
        for doc in &yaml {
            array.push(transform(doc));
        }
        Value::Array(array)
    };
    crate::generate(value, output)?;
    Ok(())
}

fn transform(yaml: &Yaml) -> Value {
    match yaml {
        Yaml::Real(_) => Value::Float(yaml.as_f64().unwrap()),
        Yaml::Integer(value) => Value::Int(*value),
        Yaml::String(value) => Value::Str(value.clone()),
        Yaml::Boolean(value) => Value::Bool(*value),
        Yaml::Array(value) => {
            let mut array = Vec::with_capacity(value.len());
            for elem in value {
                array.push(transform(elem));
            }
            Value::Array(array)
        }
        Yaml::Hash(value) => {
            let mut is_obj = true;
            for (key, _) in value {
                if !matches!(key, Yaml::String(_)) {
                    is_obj = false;
                    break;
                }
            }

            if is_obj {
                let mut obj = Vec::with_capacity(value.len());
                for (key, value) in value {
                    let Yaml::String(key) = key else {
                        unreachable!();
                    };
                    obj.push((key.clone(), transform(value)));
                }
                Value::Object(obj)
            } else {
                let mut map = Vec::with_capacity(value.len());
                for (key, value) in value {
                    map.push((transform(key), transform(value)));
                }
                Value::Map(map)
            }
        }
        Yaml::Alias(_) => panic!("YAML aliases are not supported"),
        Yaml::Null => Value::Null,
        Yaml::BadValue => panic!("Invalid YAML"),
    }
}
