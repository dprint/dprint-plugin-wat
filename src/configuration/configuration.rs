use dprint_core::configuration::NewLineKind;
use dprint_core::configuration::ParseConfigurationError;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  pub line_width: u32,
  pub use_tabs: bool,
  pub indent_width: u8,
  pub new_line_kind: NewLineKind,
  pub split_closing_parens: bool,
  pub wrap_before_locals: WrapBefore,
  pub wrap_before_fields: WrapBefore,
  pub wrap_before_const_expr: WrapBefore,
  pub multi_line_locals: MultiLine,
  pub multi_line_fields: MultiLine,
  pub format_comments: bool,
  pub ignore_comment_directive: String,
}

macro_rules! generate_str_to_from {
  ($enum_name:ident, $([$member_name:ident, $string_value:expr]),* ) => {
    impl std::str::FromStr for $enum_name {
      type Err = ParseConfigurationError;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
          $($string_value => Ok(Self::$member_name)),*,
          _ => Err(ParseConfigurationError(String::from(s))),
        }
      }
    }

    impl std::fmt::Display for $enum_name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $(Self::$member_name => f.write_str($string_value)),*,
        }
      }
    }
  };
}

/// When to put a line break before a group of items.
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WrapBefore {
  Never,
  Overflow,
  MultiOnly,
  Always,
}

generate_str_to_from![
  WrapBefore,
  [Never, "never"],
  [Overflow, "overflow"],
  [MultiOnly, "multiOnly"],
  [Always, "always"]
];

/// How to separate multiple items of a group.
#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MultiLine {
  Never,
  Overflow,
  Smart,
  Wrap,
  Always,
}

generate_str_to_from![
  MultiLine,
  [Never, "never"],
  [Overflow, "overflow"],
  [Smart, "smart"],
  [Wrap, "wrap"],
  [Always, "always"]
];
