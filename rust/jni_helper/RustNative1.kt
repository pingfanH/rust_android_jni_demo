package top.pingfanh.jni
class RustNative {
    companion object {
        init {
            System.loadLibrary("ping") // 加载你的 JNI 库
        }
    }

    external fun greeting(string: String): String
    external fun add(string: String): String
    external fun hi(string: String): String
}