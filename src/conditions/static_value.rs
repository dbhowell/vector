use serde::{Deserialize, Serialize};

use crate::{
    conditions::{Condition, ConditionDefinition},
    Event,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StaticConfig {
    value: bool,
}

impl Condition for StaticConfig {
    fn check(&self, _: &Event) -> Result<bool, String> {
        return Ok(self.value);
    }
}

inventory::submit! {
    ConditionDefinition::new::<StaticConfig>("static")
}

#[cfg(test)]
mod test {
    use crate::{conditions::ConditionConfig, Event};

    #[test]
    fn parse_static_config() {
        let config_false: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = false
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(false),
            config_false
                .component
                .inner
                .check(&Event::from("foo bar baz".to_owned()))
        );

        let config_true: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = true
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(true),
            config_true
                .component
                .inner
                .check(&Event::from("foo bar baz".to_owned()))
        );
    }

    #[test]
    fn print_static_config() {
        let config_false: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = false
      "#,
        )
        .unwrap();

        assert_eq!(
            r#"type = "static"
value = false
"#,
            toml::to_string(&config_false).unwrap()
        );

        let config_true: ConditionConfig = toml::from_str(
            r#"
      type = "static"
      value = true
      "#,
        )
        .unwrap();

        assert_eq!(
            r#"type = "static"
value = true
"#,
            toml::to_string(&config_true).unwrap()
        );
    }
}
