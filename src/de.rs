use crate::{DeserializeFn, DeserializerRegistry, Registry};
use serde::de::{self, DeserializeSeed, Deserializer, Expected, Visitor};
use std::fmt;

pub struct MapLookupVisitor<'a, T: ?Sized + 'static> {
    pub expected: &'a dyn Expected,
    pub registry: &'static Registry<T>,
}

impl<'de, 'a, T: ?Sized + 'static> Visitor<'de> for MapLookupVisitor<'a, T> {
    type Value = DeserializeFn<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Expected::fmt(self.expected, formatter)
    }

    fn visit_str<E>(self, key: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.registry.get_deserializer(key, self.expected)
    }
}

impl<'de, 'a, T: ?Sized + 'static> DeserializeSeed<'de> for MapLookupVisitor<'a, T> {
    type Value = DeserializeFn<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

pub struct FnApply<T: ?Sized> {
    pub deserialize_fn: DeserializeFn<T>,
}

impl<'de, T: ?Sized> DeserializeSeed<'de> for FnApply<T> {
    type Value = Box<T>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut erased = <dyn erased_serde::Deserializer>::erase(deserializer);
        (self.deserialize_fn)(&mut erased).map_err(de::Error::custom)
    }
}
