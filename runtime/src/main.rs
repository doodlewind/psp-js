#![no_std]
#![no_main]

use libquickjs_sys::*;

psp::module!("hello_world_module", 1, 1);

fn psp_main() {
  unsafe {
    psp::enable_home_button();

    psp::dprintln!("debug: init JSContext from Rust");
    let rt = JS_NewRuntime();
    let ctx = JS_NewContext(rt);

    let code_str = b"1 + 1\0";
    let script_str = b"script\0";
    let len = (code_str.len() - 1) as u32;
    psp::dprintln!("debug: rust side input_len: {}", len);

    let value = JS_Eval(
      ctx,
      code_str.as_ptr(),
      len,
      script_str.as_ptr(),
      JS_EVAL_TYPE_GLOBAL as i32,
    );

    psp::dprint!("Hello QuickJS! tag {}, value {}", value.tag, value.u.int32);
  }
}
