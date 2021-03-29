# Experiment with http-sig

So this doesn't work as noted below and I filed
[issue/9](https://github.com/PassFort/http-signatures/issues/9)

I was hoping to use this with [Signed Endpoint Examples for POST /api/v3/order](https://github.com/binance-us/binance-official-api-docs/blob/fc916164ae04eb2e95ff7f98c2d49d8d6bd6d096/rest-api.md#signed-endpoint-examples-for-post-apiv3order)
but it Diggory Blace (@Diggsey) confirmed the unsurprising
fact that binance.us API's don't conform to the [W3C CCG](https://github.com/w3c-ccg) RFC
so, for now at least I'm not going to persue this any farther.
Although, I could see possibling using this in other projects.
But there is one caveate, the W3C CCG is working on a new spec,
so we'll have to see.

## main.rs

I've taken this from [the docs](https://docs.rs/http-sig/0.3.1/http_sig/#example-usage-reqwest):
```
use reqwest;

use http_sig::*;


fn main() -> () {

    const SECRET_KEY: &[u8] = b"secret";

    let config = SigningConfig::new_default("My Key", SECRET_KEY);

    let client = reqwest::blocking::Client::new();

    let req = client
        .get("http://localhost:8080/")
        .build()
        .unwrap()
        .signed(&config)
        .unwrap();

    println!("req={:?}", req);

    //let result = client.execute(req).unwrap();
}
```

Right now I'm getting an error:
```
wink@3900x:~/prgs/rust/projects/expr-http-sig (main)
$ cargo build
   Compiling expr-http-sig v0.1.0 (/home/wink/prgs/rust/projects/expr-http-sig)
error[E0599]: the method `signed` exists for struct `reqwest::blocking::Request`, but its trait bounds were not satisfied
  --> src/main.rs:18:10
   |
18 |         .signed(&config)
   |          ^^^^^^ method cannot be called on `reqwest::blocking::Request` due to unsatisfied trait bounds
   | 
  ::: /home/wink/.cargo/registry/src/github.com-1ecc6299db9ec823/reqwest-0.11.2/src/blocking/request.rs:20:1
   |
20 | pub struct Request {
   | ------------------
   | |
   | doesn't satisfy `reqwest::blocking::Request: ClientRequestLike`
   | doesn't satisfy `reqwest::blocking::Request: http_sig::SigningExt`
   |
   = note: the following trait bounds were not satisfied:
           `reqwest::blocking::Request: ClientRequestLike`
           which is required by `reqwest::blocking::Request: http_sig::SigningExt`
           `&reqwest::blocking::Request: ClientRequestLike`
           which is required by `&reqwest::blocking::Request: http_sig::SigningExt`
           `&mut reqwest::blocking::Request: ClientRequestLike`
           which is required by `&mut reqwest::blocking::Request: http_sig::SigningExt`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `expr-http-sig`

To learn more, run the command again with --verbose.
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
