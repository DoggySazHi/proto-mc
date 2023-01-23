use crate::ping::*;
use super::read_config;

#[tokio::test]
async fn read() {
    let config = read_config();
    ping(config.host).await.expect("Failed to ping");
}