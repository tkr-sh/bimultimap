use {
    crate::BiMultiMap,
    serde::{
        Deserialize,
        Serialize,
        de::{MapAccess, Visitor},
        ser::SerializeMap,
    },
    std::{borrow::Borrow, hash::Hash, marker::PhantomData},
};

impl<L, R> Serialize for BiMultiMap<L, R>
where
    L: Hash + Eq + Serialize,
    R: Hash + Eq + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_map(None)?;
        for (left, right) in self.iter() {
            seq.serialize_entry(left.borrow() as &L, right.borrow() as &R)?;
        }
        seq.end()
    }
}

impl<'de, L, R> serde::Deserialize<'de> for BiMultiMap<L, R>
where
    L: Deserialize<'de> + Hash + Eq,
    R: Deserialize<'de> + Hash + Eq,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(BiMultiMapVisitor::new())
    }
}

struct BiMultiMapVisitor<L: Hash + Eq, R: Hash + Eq> {
    marker: PhantomData<fn() -> BiMultiMap<L, R>>,
}

impl<L: Hash + Eq, R: Hash + Eq> BiMultiMapVisitor<L, R> {
    fn new() -> Self {
        BiMultiMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de, L, R> Visitor<'de> for BiMultiMapVisitor<L, R>
where
    L: Deserialize<'de> + Hash + Eq,
    R: Deserialize<'de> + Hash + Eq,
{
    type Value = BiMultiMap<L, R>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a very special map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = BiMultiMap::new();

        while let Some((left, right)) = access.next_entry()? {
            map.insert(left, right);
        }

        Ok(map)
    }
}
