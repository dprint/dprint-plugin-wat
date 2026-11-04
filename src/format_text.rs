use std::ops::Range;
use std::path::Path;

use dprint_core::configuration::resolve_new_line_kind;
use wat_syntax::AmberNode;
use wat_syntax::TextRange;
use wat_syntax::TextSize;

use crate::configuration::Configuration;
use crate::configuration::MultiLine;
use crate::configuration::WrapBefore;

/// Error that occurred while formatting WebAssembly text.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
  /// The text could not be parsed. wat_formatter can format a tree with syntax errors,
  /// but that would move code around rather than report what's wrong with it.
  #[error("Syntax error at line {line}, column {column}: {message}")]
  Syntax {
    line: usize,
    column: usize,
    message: String,
  },
  /// The byte range to format did not fall on character boundaries.
  #[error("The range {start}..{end} is not on character boundaries.")]
  InvalidRange { start: usize, end: usize },
}

pub fn format_text(file_path: &Path, input_text: &str, config: &Configuration) -> Result<Option<String>, FormatError> {
  format(file_path, input_text, None, config)
}

/// Formats the smallest syntax node that contains the provided byte range, leaving the rest of the text as it was.
pub fn format_text_range(
  file_path: &Path,
  input_text: &str,
  range: Range<usize>,
  config: &Configuration,
) -> Result<Option<String>, FormatError> {
  format(file_path, input_text, Some(range), config)
}

fn format(
  _file_path: &Path,
  input_text: &str,
  range: Option<Range<usize>>,
  config: &Configuration,
) -> Result<Option<String>, FormatError> {
  let (root, errors) = wat_parser::parse(input_text);
  if let Some(error) = errors.first() {
    let (line, column) = line_and_column(input_text, error.range.start().into());
    return Err(FormatError::Syntax {
      line,
      column,
      message: error.message.to_string(),
    });
  }

  let options = build_options(input_text, config);
  let formatted = match range {
    Some(range) => {
      let range = text_range(input_text, range)?;
      let mut node = AmberNode::new_root(&root);
      while let Some(child) = node.child_at_range(range) {
        node = child;
      }
      let node_range = node.text_range();
      let node_start: usize = node_range.start().into();
      let base_indent = visual_column(&input_text[..node_start], config.indent_width as usize);
      let mut text = String::with_capacity(input_text.len());
      text.push_str(&input_text[..node_start]);
      text.push_str(&wat_formatter::format_node(node, &options, base_indent));
      text.push_str(&input_text[node_range.end().into()..]);
      text
    }
    None => wat_formatter::format(&root, &options),
  };
  if formatted == input_text {
    Ok(None)
  } else {
    Ok(Some(formatted))
  }
}

fn build_options(input_text: &str, config: &Configuration) -> wat_formatter::config::FormatOptions {
  wat_formatter::config::FormatOptions {
    layout: wat_formatter::config::LayoutOptions {
      print_width: config.line_width as usize,
      indent_width: config.indent_width as usize,
      line_break: if resolve_new_line_kind(input_text, config.new_line_kind) == "\n" {
        wat_formatter::config::LineBreak::Lf
      } else {
        wat_formatter::config::LineBreak::Crlf
      },
      use_tabs: config.use_tabs,
    },
    language: wat_formatter::config::LanguageOptions {
      split_closing_parens: config.split_closing_parens,
      wrap_before_locals: wrap_before(config.wrap_before_locals),
      wrap_before_fields: wrap_before(config.wrap_before_fields),
      wrap_before_const_expr: wrap_before(config.wrap_before_const_expr),
      multi_line_locals: multi_line(config.multi_line_locals),
      multi_line_fields: multi_line(config.multi_line_fields),
      format_comments: config.format_comments,
      ignore_comment_directive: config.ignore_comment_directive.clone(),
    },
  }
}

fn wrap_before(value: WrapBefore) -> wat_formatter::config::WrapBefore {
  match value {
    WrapBefore::Never => wat_formatter::config::WrapBefore::Never,
    WrapBefore::Overflow => wat_formatter::config::WrapBefore::Overflow,
    WrapBefore::MultiOnly => wat_formatter::config::WrapBefore::MultiOnly,
    WrapBefore::Always => wat_formatter::config::WrapBefore::Always,
  }
}

fn multi_line(value: MultiLine) -> wat_formatter::config::MultiLine {
  match value {
    MultiLine::Never => wat_formatter::config::MultiLine::Never,
    MultiLine::Overflow => wat_formatter::config::MultiLine::Overflow,
    MultiLine::Smart => wat_formatter::config::MultiLine::Smart,
    MultiLine::Wrap => wat_formatter::config::MultiLine::Wrap,
    MultiLine::Always => wat_formatter::config::MultiLine::Always,
  }
}

fn text_range(input_text: &str, range: Range<usize>) -> Result<TextRange, FormatError> {
  let is_valid = range.start <= range.end
    && input_text.is_char_boundary(range.start)
    && input_text.is_char_boundary(range.end)
    && u32::try_from(range.end).is_ok();
  if !is_valid {
    return Err(FormatError::InvalidRange {
      start: range.start,
      end: range.end,
    });
  }
  Ok(TextRange::new(
    TextSize::from(range.start as u32),
    TextSize::from(range.end as u32),
  ))
}

/// The column the text after this text would start at, counting a tab as an indent.
fn visual_column(text_before: &str, indent_width: usize) -> usize {
  let line_start = text_before.rfind('\n').map(|i| i + 1).unwrap_or(0);
  text_before[line_start..]
    .chars()
    .map(|c| if c == '\t' { indent_width } else { 1 })
    .sum()
}

fn line_and_column(text: &str, offset: usize) -> (usize, usize) {
  let text_before = &text[..offset.min(text.len())];
  let line = text_before.matches('\n').count() + 1;
  let line_start = text_before.rfind('\n').map(|i| i + 1).unwrap_or(0);
  let column = text_before[line_start..].chars().count() + 1;
  (line, column)
}
