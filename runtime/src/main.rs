#![no_std]
#![no_main]

use libquickjs_sys::*;

psp::module!("hello_world_module", 1, 1);

fn psp_main() {
  unsafe {
    psp::enable_home_button();

    let rt = JS_NewRuntime();
    let ctx = JS_NewContext(rt);

    let code_str = include_str!("./test.js");
    psp::dprintln!("debug: code len {}", code_str.len());

    let value = JS_Eval(
      ctx,
      code_str.as_ptr(),
      code_str.len() - 1,
      b"script\0".as_ptr(),
      JS_EVAL_TYPE_GLOBAL as i32,
    );

    psp::dprintln!("Hello QuickJS! value {}", value);
  }
}
