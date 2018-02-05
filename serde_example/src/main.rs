extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct MSG {
    from: String,
    to: String,
    message: String,
}

fn main() {
    let new = MSG {
        from: "me".to_string(),
        to: "you".to_string(),
        message: "hello".to_string(),
    };
    // Serialize it to a JSON string.
    let j = serde_json::to_string(&new).unwrap();

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);
}