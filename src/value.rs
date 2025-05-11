use serde::Deserialize;
use wasmtime::component::{Type, Val};

#[derive(Debug)]
pub struct Value {
    ty: Type,
    value: String,
}

impl Value {
    pub fn val(&self) -> Result<Val, wasm_wave::parser::ParserError> {
        Ok(wasm_wave::from_str(&self.ty, &self.value)?)
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Type,
            Value,
        }

        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Value with type and value fields")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut ty = None;
                let mut value = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if ty.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let type_str: String = map.next_value()?;
                            ty = Some(match type_str.as_str() {
                                "bool" => Type::Bool,
                                "s8" => Type::S8,
                                "u8" => Type::U8,
                                "s16" => Type::S16,
                                "u16" => Type::U16,
                                "s32" => Type::S32,
                                "u32" => Type::U32,
                                "s64" => Type::S64,
                                "u64" => Type::U64,
                                "float32" => Type::Float32,
                                "float64" => Type::Float64,
                                "char" => Type::Char,
                                "string" => Type::String,
                                _ => {
                                    return Err(serde::de::Error::custom(format!(
                                        "unsupported type: {}",
                                        type_str
                                    )))
                                }
                            });
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                    }
                }

                let ty = ty.ok_or_else(|| serde::de::Error::missing_field("type"))?;
                let value = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;

                Ok(Value { ty, value })
            }
        }

        deserializer.deserialize_struct("Value", &["type", "value"], ValueVisitor)
    }
}
