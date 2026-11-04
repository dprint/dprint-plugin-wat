use super::Configuration;
use super::MultiLine;
use super::WrapBefore;
use dprint_core::configuration::*;

/// Resolves configuration from a collection of key value strings.
///
/// # Example
///
/// ```
/// use dprint_core::configuration::ConfigKeyMap;
/// use dprint_core::configuration::resolve_global_config;
/// use dprint_plugin_wat::configuration::resolve_config;
///
/// let mut config_map = ConfigKeyMap::new(); // get a collection of key value pairs from somewhere
/// let global_config_result = resolve_global_config(&mut config_map);
///
/// // check global_config_result.diagnostics here...
///
/// let config_map = ConfigKeyMap::new(); // get a collection of k/v pairs from somewhere
/// let config_result = resolve_config(
///     config_map,
///     &global_config_result.config
/// );
///
/// // check config_result.diagnostics here and use config_result.config
/// ```
pub fn resolve_config(
  config: ConfigKeyMap,
  global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();
  let mut config = config;

  // the defaults are wat_formatter's rather than dprint's recommended ones
  let resolved_config = Configuration {
    line_width: get_value(
      &mut config,
      "lineWidth",
      global_config.line_width.unwrap_or(80),
      &mut diagnostics,
    ),
    use_tabs: get_value(
      &mut config,
      "useTabs",
      global_config.use_tabs.unwrap_or(false),
      &mut diagnostics,
    ),
    indent_width: get_value(
      &mut config,
      "indentWidth",
      global_config.indent_width.unwrap_or(2),
      &mut diagnostics,
    ),
    new_line_kind: get_value(
      &mut config,
      "newLineKind",
      global_config.new_line_kind.unwrap_or(NewLineKind::LineFeed),
      &mut diagnostics,
    ),
    split_closing_parens: get_value(&mut config, "splitClosingParens", false, &mut diagnostics),
    wrap_before_locals: get_value(&mut config, "wrapBeforeLocals", WrapBefore::Always, &mut diagnostics),
    wrap_before_fields: get_value(&mut config, "wrapBeforeFields", WrapBefore::MultiOnly, &mut diagnostics),
    wrap_before_const_expr: get_value(&mut config, "wrapBeforeConstExpr", WrapBefore::Always, &mut diagnostics),
    multi_line_locals: get_value(&mut config, "multiLineLocals", MultiLine::Smart, &mut diagnostics),
    multi_line_fields: get_value(&mut config, "multiLineFields", MultiLine::Smart, &mut diagnostics),
    format_comments: get_value(&mut config, "formatComments", false, &mut diagnostics),
    ignore_comment_directive: get_value(
      &mut config,
      "ignoreCommentDirective",
      "fmt-ignore".to_string(),
      &mut diagnostics,
    ),
  };

  diagnostics.extend(get_unknown_property_diagnostics(config));

  ResolveConfigurationResult {
    config: resolved_config,
    diagnostics,
  }
}
