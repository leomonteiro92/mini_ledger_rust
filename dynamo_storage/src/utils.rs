use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{
    operation::create_table::CreateTableOutput,
    types::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client,
};

pub async fn connect() -> Result<aws_sdk_dynamodb::Client, String> {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .test_credentials()
        .endpoint_url("http://localhost:8000")
        .region(RegionProviderChain::default_provider().or_else("us-east-1"))
        .load()
        .await;

    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let client = aws_sdk_dynamodb::Client::from_conf(dynamodb_local_config);
    Ok(client)
}

pub async fn create_table(client: &Client) -> Result<CreateTableOutput, String> {
    let attribute_definitions = vec![
        AttributeDefinition::builder()
            .attribute_name("pk")
            .attribute_type(ScalarAttributeType::S)
            .build()
            .expect("Couldn't build attribute definition"),
        AttributeDefinition::builder()
            .attribute_name("sk")
            .attribute_type(ScalarAttributeType::S)
            .build()
            .expect("Couldn't build attribute definition"),
    ];

    let key_schema = vec![
        KeySchemaElement::builder()
            .attribute_name("pk")
            .key_type(KeyType::Hash)
            .build()
            .expect("Couldn't build key schema"),
        KeySchemaElement::builder()
            .attribute_name("sk")
            .key_type(KeyType::Range)
            .build()
            .expect("Couldn't build key schema"),
    ];

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build()
        .expect("Couldn't build provisioned throughput");

    return client
        .create_table()
        .table_name("mini_ledger")
        .set_attribute_definitions(Some(attribute_definitions))
        .set_key_schema(Some(key_schema))
        .provisioned_throughput(pt)
        .send()
        .await
        .map_err(|e| format!("Failed to create table: {:?}", e));
}
