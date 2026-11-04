# dprint-plugin-wat

[![npm version](https://img.shields.io/npm/v/@dprint/wat.svg)](https://www.npmjs.com/package/@dprint/wat) [![CI](https://github.com/dprint/dprint-plugin-wat/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-wat/actions?query=workflow%3ACI)

Adapter for [wat_formatter](https://github.com/g-plane/wasm-language-tools) for use as a formatting plugin in [dprint](https://github.com/dprint/dprint). It formats [WebAssembly Text Format](https://webassembly.github.io/spec/core/text/index.html) files (`.wat`).

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's directory with a dprint.json file, run:

```shellsession
dprint add wat
```

Note: You do not need wasm-language-tools installed as dprint will run wat_formatter from the .wasm file in a sandboxed environment.

## Configuration

To add configuration, specify a `"wat"` key in your dprint.json:

```jsonc
{
  "wat": {
    "lineWidth": 100,
    "indentWidth": 4,
    "splitClosingParens": true,
    "multiLineLocals": "always",
  },
  "plugins": [
    // ...etc...
  ],
}
```

The options and their defaults match [wat_formatter's](https://docs.rs/wat_formatter/latest/wat_formatter/config/), with dprint's names for the common ones:

| dprint                   | wat_formatter              | Default        |
| ------------------------ | -------------------------- | -------------- |
| `lineWidth`              | `print_width`              | `80`           |
| `useTabs`                | `use_tabs`                 | `false`        |
| `indentWidth`            | `indent_width`             | `2`            |
| `newLineKind`            | `line_break`               | `"lf"`         |
| `splitClosingParens`     | `split_closing_parens`     | `false`        |
| `wrapBeforeLocals`       | `wrap_before_locals`       | `"always"`     |
| `wrapBeforeFields`       | `wrap_before_fields`       | `"multiOnly"`  |
| `wrapBeforeConstExpr`    | `wrap_before_const_expr`   | `"always"`     |
| `multiLineLocals`        | `multi_line_locals`        | `"smart"`      |
| `multiLineFields`        | `multi_line_fields`        | `"smart"`      |
| `formatComments`         | `format_comments`          | `false`        |
| `ignoreCommentDirective` | `ignore_comment_directive` | `"fmt-ignore"` |

Option values are camelCase versions of wat_formatter's (ex. `"multi-only"` is `"multiOnly"`). Since the defaults are wat_formatter's, the global `lineWidth`, `useTabs`, `indentWidth` and `newLineKind` in your dprint.json are used when they're set and wat_formatter's defaults otherwise.

A `;; fmt-ignore` comment keeps the module or module field after it as it is, and range formatting is supported. A file with a syntax error is reported as an error rather than formatted.

For an overview of the config, see https://github.com/dprint/dprint-plugin-wat/blob/main/deployment/schema.json

## JS Formatting API

- [JS Formatter](https://github.com/dprint/js-formatter) - Browser/Deno and Node
- [npm package](https://www.npmjs.com/package/@dprint/wat)

## Versioning

This repo automatically upgrades to the latest version of wat_formatter once a day. You can check which version of wat_formatter is being used by looking at the `wat_formatter` entry in the Cargo.toml file in this repo:

https://github.com/dprint/dprint-plugin-wat/blob/main/Cargo.toml

At the moment, the version of this plugin does not reflect the version of wat_formatter.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. wat_formatter is licensed under the [MIT License](https://github.com/g-plane/wasm-language-tools/blob/main/LICENSE) as well.
