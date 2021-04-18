fn main() -> Result<(), Box<dyn std::error::Error>> {
    // セレクターをパース　(このセレクターは記事のアンカーノード群(タイトル)を指す。 <a href="link">Title</a>)
    let selector = scraper::Selector::parse("td.bn > a").unwrap();

    // `https://blog.rust-lang.org/` へHTTPリクエスト
    let body = reqwest::blocking::get("https://blog.rust-lang.org/")?.text()?;

    // HTMLをパース
    let document = scraper::Html::parse_document(&body);

    // セレクターを用いて要素を取得
    let elements = document.select(&selector);

    // 全記事名を出力
    elements.for_each(|e| println!("{}", e.text().next().unwrap()));

    // 一件目の記事名
    // assert_eq!(elements.next().unwrap().text().next().unwrap(), "Announcing Rust 1.50.0");
    // 二件目の記事名
    // assert_eq!(elements.next().unwrap().text().next().unwrap(), "mdBook security advisory");
    // 三件目の記事名
    // assert_eq!(elements.next().unwrap().text().next().unwrap(), "Announcing Rust 1.49.0");
    // 最古の記事名
    // assert_eq!(elements.last().unwrap().text().next().unwrap(), "Road to Rust 1.0");

    Ok(())
}
