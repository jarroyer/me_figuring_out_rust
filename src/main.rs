use messenger::Message;

fn main() {
    let contents = "Hello, world!".as_bytes();

    let message = match Message::build(contents) {
        Ok(m) => m,
        Err(m) => panic!("Could not save message {}", m),
    };

    let message_out = match std::str::from_utf8(message.contents) {
        Ok(b) => b,
        Err(b) => panic!("Invalid UTF-8 sequence: {}", b),
    };

    println!("{}", message_out);
}
