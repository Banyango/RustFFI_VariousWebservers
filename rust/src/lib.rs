
use std::ffi::CString;

#[no_mangle]
pub extern fn fib(input: i64) -> CString {
    
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

    return CString::new(b.to_string()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::fib;
    use std::ffi::CString;

    #[test]
    fn test_fib() {
        println!("{}",fib(60).into_string().unwrap());
        assert_eq!(CString::new("2").unwrap(), fib(2));
    }
}
