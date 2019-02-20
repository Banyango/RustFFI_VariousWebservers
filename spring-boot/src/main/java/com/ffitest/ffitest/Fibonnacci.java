package com.ffitest.ffitest;

public class Fibonnacci {

    public static native String fib(long input);

    static {
        System.loadLibrary("ffitest");
    }

}
