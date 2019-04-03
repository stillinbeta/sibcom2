extern crate serde;

use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::fmt;

use crate::Value;

struct BMONVisitor;

impl<'de> Visitor<'de> for BMONVisitor {
    type Value = crate::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A String, a boolean, a number, an object, or a list")
    }

    fn visit_i64<E>(self, i: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(i))
    }

    fn visit_u64<E>(self, u: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(u as i64))
    }

    fn visit_f64<E>(self, f: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Number(f.round() as i64))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let s = String::from(s);
        Ok(if s.starts_with('/') {
            Value::RelativeLink(s)
        } else if s.contains('/') {
            Value::Link(s)
        } else {
            Value::String(s)
        })
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Null)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Value::Null)
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some(element) = visitor.next_element()? {
            vec.push(element);
        }

        Ok(Value::Sequence(vec))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut values = Mapping::new();

        while let Some((key, value)) = visitor.next_entry()? {
            values.insert(key, value);
        }

        Ok(Value::Mapping(values))
    }
}

impl<'de> Deserialize<'de> for crate::Value {
    fn deserialize<D>(deserializer: D) -> Result<crate::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BMONVisitor)
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
