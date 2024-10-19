package top.pingfanh.jni

import android.os.Bundle
import android.view.animation.AnimationUtils
import androidx.appcompat.app.AppCompatActivity
import top.pingfanh.jni.databinding.ActivityMainBinding


class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding
    private lateinit var rustNative: RustNative
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        rustNative = RustNative()
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)
        binding.sampleText.text = rustNative.greeting("ping")
        val addButton = binding.addButton
        addButton.text = rustNative.add("click")
        val moveAnimation = AnimationUtils.loadAnimation(this, R.anim.move_animation)
        addButton.setOnClickListener {
            binding.sampleText.text=rustNative.fuck1("you");
            //addButton.text = rustNative.add(addButton.text as String)
            addButton.startAnimation(moveAnimation)
        }
    }

//    external fun rust_greeting(string: String): String
//    external fun rust_add(string: String): String
//
//    companion object {
//        init {
//            System.loadLibrary("ping")
//        }
//    }
}