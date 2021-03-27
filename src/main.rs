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
