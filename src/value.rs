use serde::Deserialize;
use wasmtime::component::Val;

#[derive(Debug)]
pub struct Value(Val);

impl Value {
    pub fn val(&self) -> Val {
        self.0.clone()
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;

        let val = match value {
            serde_json::Value::Bool(b) => Val::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    if i >= i8::MIN as i64 && i <= i8::MAX as i64 {
                        Val::S8(i as i8)
                    } else if i >= i16::MIN as i64 && i <= i16::MAX as i64 {
                        Val::S16(i as i16)
                    } else if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                        Val::S32(i as i32)
                    } else {
                        Val::S64(i)
                    }
                } else if let Some(u) = n.as_u64() {
                    if u <= u8::MAX as u64 {
                        Val::U8(u as u8)
                    } else if u <= u16::MAX as u64 {
                        Val::U16(u as u16)
                    } else if u <= u32::MAX as u64 {
                        Val::U32(u as u32)
                    } else {
                        Val::U64(u)
                    }
                } else if let Some(f) = n.as_f64() {
                    if f >= f32::MIN as f64 && f <= f32::MAX as f64 {
                        Val::Float32(f as f32)
                    } else {
                        Val::Float64(f)
                    }
                } else {
                    return Err(serde::de::Error::custom("invalid number format"));
                }
            }
            serde_json::Value::String(s) => Val::String(s),
            serde_json::Value::Null => Val::String("".to_string()), // TODO
            _ => return Err(serde::de::Error::custom("unsupported value type")),
        };

        Ok(Value(val))
    }
}
