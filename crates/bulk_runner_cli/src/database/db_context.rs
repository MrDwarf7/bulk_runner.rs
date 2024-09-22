use std::collections::HashMap;
use std::path::PathBuf;

use super::{db_info::DbInfo, query_engine::QueryEngine};
use super::{AsyncFrom, DbContext};
use crate::{Result, Status};

use crate::prelude::*;

pub struct Context<T> {
    pub db: T,
    pub query_engine: QueryEngine,
    pub total_bots: usize,
    pub bot_list: HashMap<String, Status>, // Name | Status(Idle, Pending etc.)
}

impl Context<DbInfo> {
    pub async fn new(file: PathBuf) -> Result<Self> {
        let db_info = DbInfo::new()?;
        let mut query_engine = QueryEngine::from(db_info.clone());
        let bot_list = query_engine.get_bot_list(file).await?;

        let total_bots = bot_list.len();

        Ok(Context {
            db: db_info,
            query_engine,
            total_bots,
            bot_list,
        })
    }
}

impl<T: DbContext> DbContext for Context<T> {
    fn get_query_engine(&self) -> Box<QueryEngine> {
        self.db.get_query_engine()
    }
}

impl<T> From<T> for Context<T>
where
    T: Into<DbInfo> + Clone,
{
    fn from(value: T) -> Self {
        let db_info = value.clone().into();
        let query_engine = QueryEngine::from(db_info.clone());
        Context {
            db: value,
            query_engine,
            total_bots: 0,
            bot_list: HashMap::new(),
        }
    }
}

impl AsyncFrom<Option<PathBuf>> for Context<DbInfo> {
    type Output = Self;

    async fn async_from(value: Option<PathBuf>) -> Self::Output {
        match value {
            Some(file) => Context::new(file).await.unwrap(),
            None => {
                let file = std::env::current_exe().unwrap();
                let file = file.parent().unwrap().join(DEFAULT_QUERY_FILE);
                Context::new(file).await.unwrap()
            }
        }
    }
}

impl AsyncFrom<PathBuf> for Context<DbInfo> {
    type Output = Self;

    async fn async_from(value: PathBuf) -> Self::Output {
        Context::new(value).await.unwrap()
    }
}
