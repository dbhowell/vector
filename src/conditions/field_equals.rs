use serde::{Deserialize, Serialize};
use string_cache::DefaultAtom as Atom;

use crate::{
    conditions::{Condition, ConditionDefinition},
    Event,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FieldEqualsConfig {
    #[serde(default = "default_field")]
    field: Atom,
    value: Atom,
}

fn default_field() -> Atom {
    "message".into()
}

impl Condition for FieldEqualsConfig {
    fn check(&self, event: &Event) -> Result<bool, String> {
        event
            .as_log()
            .get(&self.field)
            .map(|v| self.value == v.to_string_lossy())
            .ok_or(format!("field '{}' not found", self.field))
    }
}

inventory::submit! {
    ConditionDefinition::new_cloneable::<FieldEqualsConfig>("field_equals")
}

#[cfg(test)]
mod test {
    use crate::{conditions::ConditionConfig, Event};

    #[test]
    fn parse_field_equals_config() {
        let config_false: ConditionConfig = toml::from_str(
            r#"
      type = "field_equals"
      value = "foo bar"
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(false),
            config_false
                .component
                .inner
                .check(&Event::from("baz".to_owned()))
        );

        let config_err: ConditionConfig = toml::from_str(
            r#"
      type = "field_equals"
      field = "doesntexist"
      value = "foo bar"
      "#,
        )
        .unwrap();

        assert_eq!(
            Err("field 'doesntexist' not found".to_owned()),
            config_err
                .component
                .inner
                .check(&Event::from("foo bar".to_owned()))
        );

        let config_true: ConditionConfig = toml::from_str(
            r#"
      type = "field_equals"
      value = "baz"
      "#,
        )
        .unwrap();

        assert_eq!(
            Ok(true),
            config_true
                .component
                .inner
                .check(&Event::from("baz".to_owned()))
        );
    }

    #[test]
    fn print_field_equals_config() {
        let config_full: ConditionConfig = toml::from_str(
            r#"
      type = "field_equals"
      field = "custom"
      value = "foo bar"
      "#,
        )
        .unwrap();

        assert_eq!(
            r#"type = "field_equals"
field = "custom"
value = "foo bar"
"#,
            toml::to_string(&config_full).unwrap()
        );

        let config_partial: ConditionConfig = toml::from_str(
            r#"
      type = "field_equals"
      value = "foo bar"
      "#,
        )
        .unwrap();

        assert_eq!(
            r#"type = "field_equals"
field = "message"
value = "foo bar"
"#,
            toml::to_string(&config_partial).unwrap()
        );
    }
}
