use tokio;
use hyper;
use hyper_tls;

use hyper::body::HttpBody;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let resume = get_from_uri("https://archlinux.org/mirrors/status/json/").await?;

    println!("{}", resume);

    Ok(())
}

async fn get_from_uri(url: &'static str) -> Result<String> {
    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let uri = url.parse()?;

    let mut resp = client.get(uri).await?;
    let mut result = String::new();

    while let Some(chunk) = resp.body_mut().data().await {
        let chunks = chunk?.into_iter().collect();
        result = String::from_utf8(chunks)?;
    }

    Ok(result)
}
