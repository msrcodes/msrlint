use swc_ecma_lints::config::RuleConfig;

#[derive(Default, Clone)]
pub struct LintConfig {
    pub quotes: RuleConfig<()>,
}
