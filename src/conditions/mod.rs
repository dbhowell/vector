use crate::topology::config::component::{ComponentBuilder, ComponentConfig};
use crate::Event;
use inventory;

pub mod field_equals;
pub mod static_value;

pub trait Condition {
    fn check(&self, e: &Event) -> Result<bool, String>;
}

/// Provides the double jump from `T: Condition` to `Box<dyn Condition>`.
pub struct BoxCondition {
    pub inner: Box<dyn Condition>,
}

impl<T> From<T> for BoxCondition
where
    T: Condition + Send + Sync + 'static,
{
    fn from(inner: T) -> Self {
        BoxCondition {
            inner: Box::new(inner),
        }
    }
}

pub type ConditionDefinition = ComponentBuilder<BoxCondition>;
pub type ConditionConfig = ComponentConfig<BoxCondition>;

inventory::collect!(ConditionDefinition);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn list_types() {
        assert_eq!(ConditionConfig::types(), ["field_equals", "static"]);
    }

    #[test]
    fn parse_bad_config_type() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      type = "not a real type"
      value = false
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "unrecognized type 'not a real type'".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_type() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      nottype = "missing a type here"
      value = false
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "missing field `type`".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_extra_field() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      type = "static"
      value = false
      extra_field = "is unexpected"
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "failed to parse type `static`: unknown field `extra_field`, expected `value`"
                .to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_field() {
        assert_eq!(
            toml::from_str::<ConditionConfig>(
                r#"
      type = "static"
      "#
            )
            .err()
            .map(|e| format!("{}", e))
            .unwrap_or("".to_owned()),
            "failed to parse type `static`: missing field `value`".to_owned(),
        );
    }
}
