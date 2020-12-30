use tokio::{net::TcpStream, task::JoinHandle};

pub struct MqttClient {
  task: Option<JoinHandle<()>>,
  stream: Option<TcpStream>,
}

impl MqttClient {
  pub fn new() -> MqttClient {
    let mut client =  MqttClient {task: None, stream: None};
    /*
    client.task = Some(tokio::spawn(async {
      client.do_loop().await;
    }));
    */

    client
  }

  async fn do_loop(&mut self) {
    //let mut stream = TcpStream::connect("127.0.0.1:1883").await?;
  }
}

impl Drop for MqttClient {
  fn drop(&mut self) {
    //if( let Some(stream) = self.stream)) {
    //}
  }
}

/*

https://github.com/fluffysquirrels/mqtt-async-client-rs/blob/master/src/client/client.rs

  let mut stream = TcpStream::connect("127.0.0.1:1883").await?;
  let mut framed = Framed::new(stream, MQTTCodec::new());

*/