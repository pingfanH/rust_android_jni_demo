plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "top.pingfanh.jni"
    compileSdk = 34

    defaultConfig {
        applicationId = "top.pingfanh.jni"
        minSdk = 24
        targetSdk = 34
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
        debug {

        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        viewBinding = true
    }
    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("jniLibs")
        }
    }
}

//tasks.named("testReleaseUnitTest") {
//    finalizedBy("cargoBuildDebug")
//}
tasks.register("cargoBuild") {
    description = "cargo build"

    doLast {
        println("Cargo Building")
        exec {
            workingDir = file("../rust")
            if (project.gradle.startParameter.taskNames.any { it.contains("Debug") }) {
                commandLine("cargo","ndk","build")
                //commandLine("cargo","build","--target","x86_64-linux-android")
            } else {
                commandLine("cargo","ndk","build","--release")
                //commandLine("cargo","build","--target","x86_64-linux-android","--release")
            }

        }
    }
}
tasks.register<Copy>("moveLibsArm64") {
    val buildType = if (project.gradle.startParameter.taskNames.any { it.contains("Debug") }) {
        "debug"
    } else {
        "release"
    }
    from("../rust/target/aarch64-linux-android/${buildType}/libping.so") // 源目录
    into("jniLibs/arm64-v8a") // 目标目录
}

// 定义用于复制 armeabi-v7a 的任务
tasks.register<Copy>("moveLibsArmv7") {
    val buildType = if (project.gradle.startParameter.taskNames.any { it.contains("Debug") }) {
        "debug"
    } else {
        "release"
    }
    from("../rust/target/armv7-linux-androideabi/${buildType}/libping.so") // 源目录
    into("jniLibs/armeabi-v7a") // 目标目录
}
tasks.register<Copy>("moveLibsx86_64") {
    val buildType = if (project.gradle.startParameter.taskNames.any { it.contains("Debug") }) {
        "debug"
    } else {
        "release"
    }
    from("rust/target/i686-linux-android/${buildType}/libping.so") // 源目录
    into("app/jniLibs/x86_64") // 目标目录
}
tasks.named("preBuild") {
    dependsOn("cargoBuild")
    dependsOn("moveLibsArm64")
    dependsOn("moveLibsArmv7")
}

dependencies {

    implementation("androidx.core:core-ktx:1.12.0")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("com.google.android.material:material:1.11.0")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.1")
}