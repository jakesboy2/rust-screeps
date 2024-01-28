## Expose Function to JS
* Add wasm_bindgen to any function you would like to expose for call from js 
  * to use a reserved name as a function name, use `js_name`:
  * ex: #[wasm_bindgen(js_name = loop)]