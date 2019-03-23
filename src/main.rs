use env_logger;
use failure::Error;
use futures::future::Future;
use lapin_futures as lapin;
use crate::lapin::channel::{BasicPublishOptions, BasicProperties, QueueDeclareOptions};
use crate::lapin::client::ConnectionOptions;
use crate::lapin::types::FieldTable;
use log::info;
use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use std::env;

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() < 2 {
    println!("arg needed: ip:port");
    return;
  }

  env_logger::init();

  //let addr = "192.168.99.100:31525".parse().unwrap();
  let addr = args[1].parse().unwrap();

  Runtime::new().unwrap().block_on_all(
    TcpStream::connect(&addr).map_err(Error::from).and_then(|stream| {

      // connect() returns a future of an AMQP Client
      // that resolves once the handshake is done
      lapin::client::Client::connect(stream, ConnectionOptions::default()).map_err(Error::from)
   }).and_then(|(client, _ /* heartbeat */)| {
      println!("connect");

      // create_channel returns a future that is resolved
      // once the channel is successfully created
      client.create_channel().map_err(Error::from)
    }).and_then(|channel| {
      let id = channel.id;
      info!("created channel with id: {}", id);
      println!("create_channel");

      // we using a "move" closure to reuse the channel
      // once the queue is declared. We could also clone
      // the channel
      channel.queue_declare("hello", QueueDeclareOptions::default(), FieldTable::new()).and_then(move |_| {
        info!("channel {} declared queue {}", id, "hello");
        println!("queue_decalre");

        channel.basic_publish("", "hello", b"hello from tokio".to_vec(), BasicPublishOptions::default(), BasicProperties::default())

      }).map_err(Error::from)
    })
  ).expect("runtime failure");
}
