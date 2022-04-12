use crate::fetch::{Fetcher, Quot, RtQuot, StockBarList};
use anyhow::{bail, Context, Ok, Result};
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime};
use regex::Regex;
use reqwest::{header::HeaderMap, Client};

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
        h.insert("Referer", "http://finance.sina.com.cn/".parse().unwrap());

        let builder = reqwest::Client::builder().default_headers(h);

        Self {
            regex: Regex::new(re_str.as_str()).unwrap(),
            client: builder.build().unwrap(),
        }
    }

   
}

#[async_trait]
impl Fetcher for Sina {
    async fn fetch_stock_minute(&self, code: &str, min: u32) -> Result<StockBarList> {
        let url = format!("http://quotes.sina.cn/cn/api/jsonp_v2.php/=/CN_MarketDataService.getKLineData?symbol={}&scale={}&datalen=20000", code, min);

        let data = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| "Send sina http request error!")?
            .text()
            .await
            .with_context(|| "Parse sina http response error!")?;

        let js: Vec<&str> = data.split("=(").collect();
        if js.len() < 2 {
            bail!("sina data error, len={}", js.len());
        }
        let js = js[1].split(");").next();
        if js.is_none() {
            bail!("sina data error");
        }

        Ok(serde_json::from_str(js.unwrap())?)
    }

    async fn fetch_rt_quot(&self, code: &Vec<String>) -> Result<RtQuot> {
        let url = format!("http://hq.sinajs.cn/?format=text&list={}", code.join(","));

        let data = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| "Send sina http request error!")?
            .text()
            .await
            .with_context(|| "Parse sina http response error!")?;

        if !self.regex.is_match(data.as_str()) {
            bail!("Sina response data error!");
        }

        let mut rq = RtQuot::new();
        for cap in self.regex.captures_iter(data.as_str()) {
            let date: NaiveDate = cap[32]
                .parse()
                .with_context(|| format!("Parse naive_date error: {}!", &cap[32]))?;
            let time: NaiveTime = cap[33]
                .parse()
                .with_context(|| format!("Parse naive_time error: {}!", &cap[33]))?;

            let date = date.format("%Y-%m-%d").to_string();
            let time = format!("{} {}", date, time.format("%H:%M:%S").to_string());

            let q = Quot {
                code: String::from(&cap[1]),
                name: String::from(&cap[2]),
                open: cap[3]
                    .parse()
                    .with_context(|| format!("Parse open error: {}!", &cap[3]))?,
                pre_close: cap[4]
                    .parse()
                    .with_context(|| format!("Parse pre_close error: {}!", &cap[4]))?,
                now: cap[5]
                    .parse()
                    .with_context(|| format!("Parse now error: {}!", &cap[5]))?,
                high: cap[6]
                    .parse()
                    .with_context(|| format!("Parse high error: {}!", &cap[6]))?,
                low: cap[7]
                    .parse()
                    .with_context(|| format!("Parse low error: {}!", &cap[7]))?,
                buy: cap[8]
                    .parse()
                    .with_context(|| format!("Parse buy error: {}!", &cap[8]))?,
                sell: cap[9]
                    .parse()
                    .with_context(|| format!("Parse sell error: {}!", &cap[9]))?,
                vol: cap[10]
                    .parse()
                    .with_context(|| format!("Parse vol error: {}!", &cap[10]))?,
                amount: cap[11]
                    .parse()
                    .with_context(|| format!("Parse amount error: {}!", &cap[11]))?,
                bid: (
                    (
                        cap[12]
                            .parse()
                            .with_context(|| format!("Parse bid[1] error: {}!", &cap[12]))?,
                        cap[13]
                            .parse()
                            .with_context(|| format!("Parse bid[1] error: {}!", &cap[13]))?,
                    ),
                    (
                        cap[14]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[14]))?,
                        cap[15]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[14]))?,
                    ),
                    (
                        cap[16]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[16]))?,
                        cap[17]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[17]))?,
                    ),
                    (
                        cap[18]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[18]))?,
                        cap[19]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[19]))?,
                    ),
                    (
                        cap[20]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[20]))?,
                        cap[21]
                            .parse()
                            .with_context(|| format!("Parse bid error: {}!", &cap[21]))?,
                    ),
                ),
                ask: (
                    (
                        cap[22]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[22]))?,
                        cap[23]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[23]))?,
                    ),
                    (
                        cap[24]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[24]))?,
                        cap[25]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[25]))?,
                    ),
                    (
                        cap[26]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[26]))?,
                        cap[27]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[27]))?,
                    ),
                    (
                        cap[28]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[28]))?,
                        cap[29]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[29]))?,
                    ),
                    (
                        cap[30]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[30]))?,
                        cap[31]
                            .parse()
                            .with_context(|| format!("Parse ask error: {}!", &cap[31]))?,
                    ),
                ),
                date,
                time,
            };
            rq.insert(q.code.clone(), q);
        }
        Ok(rq)
    }
}

#[cfg(test)]
mod test_sina {
    use super::Sina;
    use crate::fetch::Fetcher;
    #[test]
    fn test_sina_rt() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            let sina = Sina::new();

            let codes: Vec<String> = "sh600063,sh601456"
                .split(",")
                .map(|x| String::from(x))
                .collect();
            let rs = sina.fetch_rt_quot(&codes).await.unwrap();
            println!("rt quot: {:?}", rs);

            // let r = sina.fetch_stock_minute("sh000001", 1).await.unwrap();
            // println!("{:?}", r.get(0));
        });
    }
}
