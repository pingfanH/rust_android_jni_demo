use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_char;

use jni::objects::*;
use jni::JNIEnv;
use jni::sys::jstring;

#[no_mangle]
pub unsafe extern "system" fn Java_top_pingfanh_jni_RustNative_greeting(
    mut env: JNIEnv,
    _class: JClass,
    java_pattern: JString,
) -> jstring {
    let world: String = env.get_string(&java_pattern).expect("invalid pattern string").into();
    let output = env.new_string(format!("Hello, {}!", world)).expect("Couldn't create java string!");
    output.into_raw()
}
#[no_mangle]
pub unsafe extern "system" fn Java_top_pingfanh_jni_RustNative_add(
    mut env: JNIEnv,
    _class: JClass,
    java_pattern: JString,
) -> jstring {
    let world: String = env.get_string(&java_pattern).expect("invalid pattern string").into();
    let output = env.new_string(format!("{}_1", world)).expect("Couldn't create java string!");
    output.into_raw()
}
