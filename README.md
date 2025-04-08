# `const-config`

`const-config` is a Rust crate for compiled-in application configuration.

This crate takes configuration defined in TOML, JSON, or YAML formats and
produces a `const` object that a crate can query even in `const` contexts.

This can be used to simplify tuning your application or baking in data.

## Example
Given a TOML config file `app-config.toml` in the root of your crate:
```toml
# app-config.toml

[general]
trace-logging = false
production = true

[networking]
allow-insecure = false
default-port = 443
```

You can bake this config into your application like so:
```rust
use const_config::{Object, include_toml};

const APP_CFG: Object<'static> = include_toml!("app-config.toml").as_object();

fn main() {
    if const { APP_CFG.get("general").as_object().get("production").as_bool() } {
        println!("production mode");
    } else {
        println!("development mode");
    }
}
```

## `serde`
If desired, a configuration can be converted to a user defined type by way of
`serde::Deserialize`. However, while more convenient, this cannot be done in
`const` contexts, which may or may not pessimize optimizations.

```rust
use const_config::{Value, include_toml};

const APP_CFG: Value<'static> = include_toml!("app-config.toml");

#[derive(serde::Deserialize)]
struct Config {
    general: General,
    networking: Networking,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct General {
    trace_logging: bool,
    production: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Networking {
    allow_insecure: bool,
    default_port: u16,
}

fn main() {
    let config: Config = APP_CFG.intepret_as();

    if config.general.production {
        println!("production mode");
    } else {
        println!("development mode");
    }
}
```
