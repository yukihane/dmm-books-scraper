use scraper::Selector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let selector = scraper::Selector::parse("#fn-list > li > div").unwrap();
    //::parse("#fn-list > li > div > a").unwrap();
    //#fn-list > li:nth-child(1) > div > div.m-boxListBookProductBtnBlock > div > span > input[type=hidden]

    // "IT・コンピューター" ジャンルのURL
    let body = reqwest::blocking::get("https://book.dmm.com/list/otherbooks/?floor=Gotherbooks&n1=DgRJTglEBQ4GLGXbsI2FtdKGtYXTuvDa4cfU5Y6GtY%2AG4ow_")?.text()?;

    // HTMLをパース
    let document = scraper::Html::parse_document(&body);

    // セレクターを用いて要素を取得
    let books = document.select(&selector);

    books.for_each(|b| {
        let a = Selector::parse("a").unwrap();
        let link = b.select(&a).next().unwrap().value().attr("href").unwrap();
        println!("{}", link);
    });

    // 全記事名を出力
    // elements.for_each(|e| println!("{}", e.text().next().unwrap()));

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
