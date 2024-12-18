use graphql_client::GraphQLQuery;
use std::str::FromStr;
use sui_sdk_types::types::Address;
use this::graphql;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pkg = "0x9cde6fd22c9518820644dd1350ac1595bb23751033d247465ff3c7572d9a7049";

    let client = reqwest::Client::new();

    let vs = graphql::test_call::Variables {
        package_address: Address::from_str(pkg)?,
    };
    let request_body = graphql::TestCall::build_query(vs);

    let res: graphql_client::Response<graphql::test_call::ResponseData> = client
        .post("https://sui-mainnet.mystenlabs.com/graphql")
        .json(&request_body)
        .send()
        .await?
        .json()
        .await?;

    let Some(body) = res.data else {
        eprintln!("{:#?}", res.errors);
        return Ok(());
    };

    println!("{}", body.chain_identifier);
    println!("{:?}", body.latest_package);

    Ok(())
}
