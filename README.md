# serde-wasm-bindgen bug

## Requirements
- Node.js
- `wasm-pack`

## Running
```bash
npm install
npm start
# open localhost:1234 in browser, check console
```

## Expected Output
```
{type: 'A', value: 0n}
true
```

## Actual Output
```
{type: 'A', value: 0n}
serde_wasm_bindgen_bug.js:364 Uncaught (in promise) Error: invalid type: JsValue(BigInt), expected any value
    at imports.wbg.__wbg_new_342a24ca698edd87 (serde_wasm_bindgen_bug.js:364)
    at serde_wasm_bindgen_bug_bg.fb6a5fbf.wasm?1641509318572:0x12847
    at serde_wasm_bindgen_bug_bg.fb6a5fbf.wasm?1641509318572:0x10828
    at serde_wasm_bindgen_bug_bg.fb6a5fbf.wasm?1641509318572:0xc14d
    at serde_wasm_bindgen_bug_bg.fb6a5fbf.wasm?1641509318572:0x8ca5
    at serde_wasm_bindgen_bug_bg.fb6a5fbf.wasm?1641509318572:0x65f1
    at serde_wasm_bindgen_bug_bg.fb6a5fbf.wasm?1641509318572:0x6198
    at serde_wasm_bindgen_bug_bg.fb6a5fbf.wasm?1641509318572:0x10956
    at Object.give_value (serde_wasm_bindgen_bug.js:201)
    at main (index.js:7)
```
