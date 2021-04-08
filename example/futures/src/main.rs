use futures::future::Future;
use tokio::prelude::*;

pub struct ChatBox{}

impl Future for ChatBox {
    type Item = String;
    type Error = ();
    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error>{
        return Ok(Async::Ready("hello".to_string())) 

    }
}
fn main() {
    let f = ChatBox{};
    let p = f.map(|s| println!("{}", s));

    println!("Begin ...");

    tokio::run(p);
    println!("End");
