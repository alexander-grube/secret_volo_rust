use sonic_rs::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct ExampleConfig {
    pub pg: deadpool_postgres::Config,
}