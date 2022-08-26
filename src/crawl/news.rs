pub async fn parse_news() -> Result<(), reqwest::Error>  {
    let resp = reqwest::get("https://www.bbc.com/zhongwen/simp/topics/ck2l9z0em07t")
        .await?
        .text()
        .await?;
    OK(())
}

