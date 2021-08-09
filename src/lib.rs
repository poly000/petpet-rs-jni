use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jint;

use petpet::file_to_gif;
use petpet::FilterType;

#[link(name = "petpet_rs")]
#[no_mangle]
pub extern "system" fn Java_TestRustCallPerformance_Petpet_GeneratePetpetToFile(env: JNIEnv,
                                            _class: JClass,
                                            input_image: JString,
                                            output_gif: JString,
                                            speed: jint) 
{
    let input_image: String = env.get_string(input_image).expect("Cannot get java String!").into();
    let output_gif: String  = env.get_string(output_gif).expect("Cannot get java String!").into();
    file_to_gif(input_image, output_gif, speed, FilterType::Lanczos3).unwrap();
}
