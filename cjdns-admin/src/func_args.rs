//! Remote function argument list.

use std::collections::BTreeMap;

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use serde_json::Value as JsonValue;

/// Argument name (alias to `String`).
pub type ArgName = String;

/// Argument value (either integer, string, or JSON).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ArgValue {
    /// Integer argument value.
    Int(i64),
    /// String argument value.
    String(String),
    /// JSON argument value.
    Json(JsonValue),
}

impl From<i64> for ArgValue {
    #[inline]
    fn from(value: i64) -> Self {
        ArgValue::Int(value)
    }
}

impl From<String> for ArgValue {
    #[inline]
    fn from(value: String) -> Self {
        ArgValue::String(value)
    }
}

impl From<&str> for ArgValue {
    #[inline]
    fn from(value: &str) -> Self {
        ArgValue::String(value.to_string())
    }
}

impl From<JsonValue> for ArgValue {
    #[inline]
    fn from(value: JsonValue) -> Self {
        ArgValue::Json(value)
    }
}

/// Remote function argument values.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct ArgValues(BTreeMap<ArgName, ArgValue>);

impl ArgValues {
    /// New empty instance.
    #[inline]
    pub fn new() -> Self {
        ArgValues(BTreeMap::new())
    }

    /// Add argument value to list.
    #[inline]
    pub fn add<N: Into<ArgName>, V: Into<ArgValue>>(&mut self, name: N, value: V) -> &mut Self {
        let ArgValues(map) = self;
        map.insert(name.into(), value.into());
        self
    }
}

impl Serialize for ArgValues {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let ArgValues(map) = self;
        let mut encoder = serializer.serialize_map(Some(map.len()))?;
        for (k, v) in map {
            match v {
                ArgValue::Int(int_val) => encoder.serialize_entry(k, int_val)?,
                ArgValue::String(str_val) => encoder.serialize_entry(k, str_val)?,
                ArgValue::Json(json_val) => encoder.serialize_entry(k, json_val)?,
            }
        }
        encoder.end()
    }
}

#[cfg(test)]
mod tests {
    use super::{ArgValue, ArgValues};

    use serde_json::json;

    #[test]
    fn test_args_ser() -> Result<(), Box<dyn std::error::Error>> {
        let mut args = ArgValues::new();
        args.add("foo".to_string(), ArgValue::String("bar".to_string()));
        args.add("boo".to_string(), ArgValue::Int(42));
        args.add("zoo".to_string(), ArgValue::Int(-42));

        let benc = String::from_utf8(bencode::to_bytes(&args)?)?;
        assert_eq!(benc, "d3:booi42e3:foo3:bar3:zooi-42ee");

        Ok(())
    }

    #[test]
    fn test_args_ser_json() -> Result<(), Box<dyn std::error::Error>> {
        let mut args = ArgValues::new();
        args.add("foo".to_string(), ArgValue::Json(json!([42, -42, "bar"])));
        args.add(
            "boo".to_string(),
            ArgValue::Json(json!({
                "key1": "baz",
                "key2": ["Lorem", "ipsum", {"dolor": ["sit", "amet"]}]
            })),
        );

        let benc = String::from_utf8(bencode::to_bytes(&args)?)?;
        assert_eq!(benc, "d3:bood4:key13:baz4:key2l5:Lorem5:ipsumd5:dolorl3:sit4:ameteeee3:fooli42ei-42e3:baree");

        Ok(())
    }

    #[test]
    fn test_arg_value_conversion() {
        fn arg<T: Into<ArgValue>>(v: T) -> ArgValue {
            v.into()
        }

        assert_eq!(arg(42), ArgValue::Int(42));
        assert_eq!(arg(-42), ArgValue::Int(-42));

        assert_eq!(arg("foo"), ArgValue::String("foo".to_string()));
        assert_eq!(arg("bar".to_string()), ArgValue::String("bar".to_string()));
    }
}
