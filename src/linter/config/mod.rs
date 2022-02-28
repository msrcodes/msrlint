use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::rules::lints::quotes::QuotesConfig;
use std::{collections::HashMap, fmt::Debug, fs::read_to_string, path::PathBuf};

#[derive(Default, Clone)]
pub struct LintConfig {
    pub quotes: RuleConfig<QuotesConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Rules {
    /// ESLint rule
    /// * 0 - turns the rule off
    /// * 1 - turn the rule on as a warning (doesn't affect exit code)
    /// * 2 - turn the rule on as an error (exit code is 1 when triggered)
    IntegerEnabled(u8),
    /// ESLint rule
    /// * "off" - turns the rule off
    /// * "warn" - turn the rule on as a warning (doesn't affect exit code)
    /// * "error" - turn the rule on as an error (exit code is 1 when triggered)
    StringEnabled(String),
    /// ESLint rule
    /// * "off" - turns the rule off
    /// * "warn" - turn the rule on as a warning (doesn't affect exit code)
    /// * "error" - turn the rule on as an error (exit code is 1 when triggered)
    ///
    /// The second property in the pair configures the rule, e.g., ["error", "single"] for quotes rule
    StringEnabledString(String, String),
    /// ESLint rule
    /// * 0 - turns the rule off
    /// * 1 - turn the rule on as a warning (doesn't affect exit code)
    /// * 2 - turn the rule on as an error (exit code is 1 when triggered)
    ///
    /// The second property in the pair configures the rule, e.g., [2, "single"] for quotes rule
    NumberEnabledString(u8, String),
    /// ESLint rule
    /// * "off" - turns the rule off
    /// * "warn" - turn the rule on as a warning (doesn't affect exit code)
    /// * "error" - turn the rule on as an error (exit code is 1 when triggered)
    ///
    /// The second property in the pair configures the rule, e.g., ["error", {prefer: "single"}] for quotes rule
    StringEnabledObject(String, HashMap<String, Value>),
    /// ESLint rule
    /// * 0 - turns the rule off
    /// * 1 - turn the rule on as a warning (doesn't affect exit code)
    /// * 2 - turn the rule on as an error (exit code is 1 when triggered)
    ///
    /// The second property in the pair configures the rule, e.g., [2, {prefer: "single"}] for quotes rule
    NumberEnabledObject(u8, HashMap<String, Value>),
}

#[derive(Serialize, Deserialize, Debug)]
struct RawConfigFile {
    extends: Vec<String>,
    rules: HashMap<String, Rules>,
}

impl From<PathBuf> for LintConfig {
    fn from(buf: PathBuf) -> Self {
        // If buf is default, the user did not specify a config file to load
        if buf == PathBuf::default() {
            return LintConfig::default();
        }

        // Get file extension
        let valid_extensions = ["json", "js", "cjs", "yml", "yaml"];
        let ext = buf.extension().unwrap().to_str().unwrap();

        // Check for invalid file extensions
        if !valid_extensions.contains(&ext) {
            panic!(
                "Extension {:?} is not a valid file extension. Please use one of {:?} instead.",
                ext, valid_extensions
            )
        }

        // Parse .eslintrc.js files as priority
        if ext == "js" {
            todo!()
        } else if ext == "json" {
            let file = read_to_string(buf).unwrap();
            let json: RawConfigFile = serde_json::from_str(file.as_str()).unwrap();
            println!("{:?}", json);
        } else {
            // TODO: parse other file types
            todo!()
        }

        LintConfig::default()
    }
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
