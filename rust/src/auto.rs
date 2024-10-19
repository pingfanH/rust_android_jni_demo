
// #[no_mangle]
// extern { 
//     pub fn rust_greeting(to: *const c_char) -> *mut c_char;
//     pub fn rust_add(to: *const c_char) -> *mut c_char;
// }
// #[macro_export]
// macro_rules! register_native_methods {
//     ($env:expr, $class:expr, { $($name:ident => $sig:expr),* $(,)? }) => {{
//         let mut methods: Vec<jni::NativeMethod> = Vec::new();
//         $(
//             methods.push(jni::NativeMethod {
//                 name: JNIString::from(stringify!($name)),
//                 sig: JNIString::from($sig),
//                 fn_ptr: $name as *mut _,
//             });
//         )*
//         $env.register_native_methods($class, &methods).unwrap();
//     }};
// }
// #[no_mangle]
// pub extern "system" fn JNI_OnLoad(vm: jni::JavaVM, _reserved: *mut std::ffi::c_void) -> jni::sys::jint {
//     let mut env = vm.get_env().expect("Cannot get reference to the JNIEnv");
//     let class_name = "top/pingfanh/jni/RustNative";
//     let class = env.find_class(class_name).unwrap();
// 
//     let methods = [
//         jni::NativeMethod {
//             name: JNIString::from("greeting"),
//             sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
//             fn_ptr: greeting as *mut _,
//         },
//         jni::NativeMethod {
//             name: JNIString::from("add"),
//             sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
//             fn_ptr: add as *mut _,
//         },
//         jni::NativeMethod {
//             name: JNIString::from("fuck"),
//             sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
//             fn_ptr: fuck as *mut _,
//         },
//     ];
//     env.register_native_methods(class, &methods).expect("Failed to register native methods");
// 
//     jni::sys::JNI_VERSION_1_6
// }
//#[no_mangle]
// pub extern "system" fn JNI_OnLoad(vm: jni::JavaVM, _reserved: *mut std::ffi::c_void) -> jni::sys::jint {
//     let mut env = vm.get_env().expect("Cannot get reference to the JNIEnv");

//     // 找到 Java 类
//     let class_name = "top/pingfanh/jni/RustNative";
//     let class = env.find_class(class_name).expect("Cannot find class");

//     // 注册的本地方法
//     let methods = [
//         jni::NativeMethod {
//             name: JNIString::from("greeting"),
//             sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
//             fn_ptr: greeting as *mut _,
//         },
//         jni::NativeMethod {
//             name: JNIString::from("add"),
//             sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
//             fn_ptr: add as *mut _,
//         },
//     ];

//     // 注册方法
//     env.register_native_methods(class, &methods).expect("Failed to register native methods");

//     jni::sys::JNI_VERSION_1_6
// }
// #[no_mangle]
// pub unsafe extern "C" fn greeting(
//     mut env: JNIEnv,
//     _class: JClass,
//     java_pattern: JString,
// ) -> jstring {
//     let world: String = env.get_string(&java_pattern).expect("invalid pattern string").into();
//     let output = env.new_string(format!("Hello, {}!", world)).expect("Couldn't create java string!");
//     output.into_raw()
// }
// #[no_mangle]
// pub unsafe extern "C" fn add(
//     mut env: JNIEnv,
//     _class: JClass,
//     java_pattern: JString,
// ) -> jstring {
//     let world: String = env.get_string(&java_pattern).expect("invalid pattern string").into();
//     let output = env.new_string(format!("{}_1", world)).expect("Couldn't create java string!");
//     output.into_raw()
// }


#[test]
fn fn_name() {
    let func_name = stringify!(add);
    println!("Function name type: {}", func_name);
}
