//! Value types for VDBE registers and results

/// A SQLite value that can be stored in a VDBE register or returned as a result
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum Value {
    /// NULL value
    #[default]
    Null,
    /// 64-bit signed integer
    Integer(i64),
    /// 64-bit floating point
    Real(f64),
    /// UTF-8 text string
    Text(String),
    /// Binary blob
    Blob(Vec<u8>),
}

impl Value {
    /// Check if the value is NULL
    #[inline]
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    /// Check if the value is an integer
    #[inline]
    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    /// Check if the value is a real (float)
    #[inline]
    pub fn is_real(&self) -> bool {
        matches!(self, Value::Real(_))
    }

    /// Check if the value is text
    #[inline]
    pub fn is_text(&self) -> bool {
        matches!(self, Value::Text(_))
    }

    /// Check if the value is a blob
    #[inline]
    pub fn is_blob(&self) -> bool {
        matches!(self, Value::Blob(_))
    }

    /// Get the value as an integer, with type coercion
    ///
    /// - Integer: returns the value
    /// - Real: truncates to integer
    /// - Text: attempts to parse
    /// - Blob/Null: returns None
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Real(r) => Some(*r as i64),
            Value::Text(s) => s.parse().ok(),
            Value::Null | Value::Blob(_) => None,
        }
    }

    /// Get the value as a real (float), with type coercion
    ///
    /// - Real: returns the value
    /// - Integer: converts to float
    /// - Text: attempts to parse
    /// - Blob/Null: returns None
    pub fn as_real(&self) -> Option<f64> {
        match self {
            Value::Real(r) => Some(*r),
            Value::Integer(i) => Some(*i as f64),
            Value::Text(s) => s.parse().ok(),
            Value::Null | Value::Blob(_) => None,
        }
    }

    /// Get the value as text
    ///
    /// Only returns Some for Text values (no coercion)
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }

    /// Get the value as a blob
    ///
    /// Only returns Some for Blob values (no coercion)
    pub fn as_blob(&self) -> Option<&[u8]> {
        match self {
            Value::Blob(b) => Some(b),
            _ => None,
        }
    }

    /// Convert to string representation
    pub fn to_string_lossy(&self) -> String {
        match self {
            Value::Null => String::from("NULL"),
            Value::Integer(i) => i.to_string(),
            Value::Real(r) => r.to_string(),
            Value::Text(s) => s.clone(),
            Value::Blob(b) => format!("X'{}'", hex_encode(b)),
        }
    }
}


// Conversion from primitive types
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v as i64)
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Real(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Real(v as f64)
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(v)
    }
}

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Value::Blob(v.to_vec())
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(val) => val.into(),
            None => Value::Null,
        }
    }
}

/// Helper function to encode bytes as hex string
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_types() {
        assert!(Value::Null.is_null());
        assert!(Value::Integer(42).is_integer());
        assert!(Value::Real(3.14).is_real());
        assert!(Value::Text("hello".into()).is_text());
        assert!(Value::Blob(vec![1, 2, 3]).is_blob());
    }

    #[test]
    fn test_integer_coercion() {
        assert_eq!(Value::Integer(42).as_integer(), Some(42));
        assert_eq!(Value::Real(3.7).as_integer(), Some(3));
        assert_eq!(Value::Text("123".into()).as_integer(), Some(123));
        assert_eq!(Value::Null.as_integer(), None);
    }

    #[test]
    fn test_from_conversions() {
        let v: Value = 42i64.into();
        assert_eq!(v, Value::Integer(42));

        let v: Value = 3.14f64.into();
        assert_eq!(v, Value::Real(3.14));

        let v: Value = "hello".into();
        assert_eq!(v, Value::Text("hello".to_string()));

        let v: Value = None::<i64>.into();
        assert_eq!(v, Value::Null);
    }
}
