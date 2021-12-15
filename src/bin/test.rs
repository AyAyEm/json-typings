use ijson::{IValue, IObject, IArray};
use serde_json::Deserializer;

fn main() {
    let data = "{\"a\": \"a\"}{\"b\": \"b\"}";

    let stream = Deserializer::from_str(data).into_iter::<IObject>();

    for value in stream {
        println!("{:?}", value.unwrap());
    }
}
