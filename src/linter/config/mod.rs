use serde::{Deserialize, Serialize};

use super::rules::lints::quotes::QuotesConfig;
use std::fmt::Debug;

#[derive(Default, Clone)]
pub struct LintConfig {
    pub quotes: RuleConfig<QuotesConfig>,
}

// Adapted from
// https://github.com/swc-project/swc/blob/e9c1b229262c07d114e4b75bbc9f104b45fbedf3/crates/swc_ecma_lints/src/config.rs#L53-L67
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RuleConfig<T: Debug + Clone + Serialize + Default>(#[serde(default)] T);

impl<T: Debug + Clone + Serialize + Default> RuleConfig<T> {
    pub(crate) fn get_rule_config(&self) -> &T {
        &self.0
    }
}
// end
