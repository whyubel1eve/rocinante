use scraper::{Html, Selector};
use colorful::Color;
use colorful::Colorful;

pub async fn parse_music() -> Result<(), reqwest::Error>  {
    let resp = reqwest::get("https://y.qq.com/n/ryqq/toplist/3")
        .await?
        .text()
        .await?;
    let doc = Html::parse_fragment(&resp);

    let song_sel = Selector::parse(".songlist__item").unwrap();
    let music_sel = Selector::parse(".songlist__cover").unwrap();
    let author_sel = Selector::parse(".playlist__author").unwrap();

    let mut rank = 1;

    for song in doc.select(&song_sel) {
        let music = song.select(&music_sel).next().unwrap().value().attr("title").unwrap();
        let music_src = song.select(&music_sel).next().unwrap().value().attr("href").unwrap();

        let author = song.select(&author_sel).next().unwrap().inner_html();
        let author = html_escape::decode_html_entities(&author);

        println!("{}. music: {}, source: {}, author: {}",
            rank.to_string().gradient(Color::LightCyan), music.gradient(Color::Green), format!("y.qq.com{}", music_src).color(Color::Cyan), author.gradient(Color::LightRed)
        );
        rank += 1;
    }
    Ok(())
}