use sui_rpc::proto::sui::rpc::v2::{SubscribeCheckpointsRequest, SubscribeCheckpointsResponse};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = sui_rpc::Client::new("https://fullnode.mainnet.sui.io:443")?;

    let mut stream = create_checkpoint_stream(&mut client).await?;
    while let Some(chk) = stream.next().await {
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

    let request =
        SubscribeCheckpointsRequest::default().with_read_mask(sui_rpc::field::FieldMask {
            paths: vec!["summary".to_string()],
        });

    let stream = subscription_client
        .subscribe_checkpoints(request)
        .await?
        .into_inner();

    Ok(stream)
}
