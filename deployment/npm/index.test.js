// @ts-check
const assert = require("assert");
const createFromBuffer = require("@dprint/formatter").createFromBuffer;
const getBuffer = require("./index").getBuffer;

const formatter = createFromBuffer(getBuffer());
const result = formatter.formatText({
  filePath: "file.wat",
  fileText: "( module ( func   $add ( param i32 i32 ) ( result i32 ) local.get 0 local.get 1 i32.add ) )",
});

assert.strictEqual(
  result,
  "(module\n  (func $add (param i32 i32) (result i32)\n    local.get 0\n    local.get 1\n    i32.add))\n",
);
