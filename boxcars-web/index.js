import { parse_replay, WebReplay } from "./boxcars_web";
//import { parse_replay, greet } from "./boxcars_web";
import { booted } from "./boxcars_web_wasm";

booted.then(() => {
  window.parse_replay = parse_replay;
  window.WebReplay = WebReplay;
/*  window.my_memory = memory;
  window.wasm = wasm;
  window.boxcars = wasm.parse_replay;*/
});
