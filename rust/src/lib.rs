//mod ui;
//mod auto;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_char;

use jni::objects::*;
use jni::JNIEnv;
use jni::sys::jstring;
use jni::strings::JNIString;
//use crate::ui::MyApp;
include!("jni.rs");

#[no_mangle]
pub unsafe extern "C" fn greeting(
    mut env: JNIEnv,
    _class: JClass,
    java_pattern: JString,
) -> jstring {
    let world: String = env.get_string(&java_pattern).expect("invalid pattern string").into();
    let output = env.new_string(format!("Hello, {}!", world)).expect("Couldn't create java string!");
    output.into_raw()
}
#[no_mangle]
pub unsafe extern "C" fn add(
    mut env: JNIEnv,
    _class: JClass,
    java_pattern: JString,
) -> jstring {
    let world: String = env.get_string(&java_pattern).expect("invalid pattern string").into();
    let output = env.new_string(format!("{}_1", world)).expect("Couldn't create java string!");
    output.into_raw()
}
#[no_mangle]
pub unsafe extern "C" fn fuck(
    mut env: JNIEnv,
    _class: JClass,
    java_pattern: JString,
) -> jstring {
    let world: String = env.get_string(&java_pattern).expect("invalid pattern string").into();
    let output = env.new_string(format!("fuck {}", world)).expect("Couldn't create java string!");
    output.into_raw()
}
// #[no_mangle]
// pub unsafe extern "system" fn egui(
//     mut env: JNIEnv,
//     _class: JClass,
// ){
//     let options = eframe::NativeOptions::default();
//     eframe::run_native(
//         "egui demo",
//         options,
//         Box::new(|_cc| Ok(Box::new(MyApp::default()))),
//     );
// }