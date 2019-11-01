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
    use crate::{conditions::BoxCondition, topology::config::component::ConfigSwapOut, Event};

    #[test]
    fn parse_static_config() {
        assert_eq!(
            Ok(false),
            toml::from_str::<ConfigSwapOut>(
                r#"
            type = "static"
            value = false
            "#,
            )
            .unwrap()
            .try_into::<BoxCondition>()
            .unwrap()
            .inner
            .check(&Event::from("foo bar baz".to_owned()))
        );

        assert_eq!(
            Ok(true),
            toml::from_str::<ConfigSwapOut>(
                r#"
            type = "static"
            value = true
            "#,
            )
            .unwrap()
            .try_into::<BoxCondition>()
            .unwrap()
            .inner
            .check(&Event::from("foo bar baz".to_owned()))
        );
    }
}
