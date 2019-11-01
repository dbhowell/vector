use inventory;
use serde::de::{Deserializer, Error};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;
use toml::Value;

/// Combines a type field and a nested plugin config to create a table.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigSwapOut {
    #[serde(rename = "type")]
    pub type_str: String,
    #[serde(flatten)]
    pub nested: Value,
}

/// Stores both a constructed plugin instance and the ConfigSwapOut used to
/// create it, this allows us to echo back the config of a component at runtime.
pub struct ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    swap_out: ConfigSwapOut,

    pub component: T,
}

impl<T> fmt::Debug for ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.swap_out.fmt(f)
    }
}

impl<T> ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    /// Returns a sorted Vec of all plugins registered of a type.
    pub fn types() -> Vec<&'static str> {
        let mut types = Vec::new();
        for definition in inventory::iter::<ComponentBuilder<T>> {
            types.push(definition.name);
        }
        types.sort();
        types
    }
}

impl<T> Serialize for ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.swap_out.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut swap_out = ConfigSwapOut::deserialize(deserializer)?;
        inventory::iter::<ComponentBuilder<T>>
            .into_iter()
            .find(|t| t.name == swap_out.type_str)
            .ok_or(Error::custom(format!(
                "unrecognized type '{}'",
                swap_out.type_str
            )))
            .and_then(|b| {
                (b.from_value)(swap_out.nested.clone())
                    .map_err(|e| {
                        Error::custom(format!(
                            "failed to parse type `{}`: {}",
                            swap_out.type_str, e,
                        ))
                    })
                    .map(|(c, v_with_defaults)| {
                        swap_out.nested = v_with_defaults.unwrap_or(swap_out.nested);
                        Self {
                            swap_out: swap_out,
                            component: c,
                        }
                    })
            })
    }
}

pub struct ComponentBuilder<T: Sized> {
    pub name: &'static str,
    from_value: fn(Value) -> Result<(T, Option<Value>), String>,
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
                    .map(|c| (c.into(), None))
                    .map_err(|e| format!("{}", e))
            },
        }
    }

    pub fn new_cloneable<'de, B>(name: &'static str) -> Self
    where
        B: Clone + Into<T> + Serialize + Deserialize<'de>,
    {
        ComponentBuilder {
            name: name,
            from_value: |value| {
                value
                    .try_into::<B>()
                    .map(|c| (c.clone().into(), Value::try_from(c).ok()))
                    .map_err(|e| format!("{}", e))
            },
        }
    }
}
