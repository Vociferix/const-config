#![cfg(feature = "toml")]

use const_config::{Date, DateTime, Time, Value};

const CFG: Value<'static> = const_config::from_toml!(
    r#"
i = 42
b = true
s = 'hello world'
a = [1, -2, 4.25, 0.0]

[o]
e = []
dt = [
    1970-02-03T04:05:06.789,
    1970-02-03T04:05:06.789Z,
    1970-02-03T04:05:06.789+01:30,
]
d = 1970-02-03
t = 10:30:05.123

[o.u]
"#
);

#[test]
fn const_test() {
    assert!(const { CFG.is_object() });
    assert!(const { CFG.as_object().contains("i") });
    assert!(const { CFG.as_object().contains("b") });
    assert!(const { CFG.as_object().contains("s") });
    assert!(const { CFG.as_object().contains("a") });
    assert!(const { CFG.as_object().contains("o") });
    assert!(const { CFG.as_object().get("i").as_u8() == 42 });
    assert!(const { matches!(CFG.as_object().get("b"), Value::Bool(true)) });
    assert!(const { CFG.as_object().get("s").is_str() });
    assert_eq!(CFG.as_object().get("s").as_str(), "hello world");
    assert!(const { CFG.as_object().get("a").as_array().len() == 4 });
    assert!(const { CFG.as_object().get("a").as_array()[0].as_u8() == 1 });
    assert!(const { CFG.as_object().get("a").as_array()[1].as_i8() == -2 });
    assert!(const { CFG.as_object().get("a").as_array()[2].as_f32() == 4.25 });
    assert!(const { CFG.as_object().get("a").as_array()[3].as_u8() == 0 });
    assert!(const { CFG.as_object().get("o").is_object() });
    assert!(const { CFG.as_object().get("o").as_object().contains("u") });
    assert!(const { CFG.as_object().get("o").as_object().contains("e") });
    assert!(const { CFG.as_object().get("o").as_object().contains("dt") });
    assert!(const { CFG.as_object().get("o").as_object().contains("d") });
    assert!(const { CFG.as_object().get("o").as_object().contains("t") });
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
    assert!(const { CFG.as_object().get("o").as_object().get("t").is_time() });
    assert!(const { CFG.as_object().get("o").as_object().get("t").as_time().hour == 10 });
    assert!(
        const {
            CFG.as_object()
                .get("o")
                .as_object()
                .get("t")
                .as_time()
                .minute
                == 30
        }
    );
    assert!(
        const {
            CFG.as_object()
                .get("o")
                .as_object()
                .get("t")
                .as_time()
                .second
                == 5
        }
    );
    assert!(
        const {
            CFG.as_object()
                .get("o")
                .as_object()
                .get("t")
                .as_time()
                .nanosecond
                == 123_000_000
        }
    );
    assert!(const { CFG.as_object().get("o").as_object().get("d").is_date() });
    assert!(const { CFG.as_object().get("o").as_object().get("d").as_date().year == 1970 });
    assert!(
        const {
            CFG.as_object()
                .get("o")
                .as_object()
                .get("d")
                .as_date()
                .month
                == 2
        }
    );
    assert!(const { CFG.as_object().get("o").as_object().get("d").as_date().day == 3 });
    assert!(const { CFG.as_object().get("o").as_object().get("dt").is_array() });
    assert!(const { CFG.as_object().get("o").as_object().get("dt").as_array()[0].is_datetime() });
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .date
                .year
                == 1970
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .date
                .month
                == 2
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .date
                .day
                == 3
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .time
                .hour
                == 4
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .time
                .minute
                == 5
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .time
                .second
                == 6
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .time
                .nanosecond
                == 789_000_000
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[0]
                .as_datetime()
                .offset
                .is_none()
        }
    );
    assert!(const { CFG.as_object().get("o").as_object().get("dt").as_array()[1].is_datetime() });
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .date
                .year
                == 1970
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .date
                .month
                == 2
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .date
                .day
                == 3
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .time
                .hour
                == 4
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .time
                .minute
                == 5
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .time
                .second
                == 6
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .time
                .nanosecond
                == 789_000_000
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .offset
                .is_some()
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[1]
                .as_datetime()
                .offset
                .unwrap()
                == 0
        }
    );
    assert!(const { CFG.as_object().get("o").as_object().get("dt").as_array()[2].is_datetime() });
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .date
                .year
                == 1970
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .date
                .month
                == 2
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .date
                .day
                == 3
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .time
                .hour
                == 4
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .time
                .minute
                == 5
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .time
                .second
                == 6
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .time
                .nanosecond
                == 789_000_000
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .offset
                .is_some()
        }
    );
    assert!(
        const {
            CFG.as_object().get("o").as_object().get("dt").as_array()[2]
                .as_datetime()
                .offset
                .unwrap()
                == 90
        }
    );
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct Test {
    i: i32,
    b: bool,
    s: String,
    a: Vec<f32>,
    o: Inner,
}

#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct Inner {
    e: Vec<i32>,
    dt: Vec<DateTime>,
    d: Date,
    t: Time,
    u: (),
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
    assert_eq!(value.o.u, ());
    assert!(value.o.e.is_empty());
    assert_eq!(value.o.t.hour, 10);
    assert_eq!(value.o.t.minute, 30);
    assert_eq!(value.o.t.second, 5);
    assert_eq!(value.o.t.nanosecond, 123_000_000);
    assert_eq!(value.o.d.year, 1970);
    assert_eq!(value.o.d.month, 2);
    assert_eq!(value.o.d.day, 3);
    assert_eq!(value.o.dt.len(), 3);
    assert_eq!(value.o.dt[0].date.year, 1970);
    assert_eq!(value.o.dt[0].date.month, 2);
    assert_eq!(value.o.dt[0].date.day, 3);
    assert_eq!(value.o.dt[0].time.hour, 4);
    assert_eq!(value.o.dt[0].time.minute, 5);
    assert_eq!(value.o.dt[0].time.second, 6);
    assert_eq!(value.o.dt[0].time.nanosecond, 789_000_000);
    assert!(value.o.dt[0].offset.is_none());
    assert_eq!(value.o.dt[1].date.year, 1970);
    assert_eq!(value.o.dt[1].date.month, 2);
    assert_eq!(value.o.dt[1].date.day, 3);
    assert_eq!(value.o.dt[1].time.hour, 4);
    assert_eq!(value.o.dt[1].time.minute, 5);
    assert_eq!(value.o.dt[1].time.second, 6);
    assert_eq!(value.o.dt[1].time.nanosecond, 789_000_000);
    assert!(value.o.dt[1].offset.is_some());
    assert_eq!(value.o.dt[1].offset.unwrap(), 0);
    assert_eq!(value.o.dt[2].date.year, 1970);
    assert_eq!(value.o.dt[2].date.month, 2);
    assert_eq!(value.o.dt[2].date.day, 3);
    assert_eq!(value.o.dt[2].time.hour, 4);
    assert_eq!(value.o.dt[2].time.minute, 5);
    assert_eq!(value.o.dt[2].time.second, 6);
    assert_eq!(value.o.dt[2].time.nanosecond, 789_000_000);
    assert!(value.o.dt[2].offset.is_some());
    assert_eq!(value.o.dt[2].offset.unwrap(), 90);
}
