use crate::mysql::query::{SchemaQueryBuilder, ColumnQueryResult, ConstraintQueryResult, IndexQueryResult, VersionQueryResult};
use crate::debug_print;
use super::*;

pub struct SchemaDiscovery {
    pub query: SchemaQueryBuilder,
    pub executor: Executor,
}

impl SchemaDiscovery {
    pub fn new<E>(executor: E) -> Self where E: IntoExecutor {
        Self {
            query: SchemaQueryBuilder::default(),
            executor: executor.into_executor(),
        }
    }

    pub async fn discover(&mut self) {
        self.query = SchemaQueryBuilder::new(self.discover_system().await);
    }

    pub async fn discover_system(&mut self) -> SystemInfo {
        let rows = self.executor.fetch_all(self.query.query_version()).await;

        for row in rows.iter() {
            let result: VersionQueryResult = row.into();
            debug_print!("{:?}", result);
            let version = result.parse();
            debug_print!("{:?}", version);
            debug_print!();
            return version;
        }
        panic!("failed to discover version")
    }
}
