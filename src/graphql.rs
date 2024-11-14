use graphql_client::GraphQLQuery;
use sui_sdk_types::types::Address;

type SuiAddress = Address;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct TestQuery;
