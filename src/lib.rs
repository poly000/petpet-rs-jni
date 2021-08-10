use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jint,jstring};

use petpet::file_to_gif;
use petpet::FilterType;

/// returns an empty string if there is no errors
#[no_mangle]
#[link(name = "petpet_rs")]
pub extern "system" fn Java_TestRustCallPerformance_Petpet_GeneratePetpetToFile(env: JNIEnv,
                                            _class: JClass,
                                            input_image: JString,
                                            output_gif: JString,
                                            speed: jint) -> jstring
{
    let input_image: String = env.get_string(input_image).expect("Cannot get java String!").into();
    let output_gif: String  = env.get_string(output_gif).expect("Cannot get java String!").into();
    if let Err(e) = file_to_gif(input_image, output_gif, speed, FilterType::Lanczos3) {
        env
        .new_string(e.to_string())
        .expect("Couldn't create java string!")
        .into_inner()
    } else {
        env
        .new_string("")
        .expect("Couldn't create java string!")
        .into_inner()
    }
}
