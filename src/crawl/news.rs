use scraper::{Html, Selector};

pub async fn parse_news() -> Result<(), reqwest::Error>  {
    let resp = reqwest::get("")
        .await?
        .text()
        .await?;
    let doc = Html::parse_fragment(&resp);
    let selector = Selector::parse("").unwrap();
    for el in doc.select(&selector) {
        println!();
    }
    Ok(())
}

