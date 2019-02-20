# Rust FFI Test

## Testing binding Rust code to other webservers through FFI

Note: Just me messing around...

I ran a few tests to see what it would be like to bind to rust code in various servers/languages and then ran WRK on them to see if there was a noticable performance boost.

For the most part the performance was the same. 

I think the reason being that the aglorithm that I chose (o(n) fibonacci) was not taxing enough. I think it might be better to retry the performance testing with something a bit more complicated.

As a little test I think this more morphed into just playing around with FFI in other languages and seeing how hard it was to implement them. 

Also just for kicks I implemented the algorithm in pure rust using nickel webserver. It was interesting because this was by far the fastest web server I tested. 40K req/s vs next best spring boot after warming up ~20K. 

( I seem to recall seeing a node.js setting somewhere that can boost the performance so maybe node is more capable than I found. )



