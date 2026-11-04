extern crate dprint_development;
extern crate dprint_plugin_wat;

use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_wat::configuration::Configuration;
use dprint_plugin_wat::configuration::resolve_config;
use dprint_plugin_wat::*;

#[test]
fn test_specs() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.wat",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      Arc::new(move |file_path, file_text, _range, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve_config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        format_text(file_path, file_text, &config_result.config).map_err(|err| err.into())
      })
    },
    Arc::new(move |_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  )
}

#[test]
fn should_fail_on_parse_error() {
  let err = format_text(&PathBuf::from("./file.wat"), "(module\n  (func\n", &default_config()).unwrap_err();
  assert_eq!(err.to_string(), "Syntax error at line 3, column 1: expected `)`");
}

#[test]
fn should_report_invalid_option_values() {
  let mut config = ConfigKeyMap::new();
  config.insert(
    "wrapBeforeLocals".to_string(),
    ConfigKeyValue::String("multi-only".to_string()),
  );
  config.insert(
    "multiLineFields".to_string(),
    ConfigKeyValue::String("sometimes".to_string()),
  );
  let result = resolve_config(config, &GlobalConfiguration::default());
  let property_names = result
    .diagnostics
    .iter()
    .map(|d| d.property_name.as_str())
    .collect::<Vec<_>>();
  assert_eq!(property_names, ["wrapBeforeLocals", "multiLineFields"]);
}

#[test]
fn should_use_crlf_when_configured() {
  let mut config = default_config();
  config.new_line_kind = NewLineKind::CarriageReturnLineFeed;
  let result = format_text(&PathBuf::from("./file.wat"), "(module\n(func))\n", &config).unwrap();
  assert_eq!(result.unwrap(), "(module\r\n  (func))\r\n");
}

#[test]
fn should_keep_crlf_with_auto() {
  let mut config = default_config();
  config.new_line_kind = NewLineKind::Auto;
  let result = format_text(&PathBuf::from("./file.wat"), "(module\r\n(func))\r\n", &config).unwrap();
  assert_eq!(result.unwrap(), "(module\r\n  (func))\r\n");
}

#[test]
fn should_format_range() {
  let text = "(module\n  (func   $a)\n  (func   $b)\n  (func   $c)\n)\n";
  let start = text.find("(func   $b").unwrap();
  let result = format_text_range(&PathBuf::from("./file.wat"), text, start..start + 5, &default_config()).unwrap();
  assert_eq!(
    result.unwrap(),
    "(module\n  (func   $a)\n  (func $b)\n  (func   $c)\n)\n"
  );
}

#[test]
fn should_indent_a_range_from_its_column() {
  let text =
    "(module\n\t(func $a (param i32) (param i32) (param i32) (param i32) (param i32) (result i32) (local i32))\n)\n";
  let start = text.find("(func").unwrap();
  let mut config = default_config();
  config.use_tabs = true;
  config.indent_width = 4;
  let result = format_text_range(&PathBuf::from("./file.wat"), text, start..start + 5, &config).unwrap();
  assert_eq!(
    result.unwrap(),
    "(module\n\t(func $a\n\t\t(param i32) (param i32) (param i32) (param i32) (param i32) (result i32)\n\t\t(local i32))\n)\n"
  );
}

#[test]
fn should_return_none_for_formatted_range() {
  let text = "(module\n  (func   $a)\n  (func $b)\n)\n";
  let start = text.find("(func $b").unwrap();
  let result = format_text_range(&PathBuf::from("./file.wat"), text, start..start + 5, &default_config()).unwrap();
  assert!(result.is_none());
}

#[test]
fn should_fail_on_parse_error_in_range() {
  let err = format_text_range(&PathBuf::from("./file.wat"), "(module (func)", 8..14, &default_config()).unwrap_err();
  assert_eq!(err.to_string(), "Syntax error at line 1, column 15: expected `)`");
}

#[test]
fn should_have_schema_defaults_matching_resolved_config() {
  let schema: serde_json::Value = serde_json::from_str(include_str!("../deployment/schema.json")).unwrap();
  let properties = schema["properties"].as_object().unwrap();
  let config = serde_json::to_value(default_config()).unwrap();
  let config = config.as_object().unwrap();
  assert_eq!(properties.len(), config.len());
  for (key, value) in config {
    assert_eq!(&properties[key]["default"], value, "default of {key}");
  }
}

fn default_config() -> Configuration {
  resolve_config(Default::default(), &GlobalConfiguration::default()).config
}
