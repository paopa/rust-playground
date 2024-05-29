use anyhow::Result;
use qdrant_client::client::QdrantClient;
use qdrant_client::qdrant::{vectors_config::Config, VectorParams, VectorsConfig};
use qdrant_client::qdrant::{CreateCollection, Distance};

#[tokio::main]
async fn main() -> Result<()> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    client
        .create_collection(&CreateCollection {
            collection_name: "test_collection".to_string(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 4,
                    distance: Distance::Dot.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;
}
