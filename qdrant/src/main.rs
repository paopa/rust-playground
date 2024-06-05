use qdrant_client::{
    client::{Payload, QdrantClient},
    qdrant::{
        vectors_config::Config, Condition, CreateCollection, Distance, Filter, PointStruct,
        SearchPoints, VectorParams, VectorsConfig,
    },
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = QdrantClient::from_url("http://localhost:6334").build()?;

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(&CreateCollection {
            collection_name: collection_name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 10,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);

    let payload: Payload = json!(
        {
            "foo": "Bar",
            "bar": 12,
            "baz": {
                "qux": "quux"
            }
        }
    )
    .try_into()
    .unwrap();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
    client
        .upsert_points_blocking(collection_name, None, points, None)
        .await?;

    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![11.; 10],
            filter: Some(Filter::all([Condition::matches("bar", 12)])),
            limit: 10,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;
    dbg!(&search_result);

    let found_point = search_result.result.into_iter().next().unwrap();
    let mut payload = found_point.payload;

    let baz_payload = payload.remove("baz").unwrap().into_json();
    println!("baz: {}", baz_payload);

    let bar_payload = payload.remove("bar").unwrap().into_json();
    println!("bar: {}", bar_payload);

    let foo_payload = payload.remove("foo").unwrap().into_json();
    println!("foo: {}", foo_payload);
    Ok(())
}
