# petpet-rs-jni

JNI bind for petpet-rs

resources can be downloaded from [here](https://github.com/poly000/petpet-rs/releases/download/v1.0.0/resource-any.7z) in 7z format.

## Usage

1. put [this class](java/Petpet.java) to where you like.
2. run `javac -h path/to/the/code` to generate function name.
(in this [example](java/Petpet.h), it's `Java_Petpet_GeneratePetpetToFile`)
3. change the function name in your fork.

CI will generate this library for `x86_64-unknown-linux-gnu`, `x86_64-pc-windows-msvc`, and `x86_64-apple-darwin`

You can add other targets manually in [rust.yml](,github/../.github/workflows/rust.yml)
