use scraper::{Html, Selector};
use colorful::{Color};
use colorful::Colorful;
use chrono::prelude::*;

pub async fn parse_trending(weibo: bool, zhihu: bool) -> Result<(), reqwest::Error>  {
    if weibo && zhihu {
        format!("{:^28}\n{}", "WARNING", "There should be one option enabled!!!").warn();
        return Ok(())
    }
    if zhihu {
        let resp = reqwest::get("https://www.zhihu.com/billboard")
            .await?
            .text()
            .await?;
        let doc = Html::parse_fragment(&resp);
        let sel = Selector::parse(".HotList-itemTitle").unwrap();
        let mut count = 1;

        println!("  {}", Local::now().format("%a %Y-%m-%d %H:%M:%S").to_string().gradient(Color::LightCyan3));

        for event in doc.select(&sel) {
            if count > 50 { break }
            let event = event.inner_html();
            println!("{}", format!("{}. {}", count, event).gradient(Color::Yellow));
            count += 1;
        }
    }
    if weibo || (!weibo && !zhihu) {
        let resp = reqwest::get("https://tophub.today/n/KqndgxeLl9")
            .await?
            .text()
            .await?;
        let doc = Html::parse_fragment(&resp);
        let sel = Selector::parse(".al").unwrap();
        let sub_sel = Selector::parse("a").unwrap();
        let mut count = 1;

        println!("  {}", Local::now().format("%a %Y-%m-%d %H:%M:%S").to_string().gradient(Color::LightCyan3));

        for el in doc.select(&sel) {
            if count > 50 { break }
            let event = el.select(&sub_sel).next().unwrap().inner_html().to_string();
            println!("{}", format!("{}. {}", count, event).gradient(Color::Yellow));
            count += 1;
        }
    }

    Ok(())
}

