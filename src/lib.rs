pub struct Message<'a> {
    pub contents: &'a [u8],
}

impl Message<'_> {
    pub fn build(buf: &[u8]) -> Result<Message, &'static str> {
        let contents = buf;

        Ok(Message { contents })
    }
}

pub struct Producer {
    pub name: String,
    pub destination: &Vec<Message<'a>>,
}

pub struct Consumer <`a> {
    pub name: String,
    pub source: &Vec<Message<'a>,
}

pub struct Transport {
    pub name: String,
    pub producer: Producer,
    pub consumer: Consumer,
    buffer: Vec<Message<'a>>
}

impl Producer {

    pub fn build(name: String, destination: &Vec<Message>) -> Result<Producer, &'static str> {
        Ok(Producer { name,  destination})
    }

    pub fn produce(&self, message: &Message<'a>) {
        self.destination.push(message);
    }

    pub fn set_destination(&self, destination: &Vec<Message>) {
        self.destination = destination;
    }
}

impl Consumer {

    pub fn build(name: String, source: &Vec<Message>) -> Result<Consumer, &'static str> {
        Ok(Consumer { name, source })
    }

    pub fn consume(&self) -> Option<Message> {
        self.source.pop()

    }

    pub fn set_source(&self, source: &Vec<Message<'a>>) {
        self.source = source;
    }
}

impl Transport {

    pub fn build(name: String, producer: Producer, consumer: Consumer) -> Result<Transport, &'static str> {
        let buffer: Vec<Message<'a>> = Vec::new();
        producer.set_destination(&buffer);
        consumer.set_source(&buffer);
        Ok(Transport { name, producer, consumer, buffer })
    }

    pub fn get_consumer(&self) -> Option<Consumer> {
        Some(self.consumer)
    }

    pub fn get_producer(&self) -> Option<Producer> {
        Some(self.producer)
    }

}


#[cfg(test)]
mod tests {
    
    use super::*;


    #[test]
    fn test_message() {

        let contents = "This is a test message".as_bytes();

        let message = match Message::build(contents) {
            Ok(m) => m,
            Err(_) => panic!("Could not build message"),
        };

        let message_out = match std::str::from_utf8(message.contents) {
            Ok(b) => b,
            Err(_) => panic!("Could not get contents from message"),
        };

        assert!(contents == message_out.as_bytes());
    }
}
