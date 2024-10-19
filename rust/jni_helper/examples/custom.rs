extern crate jni_helper;

use jni_helper::custom;
//include!("examples/kt_jni.rs");
//include the kt_jni.rs in your lib
fn main() {
    let kt_path = "RustNative.kt"; // Kotlin File Path
    custom(kt_path,vec!["your/kotlinc/path/kotlinc.bat","-d","classes/kt"],"src/kt_jni_1.rs");
}