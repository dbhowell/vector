use inventory;
use serde::{Deserialize, Serialize};
use toml::Value;

/// Combines a type field and a nested plugin config to create a table.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigSwapOut {
    #[serde(rename = "type")]
    pub type_str: String,
    #[serde(flatten)]
    pub nested: Value,
}

impl ConfigSwapOut {
    pub fn try_into<T>(self) -> Result<T, String>
    where
        T: 'static + Sized,
        inventory::iter<ComponentBuilder<T>>:
            std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
    {
        inventory::iter::<ComponentBuilder<T>>
            .into_iter()
            .find(|t| t.name == self.type_str)
            .ok_or(format!("unrecognized type '{}'", self.type_str))
            .and_then(|b| (b.from_value)(self.nested.clone()))
    }
}

pub struct ComponentBuilder<T: Sized> {
    pub name: &'static str,
    from_value: fn(Value) -> Result<T, String>,
}

impl<T: Sized> ComponentBuilder<T> {
    pub fn new<'de, B>(name: &'static str) -> Self
    where
        B: Into<T> + Serialize + Deserialize<'de>,
    {
        ComponentBuilder {
            name: name,
            from_value: |value| {
                value
                    .try_into::<B>()
                    .map(|b| b.into())
                    .map_err(|e| format!("{}", e))
            },
        }
    }

    /// Returns a sorted Vec of all plugins registered of a type.
    pub fn types() -> Vec<&'static str>
    where
        T: 'static + Sized,
        inventory::iter<ComponentBuilder<T>>:
            std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
    {
        let mut types = Vec::new();
        for definition in inventory::iter::<ComponentBuilder<T>> {
            types.push(definition.name);
        }
        types.sort();
        types
    }
}
