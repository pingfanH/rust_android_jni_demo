// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {
    id("com.android.application") version "8.2.2" apply false
    id("org.jetbrains.kotlin.android") version "1.9.22" apply false
    //id("org.mozilla.rust-android-gradle.rust-android") version "0.9.4"
}

//apply(plugin = "org.mozilla.rust-android-gradle.rust-android")
//cargo{
//    module="./rust"
//    libname = "ping"
//
//}