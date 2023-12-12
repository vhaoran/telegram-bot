use futures::{TryFutureExt, TryStreamExt};
// use headers::Authorization;
use hyper::client::{HttpConnector, ResponseFuture};
use hyper::{body::HttpBody, Body, Client, Method, Request, Response, Uri};
use hyper_proxy::{Intercept, Proxy, ProxyConnector};
use std::error::Error;
use tokio::io::{stdout, AsyncWriteExt as _};
//
use crate::errors::ErrorKind;
use hyper::body::to_bytes;
use std::collections::HashSet;

#[derive(Clone, Default, Debug)]
pub struct ProxyCfg {
    pub url: String,
    pub user_name: String,
    pub pwd: String,
}

#[tokio::test]
async fn test_xxx() -> anyhow::Result<()> {
    // self::p_test().await?;

    let uri: Uri =
        "https://api.telegram.org/bot1684733840:AAFxAOa2pgSzV1m9jggcsuEdS6guFmN_h-0/getMe"
            .parse()?;
    let mut req = Request::get(uri.clone()).body(hyper::Body::empty())?;
    let proxy = ProxyCfg {
        url: "http://127.0.0.1:33210".to_string(),
        user_name: "".to_string(),
        pwd: "".to_string(),
    };

    let r = self::proxy_exec(proxy, req).await?;

    // r.body()?;
    // println!("----response:-------{r:#?}-----------",);
    let whole_chunk = to_bytes(r.into_body()).await;
    let body = whole_chunk
        .iter()
        .fold(vec![], |mut acc, chunk| -> Vec<u8> {
            acc.extend_from_slice(&chunk);
            acc
        });

    println!("-------body:----{body:#?}-----------",);

    let body_str = String::from_utf8(body)?;
    println!("-----------body_str: {body_str}-----------",);
    Ok(())
}

pub async fn proxy_exec(p_cfg: ProxyCfg, req: Request<Body>) -> anyhow::Result<Response<Body>> {
    // let mut req = req.clone();
    let proxy = {
        let proxy_uri = p_cfg.url.as_str().parse()?;
        let mut proxy = Proxy::new(Intercept::All, proxy_uri);
        // if p_cfg.user_name.len() > 0 {
        //     proxy.set_authorization(Authorization::basic(
        //         p_cfg.user_name.as_str(),
        //         p_cfg.pwd.as_str(),
        //     ));
        // }
        //
        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy)?;
        proxy_connector
    };

    let uri: Uri = req.uri().clone();
    // let mut req = Request::get(uri.clone()).body(hyper::Body::empty())?;
    // if let Some(headers) = proxy.http_headers(&uri) {
    //     req.headers_mut().extend(headers.clone().into_iter());
    // }

    let client = Client::builder().build(proxy);
    let resp = client.request(req).await?;

    Ok(resp)
}

pub async fn p_test() -> anyhow::Result<()> {
    let proxy = {
        let proxy_uri = "http://127.0.0.1:33210".parse()?;
        let mut proxy = Proxy::new(Intercept::All, proxy_uri);
        // proxy.set_authorization(Authorization::basic("John Doe", "Agent1234"));
        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy)?;
        proxy_connector
    };

    let uri: Uri =
        "https://api.telegram.org/bot1684733840:AAFxAOa2pgSzV1m9jggcsuEdS6guFmN_h-0/getMe"
            .parse()?;
    let mut req = Request::get(uri.clone()).body(hyper::Body::empty())?;

    {
        let hd = req.headers().clone();
        println!("-----------hearer_before: {hd:#?}-----------",);
    }

    if let Some(headers) = proxy.http_headers(&uri) {
        req.headers_mut().extend(headers.clone().into_iter());
    }
    {
        let hd = req.headers().clone();
        println!("-----------hearer_after: {hd:#?}-----------",);
    }

    let client = Client::builder().build(proxy);
    let mut resp = client.request(req).await?;
    println!("Response: {}", resp.status());
    while let Some(chunk) = resp.body_mut().data().await {
        println!("-----------{chunk:#?}-----------",);
    }

    Ok(())
}
