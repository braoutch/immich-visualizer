use openapi::models;
use std::env;
use immich_visualizer::*;

// #[test]
#[tokio::test]
async fn get_random_assets() {
    let api_key = env::var("IMMICH_API_KEY").expect("Define the api key in environment variable IMMICH_API_KEY");
    let client = immich::ApiClient::new(
        "http://192.168.50.214:2283/api".to_string(),
        api_key.to_string(),
        false,
    );

    let count: usize = 30;

    // let random_asset: Vec<models::AssetResponseDto> = futures::executor::block_on(client.get_random_asset(count)).expect("random asset crashed");
    let random_assets: Vec<models::AssetResponseDto> = client.get_random_asset(count.try_into().unwrap()).await.expect("random asset crashed");

    assert_eq!(random_assets.len(), count);
}