#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter,QueryString};

fn main() {
    let mut server = Nickel::new();
    server.get("/fib", middleware! { |request, _response|
        let query = request.query();

        let value = query.get("i");

        fib(value.unwrap().parse::<i64>().unwrap())
    });
    server.listen("127.0.0.1:6767");
}

fn fib(input: i64) -> String {
    let mut num = input;
    let mut a :i64= 1;
    let mut b :i64 = 0;
    let mut temp : i64 = 0;
    
    while num >= 0 {
        temp = a;
        a = a + b;
        b = temp;
        num = num - 1;
    }

    return b.to_string();
}