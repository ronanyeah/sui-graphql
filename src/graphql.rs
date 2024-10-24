use graphql_client::GraphQLQuery;
use sui_sdk::types::base_types::SuiAddress;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct TestQuery;
