# @dprint/wat

npm distribution of [dprint-plugin-wat](https://github.com/dprint/dprint-plugin-wat) which is an adapter plugin for [wat_formatter](https://github.com/g-plane/wasm-language-tools).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-wat Wasm file](https://github.com/dprint/dprint-plugin-wat/releases).

## Example

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getBuffer } from "@dprint/wat";

const formatter = createFromBuffer(getBuffer());

console.log(
  formatter.formatText({
    filePath: "file.wat",
    fileText: "( module )",
  }),
);
```
