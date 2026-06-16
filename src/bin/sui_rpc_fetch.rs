#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = sui_rpc::Client::new("https://fullnode.mainnet.sui.io:443")?;

    let mut ledg = client.ledger_client();

    let bridge_obj = ledg
        .get_object(
            sui_rpc::proto::sui::rpc::v2::GetObjectRequest::default()
                .with_object_id(
                    "0xde6d67a0d944b1cc6f8a6435cfac4d6eca3957542d1330d32fcdb705a0464cb2"
                        .to_string(),
                )
                .with_read_mask(sui_rpc::field::FieldMask {
                    paths: vec!["bcs".to_string()],
                }),
        )
        .await?;

    println!("object: {}", bridge_obj.get_ref().object().object_id());

    let bcs_bts = bridge_obj
        .get_ref()
        .object_opt()
        .unwrap()
        .bcs_opt()
        .unwrap()
        .value_opt()
        .unwrap();

    println!("bcs len: {}", bcs_bts.len());

    Ok(())
}
