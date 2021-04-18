use scraper::html::Select;
use scraper::{Html, Selector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // "IT・コンピューター" ジャンルのURL
    let body = reqwest::blocking::get("https://book.dmm.com/list/otherbooks/?floor=Gotherbooks&n1=DgRJTglEBQ4GLGXbsI2FtdKGtYXTuvDa4cfU5Y6GtY%2AG4ow_")?.text()?;

    // HTMLをパース
    let document = Html::parse_document(&body);

    // セレクターを用いて要素を取得
    let book_selector = Selector::parse("#fn-list > li > div").unwrap();
    let books = document.select(&book_selector);

    books.for_each(|b| {
        let a = Selector::parse("a").unwrap();
        let link = b.select(&a).next().unwrap().value().attr("href").unwrap();

        let title_selector =
            Selector::parse("div.m-boxListBookProductBtnBlock > div > span > input[type=hidden]")
                .unwrap();
        let title = b
            .select(&title_selector)
            .next()
            .unwrap()
            .value()
            .attr("value")
            .unwrap();

        // println!("{}\t{}", title, link);
    });

    // #l-contents > div.m-boxListController__itemPagenation > div > ul > li:nth-child(5) > span
    let nav_selector =
        Selector::parse("#l-contents > div.m-boxListController__itemPagenation > div > ul > li")
            .unwrap();
    let nav = document.select(&nav_selector);

    find_next(nav);

    Ok(())
}

fn find_next(nav: Select) {
    nav.win
    nav.for_each(|n| {
        println!(".");
        let cur_selector = Selector::parse("span.is-current").unwrap();
        let cur = n.select(&cur_selector).next();
        if let Some(v) = cur {
            return 
        }
    });
}
