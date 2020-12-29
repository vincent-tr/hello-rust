mod mqtt;

use mqtt::MqttClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  let client = MqttClient::new();

  Ok(())
}
