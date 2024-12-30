use cassandra_cpp::{Cluster, Session};

use super::config::CassandraConfig;

pub async fn connect(config: CassandraConfig) -> cassandra_cpp::Result<Session> {
    let mut cluster = Cluster::default();
    cluster.set_contact_points(&config.contact_points).unwrap();
    cluster.connect().await
}
