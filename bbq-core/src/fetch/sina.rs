use std::{collections::HashMap, fmt::format};
use crate::fetch::{Fetcher, Quot, RtQuot};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime};
use regex::Regex;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::{QError, QResult};

#[derive(Debug, Clone)]
pub struct Sina {
    regex: Regex,
    client: Client,
}

impl Sina {
    pub fn new() -> Self {
        let mut re_str = String::from(r"(\w+)=([^\s][^,]+?)");
        for _ in 0..29 {
            re_str.push_str(r",([\.\d]+)")
        }
        for _ in 0..2 {
            re_str.push_str(r",([-\.\d:]+)");
        }

        let mut h = HeaderMap::new();
        h.insert("user-agent",
                 "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.86 Safari/537.36".parse().unwrap());
        h.insert("Referer",
                 "http://finance.sina.com.cn/".parse().unwrap());

        let builder = reqwest::Client::builder()
            .default_headers(h);

        Self {
            regex: Regex::new(re_str.as_str()).unwrap(),
            client: builder.build().unwrap(),
        }
    }
}

#[async_trait]
impl Fetcher for Sina {
    async fn fetch_rt_quot(&self, code: &Vec<String>) -> QResult<RtQuot> {
        let url = format!("http://hq.sinajs.cn/?format=text&list={}", code.join(","));

        let data = self.client.get(url)
            .send().await.map_err(|e| QError::Fetch("Send sina http request error!".into()))?
            .text().await.map_err(|e| QError::Fetch("Parse sina http response error!".into()))?;

        if !self.regex.is_match(data.as_str()) {
            return Err(QError::Fetch("Sina response data error!".into()));
        }

        let mut rq = RtQuot::new();
        for cap in self.regex.captures_iter(data.as_str()) {
            let q = Quot {
                code: String::from(&cap[1]),
                name: String::from(&cap[2]),
                open: cap[3].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                pre_close: cap[4].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                now: cap[5].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                high: cap[6].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                low: cap[7].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                buy: cap[8].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                sell: cap[9].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                vol: cap[10].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                amount: cap[11].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
                bid: (
                    (cap[12].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[13].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[14].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[15].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[16].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[17].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[18].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[19].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[20].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[21].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                ),
                ask: (
                    (cap[22].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[23].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[24].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[25].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[26].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[27].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[28].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[29].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                    (cap[30].parse().map_err(|e| QError::Fetch(format!("Parse int: {}", e)))?,
                     cap[31].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?),
                ),
                date: cap[32].parse().map_err(|e| QError::Fetch(format!("Parse naive_date: {}", e)))?,
                time: cap[33].parse().map_err(|e| QError::Fetch(format!("Parse float: {}", e)))?,
            };
            rq.insert(q.code.clone(), q);
        }
        Ok(rq)
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_sina_rt() {
        use super::Sina;
        use crate::fetch::Fetcher;
        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let sina = Sina::new();
            let codes: Vec<String> = "sh600063,sh601456".split(",")
                .map(|x|String::from(x)).collect();
            let rs = sina.fetch_rt_quot(&codes).await.unwrap();
            println!("rt quot: {:?}", rs);
        });
    }
}

