class Petpet {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native void GeneratePetpetToFile(String image, String gif, int speed);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("petpet_jni");
    }
}
