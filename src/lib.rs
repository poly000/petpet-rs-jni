use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint};

use petpet::file_to_gif;
use petpet::FilterType;

#[no_mangle]
pub extern "system" fn GeneratePetpetToFile(env: JNIEnv,
                                            _class: JClass,
                                            input: (JString, JString, jint)) -> jboolean {
    let input: (String, String) = (env.get_string(input.0).expect("Couldn't get java string!").into(),
                                   env.get_string(input.1).expect("Couldn't get java string!").into());

    file_to_gif(input.0.into(), input.1.into(), input.2, FilterType::Lanczos3).is_ok()
}