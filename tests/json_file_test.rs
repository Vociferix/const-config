#![cfg(feature = "json")]

use const_config::{Number, Value};

const CFG: Value<'static> = const_config::include_json!("tests/test.json");

#[test]
fn const_test() {
    assert!(const { CFG.is_object() });
    assert!(const { CFG.as_object().contains("i") });
    assert!(const { CFG.as_object().contains("b") });
    assert!(const { CFG.as_object().contains("s") });
    assert!(const { CFG.as_object().contains("a") });
    assert!(const { CFG.as_object().contains("o") });
    assert!(const { matches!(CFG.as_object().get("i"), Value::Number(Number::UInt(42))) });
    assert!(const { matches!(CFG.as_object().get("b"), Value::Bool(true)) });
    assert!(const { CFG.as_object().get("s").is_str() });
    assert_eq!(CFG.as_object().get("s").as_str(), "hello world");
    assert!(
        const {
            matches!(
                CFG.as_object().get("a"),
                &Value::Array(&[
                    Value::Number(Number::UInt(1)),
                    Value::Number(Number::Int(-2)),
                    Value::Number(Number::Float(_)),
                    Value::Number(Number::Float(_)),
                ])
            )
        }
    );
    assert!(const { CFG.as_object().get("a").as_array()[2].as_f64() == 4.25 });
    assert!(const { CFG.as_object().get("a").as_array()[3].as_f64() == 0.0 });
    assert!(const { CFG.as_object().get("o").is_object() });
    assert!(const { CFG.as_object().get("o").as_object().contains("n") });
    assert!(const { CFG.as_object().get("o").as_object().contains("u") });
    assert!(const { CFG.as_object().get("o").as_object().contains("e") });
    assert!(const { CFG.as_object().get("o").as_object().get("n").is_null() });
    assert!(const { CFG.as_object().get("o").as_object().get("u").is_object() });
    assert!(
        const {
            CFG.as_object()
                .get("o")
                .as_object()
                .get("u")
                .as_object()
                .is_empty()
        }
    );
    assert!(const { CFG.as_object().get("o").as_object().get("e").is_array() });
    assert!(
        const {
            CFG.as_object()
                .get("o")
                .as_object()
                .get("e")
                .as_array()
                .is_empty()
        }
    );
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct Test {
    i: i32,
    b: bool,
    s: String,
    a: Vec<f32>,
    o: Inner,
}

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
struct Inner {
    n: Option<i32>,
    u: (),
    e: Vec<i32>,
}

#[cfg(feature = "serde")]
#[test]
fn deser_test() {
    let value: Test = CFG.interpret_as();

    assert_eq!(value.i, 42);
    assert_eq!(value.b, true);
    assert_eq!(value.s, "hello world");
    assert_eq!(value.a.len(), 4);
    assert!(value.a[0] == 1.0);
    assert!(value.a[1] == -2.0);
    assert!(value.a[2] == 4.25);
    assert!(value.a[3] == 0.0);
    assert!(value.o.n.is_none());
    assert_eq!(value.o.u, ());
    assert!(value.o.e.is_empty());
}
