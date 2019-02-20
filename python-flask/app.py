from flask import Flask, request
from cffi import FFI


ffi = FFI()
ffi.cdef("""
    char* fib(int);
""")

C = ffi.dlopen("../rust/target/release/libffitest.dylib")

application = Flask(__name__)

@application.route("/fib")
def fib():
    num = int(request.args.get('i', 0))
    a = 1
    b = 0
    temp = 0
    while num >= 0:
        temp = a
        a = a + b
        b = temp
        num-=1
    return str(b)

@application.route("/fib-opt")
def fibopt():
    c_str = C.fib(int(request.args.get('i',0)))
    return ffi.string(c_str).decode("UTF-8")

if __name__ == "__main__":
    application.run(host='0.0.0.0')