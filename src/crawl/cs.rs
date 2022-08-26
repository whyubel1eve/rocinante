use scraper::{Html, Selector};
use colorful::Color;
use colorful::Colorful;

pub async fn parse_cs(id: &String) -> Result<(), reqwest::Error>  {
    let resp = reqwest::get(format!("https://www.csgola.com/player/{}", id))
        .await?
        .text()
        .await?;
    let doc = Html::parse_fragment(&resp);

    let sel = Selector::parse(".data-sec").unwrap();
    let k1_sel = Selector::parse(".title").unwrap();
    let v1_sel = Selector::parse(".datala").unwrap();

    let more_sel = Selector::parse(".col-md-4").unwrap();
    let li_sel = Selector::parse(".list-group-item").unwrap();
    let k2_sel = Selector::parse(".stats-title").unwrap();
    let v2_sel = Selector::parse(".stats-count").unwrap();

    for el in doc.select(&sel) {
        let k = el.select(&k1_sel).next().unwrap().inner_html();
        let v = el.select(&v1_sel).next().unwrap().inner_html();

        println!("{}: {}", k.gradient(Color::LightYellow), v.gradient(Color::Green));
    }
    println!("{}", "====================More======================".gradient(Color::Cyan));
    for el in doc.select(&more_sel) {
        for li in el.select(&li_sel) {
            let k = li.select(&k2_sel).next().unwrap().inner_html();
            let v = li.select(&v2_sel).next().unwrap().inner_html();
            println!("{}: {}", k.gradient(Color::LightYellow), v.gradient(Color::Green));
        }
    }

    Ok(())
}

