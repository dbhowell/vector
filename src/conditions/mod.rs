use crate::topology::config::component::ComponentBuilder;
use crate::Event;
use inventory;

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

inventory::collect!(ConditionDefinition);

#[cfg(test)]
mod test {
    use super::*;
    use crate::topology::config::component::ConfigSwapOut;

    #[test]
    fn list_types() {
        assert_eq!(ConditionDefinition::types(), ["static"]);
    }

    #[test]
    fn parse_bad_config_type() {
        assert_eq!(
            toml::from_str::<ConfigSwapOut>(
                r#"
      type = "not a real type"
      value = false
      "#
            )
            .map_err(|e| format!("{}", e))
            .and_then(|c| c.try_into::<BoxCondition>())
            .err()
            .unwrap_or("".to_owned()),
            "unrecognized type 'not a real type'".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_type() {
        assert_eq!(
            toml::from_str::<ConfigSwapOut>(
                r#"
      nottype = "missing a type here"
      value = false
      "#
            )
            .map_err(|e| format!("{}", e))
            .and_then(|c| c.try_into::<BoxCondition>())
            .err()
            .unwrap_or("".to_owned()),
            "missing field `type`".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_extra_field() {
        assert_eq!(
            toml::from_str::<ConfigSwapOut>(
                r#"
      type = "static"
      value = false
      extra_field = "is unexpected"
      "#
            )
            .map_err(|e| format!("{}", e))
            .and_then(|c| c.try_into::<BoxCondition>())
            .err()
            .unwrap_or("".to_owned()),
            "unknown field `extra_field`, expected `value`".to_owned(),
        );
    }

    #[test]
    fn parse_bad_config_missing_field() {
        assert_eq!(
            toml::from_str::<ConfigSwapOut>(
                r#"
      type = "static"
      "#
            )
            .map_err(|e| format!("{}", e))
            .and_then(|c| c.try_into::<BoxCondition>())
            .err()
            .unwrap_or("".to_owned()),
            "missing field `value`".to_owned(),
        );
    }
}
