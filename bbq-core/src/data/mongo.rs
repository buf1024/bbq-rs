use std::fmt::Debug;
use mongodb::bson::{DateTime, Document};
use mongodb::{Client, Database};
use mongodb::options::FindOptions;
use serde::{Serialize, Deserialize};
use crate::{QError, QResult};
use futures::TryStreamExt;
use serde::de::DeserializeOwned;

// 股票信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    pub code: String,
    pub name: String,
    pub listing_date: DateTime,
    pub block: String,
    pub is_margin: f64,
}

// 股票日线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockDaily {
    pub code: String,
    pub trade_date: DateTime,
    pub close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u32,
    pub turnover: f64,
    pub hfq_factor: f64,
}

// 股票指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockIndex {
    pub code: String,
    pub trade_date: DateTime,
    pub pe: f64,
    pub pe_ttm: f64,
    pub pb: f64,
    pub ps: f64,
    pub ps_ttm: f64,
    pub dv_ratio: f64,
    pub dv_ttm: f64,
    pub total_mv: f64,
}

// 股票复权因子
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockFqFactor {
    pub code: String,
    pub trade_date: DateTime,
    pub hfq_factor: f64,
    pub qfq_factor: f64,
}

// 指数信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    pub code: String,
    pub name: String,
}

// 指数日线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexDaily {
    pub code: String,
    pub trade_date: DateTime,
    pub close: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u32,
}

#[derive(Clone, Debug)]
pub struct MongoDB {
    client: Client,

    stock_coll: Vec<&'static str>,
    fund_coll: Vec<&'static str>
}

impl MongoDB {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            stock_coll: vec!["stock_info", "stock_daily", "stock_index", "stock_fq_factor"],
            fund_coll: vec!["index_info", "index_daily"],
        }
    }

    fn get_db(&self, coll: &str) -> QResult<Database> {
        if self.fund_coll.contains(&coll) {
            return Ok(self.client.database("bbq_fund_db"))
        }
        if self.stock_coll.contains(&coll) {
            return Ok(self.client.database("bbq_stock_db"))
        }
        Err(QError::MongoDBError(format!("collection {} not found", coll)))
    }

    pub async fn find<T>(&self, coll: &str,
                                filter: impl Into<Option<Document>>,
                                opts: impl Into<Option<FindOptions>>) -> QResult<Vec<T>>
        where T: DeserializeOwned + Unpin + Send + Sync {
        let db = self.get_db(coll)?;
        let coll = db.collection::<T>(coll);
        let mut cursor = coll.find(filter, opts).await?;
        let mut list = Vec::new();
        while let Some(item) = cursor.try_next().await? {
            list.push(item);
        }
        Ok(list)
    }
}


#[cfg(test)]
mod test {
    // use std::error::Error;
    // use std::time::Duration;
    // use mongodb::{bson::doc, bson::Document, Client};
    // use mongodb::options::{ClientOptions, FindOptions};
    // use crate::data::mongodb::{IndexInfo, MongoDB, StockInfo};
    // use futures::StreamExt;
    // use futures::TryStreamExt;


    #[test]
    fn test_db() {
        // async fn test() -> Result<(), Box<dyn Error>> {
        //     let mut clt_opts = ClientOptions::parse("mongodb://localhost:27017").await?;
        //     clt_opts.app_name = Some("bbq".into());
        //     clt_opts.connect_timeout = Some(Duration::from_secs(3));
        //
        //     let client = Client::with_options(clt_opts)?;
        //
        //
        //     let s = MongoDB::new(client);
        //     let find_opts = FindOptions::builder()
        //         .limit(15).build();
        //     let rs: Vec<IndexInfo> = s.find(s.stockdb(), "index_info", None, find_opts).await.unwrap();
        //     for v in rs {
        //         println!("item2: {:?}", v)
        //     }
        //     println!("what");
        //
        //     let find_opts = FindOptions::builder()
        //         .projection(doc! {"code": 1})
        //         .limit(15).build();
        //
        //     let rs: Vec<StockInfo> = s.find(s.stockdb(), "stock_info", doc! {"code":
        //         {"$in": vec!["sh601456".to_string(), "sh600063".to_string()]}}, None).await.unwrap();
        //     for v in rs {
        //         println!("item3: {:?}", v)
        //     }
        //     Ok(())
        // }
        //
        // let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        // rt.block_on(async move {
        //     test().await.unwrap()
        // });
    }
}
