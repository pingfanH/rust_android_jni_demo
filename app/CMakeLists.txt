# 设置 CMake 最小版本
cmake_minimum_required(VERSION 3.22.1)

# 项目名称
project("jnitest")

# Rust 库路径（相对路径）
set(RUST_LIB_PATH "${CMAKE_SOURCE_DIR}/src/main/rust")

# 添加自定义命令构建 Rust 库
add_custom_command(
        OUTPUT ${RUST_LIB_PATH}/target/aarch64-linux-android/debug/libping.so
        COMMAND cargo ndk build --release
        WORKING_DIRECTORY ${RUST_LIB_PATH}
        COMMENT "Building Rust library with Cargo"
        VERBATIM
)

# 导入 Rust 库
add_library(ping SHARED IMPORTED)
set_target_properties(ping PROPERTIES IMPORTED_LOCATION
        ${RUST_LIB_PATH}/target/aarch64-linux-android/debug/libping.so)

# 链接库（使用 INTERFACE 链接 android 和 log 库）
target_link_libraries(ping INTERFACE
        android
        log)