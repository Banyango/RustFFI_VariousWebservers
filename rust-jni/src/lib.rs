extern crate jni;

use jni::JNIEnv;

use jni::objects::{JClass};

use jni::sys::{jstring, jlong};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_com_ffitest_ffitest_Fibonnacci_fib(
    env:JNIEnv, 
    class: JClass, 
    input: jlong) -> jstring {
    
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

    let output = env.new_string(
        format!("{}", b))
        .expect("Couldn't create the java string!");
    
    output.into_inner()
}