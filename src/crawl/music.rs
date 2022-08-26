pub async fn parse_music() -> Result<(), reqwest::Error>  {
    let resp = reqwest::get("https://y.qq.com/n/ryqq/toplist/3")
        .await?
        .text()
        .await?;
    OK(())
}