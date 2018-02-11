import * as wasm from "./boxcars_web";
//import { parse_replay } from "./boxcars_web";
import { booted, memory } from "./boxcars_web_wasm";

booted.then(() => {
  window.my_memory = memory;
  window.wasm = wasm;
  window.boxcars = wasm.parse_replay;
});
