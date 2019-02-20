#[macro_use]
extern crate neon;

use neon::prelude::*;

fn fibaa(mut cx: FunctionContext) -> JsResult<JsString> {
    
    let arg0 = cx.argument::<JsNumber>(0)?.value();
    
    Ok(cx.string(fib(arg0 as i64)))
}

register_module!(mut cx, {
    cx.export_function("fib", fibaa)
});

fn fib(input: i64) -> String {
    let mut num = input;
    let mut a :i64= 1;
    let mut b :i64 = 0;
    let mut temp : i64 = 0;
    
    while num >= 0 {
        temp = a;
        a = a + b;
        b = temp;
        num = num - 1;
    }

    return b.to_string();
}