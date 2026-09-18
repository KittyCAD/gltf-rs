//! Preserve finite IEEE-754 values in glTF and historical geometry metadata without
//! enabling serde_json's `float_roundtrip` feature for unrelated consumers.

use serde::de::Error;
use serde_json::{value::RawValue, Value};

/// Decode a JSON value with correctly rounded finite floating-point numbers.
/// JSON syntax, strings, and container boundaries are handled by serde_json.
/// The depth bound also applies to opaque, otherwise unvalidated metadata.
pub fn from_slice(bytes: &[u8]) -> serde_json::Result<Value> {
    let raw: &RawValue = serde_json::from_slice(bytes)?;
    decode(raw, 0)
}

fn decode(raw: &RawValue, depth: usize) -> serde_json::Result<Value> {
    if depth >= 128 {
        return Err(serde_json::Error::custom(
            "metadata recursion limit exceeded",
        ));
    }
    let token = raw.get();
    match token.as_bytes()[0] {
        b'[' => {
            let values: Vec<&RawValue> = serde_json::from_str(token)?;
            values.into_iter().map(|v| decode(v, depth + 1)).collect()
        }
        b'{' => {
            // Insert in wire order: downstream feature unification may enable
            // serde_json/preserve_order. A BTreeMap intermediate would silently
            // reorder opaque source history in that configuration.
            struct ObjectVisitor(usize);
            impl<'de> serde::de::Visitor<'de> for ObjectVisitor {
                type Value = Value;

                fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str("a JSON object")
                }

                fn visit_map<A: serde::de::MapAccess<'de>>(
                    self,
                    mut entries: A,
                ) -> Result<Value, A::Error> {
                    let mut object = serde_json::Map::new();
                    while let Some((key, raw)) = entries.next_entry::<String, &'de RawValue>()? {
                        object.insert(key, decode(raw, self.0 + 1).map_err(A::Error::custom)?);
                    }
                    Ok(Value::Object(object))
                }
            }
            serde::Deserializer::deserialize_map(
                &mut serde_json::Deserializer::from_str(token),
                ObjectVisitor(depth),
            )
        }
        b'-' | b'0'..=b'9' => {
            // Keep JSON integers as integers, and keep negative zero's sign.
            if token != "-0" && !token.contains(['.', 'e', 'E']) {
                if let Ok(value) = token.parse::<u64>() {
                    return Ok(value.into());
                }
                if let Ok(value) = token.parse::<i64>() {
                    return Ok(value.into());
                }
            }
            let value: f64 = token.parse().map_err(serde_json::Error::custom)?;
            serde_json::Number::from_f64(value)
                .map(Value::Number)
                .ok_or_else(|| serde_json::Error::custom("non-finite metadata number"))
        }
        _ => serde_json::from_str(token),
    }
}

#[cfg(feature = "extras")]
pub(crate) fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<super::Extras, D::Error> {
    let raw = <Box<RawValue> as serde::Deserialize>::deserialize(deserializer)?;
    let value = from_slice(raw.get().as_bytes()).map_err(D::Error::custom)?;
    serde_json::from_value(value).map_err(D::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_float_bits_and_integer_types() {
        let input =
            b"[0.12958333333333039,-0,-0.0,18446744073709551615,-9223372036854775808,5e-324]";
        let decoded = from_slice(input).unwrap();
        assert_eq!(decoded[0].as_f64().unwrap().to_bits(), 0x3fc0962fc962fc2c);
        assert_eq!(decoded[1].as_f64().unwrap().to_bits(), (-0.0f64).to_bits());
        assert_eq!(decoded[2].as_f64().unwrap().to_bits(), (-0.0f64).to_bits());
        assert_eq!(decoded[3].as_u64(), Some(u64::MAX));
        assert_eq!(decoded[4].as_i64(), Some(i64::MIN));
        assert_eq!(decoded[5].as_f64().unwrap().to_bits(), 1);
    }

    #[test]
    fn preserves_object_order_under_the_consumers_json_features() {
        let mut object = serde_json::Map::new();
        object.insert("z".into(), Value::from(1));
        object.insert("a".into(), serde_json::json!({"y": 2, "b": 3}));
        let original = Value::Object(object);
        let bytes = serde_json::to_vec(&original).unwrap();
        let decoded = from_slice(&bytes).unwrap();
        assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
    }

    #[test]
    fn root_readers_preserve_active_geometry_numbers() {
        let text = r#"{"asset":{"version":"2.0"},"accessors":[{"componentType":5126,"count":1,"type":"SCALAR","min":[0.12958333333333039]}]}"#;
        for root in [
            crate::Root::from_str(text).unwrap(),
            crate::Root::from_slice(text.as_bytes()).unwrap(),
            crate::Root::from_reader(text.as_bytes()).unwrap(),
        ] {
            let value = root.accessors[0].min.as_ref().unwrap()[0].as_f64().unwrap();
            assert_eq!(value.to_bits(), 0x3fc0962fc962fc2c);
        }
    }

    #[test]
    fn rejects_invalid_nonfinite_and_deep_metadata() {
        for input in ["NaN", "1e400", "[01]", "[1,]", "{\"v\":-Infinity}"] {
            assert!(from_slice(input.as_bytes()).is_err(), "{input}");
        }
        let input = format!("{}0{}", "[".repeat(129), "]".repeat(129));
        assert!(from_slice(input.as_bytes()).is_err());
    }

    #[cfg(feature = "extras")]
    #[test]
    fn root_extras_preserve_bits_through_text_and_value_decoding() {
        let root: crate::Root = serde_json::from_str(
            r#"{"asset":{"version":"2.0"},"extras":{"witness":0.12958333333333039}}"#,
        )
        .unwrap();
        let bits = root.extras["witness"].as_f64().unwrap().to_bits();
        assert_eq!(bits, 0x3fc0962fc962fc2c);
        let root: crate::Root =
            serde_json::from_value(serde_json::to_value(root).unwrap()).unwrap();
        assert_eq!(root.extras["witness"].as_f64().unwrap().to_bits(), bits);
        let ordinary: f64 = serde_json::from_str("0.12958333333333039").unwrap();
        assert_eq!(ordinary.to_bits(), 0x3fc0962fc962fc2d);
    }

    #[cfg(all(feature = "extras", feature = "KITTYCAD_boundary_representation"))]
    #[test]
    fn solid_extras_preserve_bits_from_reader_and_value() {
        use crate::extensions::kittycad_boundary_representation::Solid;
        let text = r#"{"shells":[],"extras":{"witness":0.12958333333333039,"opaque":{"signedZero":-0.0}}}"#;
        let solid: Solid = serde_json::from_reader(text.as_bytes()).unwrap();
        assert_eq!(
            solid.extras["witness"].as_f64().unwrap().to_bits(),
            0x3fc0962fc962fc2c
        );
        let solid: Solid = serde_json::from_value(serde_json::to_value(solid).unwrap()).unwrap();
        assert_eq!(
            solid.extras["witness"].as_f64().unwrap().to_bits(),
            0x3fc0962fc962fc2c
        );
        assert_eq!(
            solid.extras["opaque"]["signedZero"]
                .as_f64()
                .unwrap()
                .to_bits(),
            (-0.0f64).to_bits()
        );
    }
}
