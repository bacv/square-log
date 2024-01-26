use std::sync::Arc;

use axum::{routing::get, Json, Router};

use crate::{
    db::{Database, Range},
    plugin::source::SourceSummary,
    record::DataRecord,
};

use super::HttpConfig;

pub struct HttpServer {
    config: HttpConfig,
    router: Router,
}

impl HttpServer {
    pub fn new<DB>(config: HttpConfig, db: Arc<DB>) -> Self
    where
        DB: Database + Send + Sync + 'static,
    {
        let router = Router::new()
            .route("/sources", get(get_sources::<DB>))
            .route("/sources/:source", get(get_source::<DB>))
            .route("/sources/:source/range", get(get_range::<DB>))
            .layer(axum::extract::Extension(db));
        Self { config, router }
    }

    pub async fn serve(self) -> color_eyre::Result<()> {
        let listener = tokio::net::TcpListener::bind(self.config.addr)
            .await
            .expect("Should be able to listen on port");
        axum::serve(listener, self.router).await.map_err(Into::into)
    }
}

async fn get_range<DB: Database>(
    axum::extract::Path(source): axum::extract::Path<String>,
    axum::extract::Query(range): axum::extract::Query<Range>,
    axum::extract::Extension(db): axum::extract::Extension<Arc<DB>>,
) -> Json<Vec<DataRecord>> {
    let result = db.get_range(&source, range);
    Json(result.unwrap_or_else(|_| Vec::new()))
}

async fn get_sources<DB: Database>(
    axum::extract::Extension(db): axum::extract::Extension<Arc<DB>>,
) -> Json<Vec<SourceSummary>> {
    let result = db.get_sources();
    Json(result.unwrap_or_else(|_| Vec::new()))
}

async fn get_source<DB: Database>(
    axum::extract::Path(source): axum::extract::Path<String>,
    axum::extract::Extension(db): axum::extract::Extension<Arc<DB>>,
) -> Json<Option<SourceSummary>> {
    let result = db.get_source(&source);
    Json(result.unwrap_or(None))
}
