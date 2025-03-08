use {
    crate::BiMultiMap,
    mlua::{Error, FromLua, IntoLua, Lua, Value},
    std::{hash::Hash, ops::Deref},
};

impl<K: Eq + Hash + FromLua, V: FromLua + Eq + Hash> FromLua for BiMultiMap<K, V> {
    #[inline]
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        if let Value::Table(table) = value {
            table.pairs().collect()
        } else {
            Err(Error::FromLuaConversionError {
                from:    value.type_name(),
                to:      String::from("BiMultiMap"),
                message: Some("expected table".to_string()),
            })
        }
    }
}

impl<K, V> IntoLua for BiMultiMap<K, V>
where
    K: Eq + Hash + IntoLua + Clone,
    V: IntoLua + Hash + Eq + Clone,
{
    #[inline]
    fn into_lua(self, lua: &Lua) -> mlua::Result<Value> {
        Ok(Value::Table(
            lua.create_table_from(
                self.into_iter()
                    .map(|(l, r)| (l.deref().clone(), r.deref().clone())),
            )?,
        ))
    }
}
