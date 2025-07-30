use sui_rpc::proto::sui::rpc::v2beta2::{
    SubscribeCheckpointsRequest, SubscribeCheckpointsResponse,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = sui_rpc::Client::new("https://fullnode.mainnet.sui.io:443")?;

    let mut sub = create_checkpoint_stream(&mut client).await?;
    println!("created");
    while let Some(chk) = sub.next().await {
        let val = chk.unwrap();
        let ts = val
            .checkpoint
            .unwrap()
            .summary
            .unwrap()
            .timestamp
            .unwrap()
            .to_string();
        println!("{}", ts);
    }
    Ok(())
}

async fn create_checkpoint_stream(
    client: &mut sui_rpc::Client,
) -> anyhow::Result<
    impl tokio_stream::Stream<Item = Result<SubscribeCheckpointsResponse, tonic::Status>>,
> {
    let mut subscription_client = client.subscription_client();

    let request = tonic::Request::new(SubscribeCheckpointsRequest {
        read_mask: Some(sui_rpc::field::FieldMask {
            paths: vec!["summary.timestamp".to_string()],
        }),
    });

    let stream = subscription_client
        .subscribe_checkpoints(request)
        .await?
        .into_inner();

    Ok(stream)
}
