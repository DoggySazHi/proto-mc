use crate::rcon::*;
use super::read_config;

#[tokio::test]
async fn query() {
    let config = read_config();
    let mut client = RCONClient::<&str>::new(&config.rcon_host, &config.password);
    client.connect().await.unwrap();
    client.login().await.unwrap();
    let data = client.send("list").await.unwrap();
    assert!(data.payload.contains("players online"));
}