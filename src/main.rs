use scraper::{Html, Selector};

fn main() {
    // "IT・コンピューター" ジャンルのURL
    let mut url = Some("https://book.dmm.com/list/otherbooks/?floor=Gotherbooks&n1=DgRJTglEBQ4GLGXbsI2FtdKGtYXTuvDa4cfU5Y6GtY%2AG4ow_".to_string());

    while let Some(v) = url {
        url = parse(&v);
    }
}

fn parse(url: &str) -> Option<String> {
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

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

        println!("{}\t{}", title, link);
    });

    find_next(&document)
}

fn find_next(document: &Html) -> Option<String> {
    // #l-contents > div.m-boxListController__itemPagenation > div > ul > li:nth-child(5) > span
    let nav_selector =
        Selector::parse("#l-contents > div.m-boxListController__itemPagenation > div > ul > li")
            .unwrap();
    let mut nav = document.select(&nav_selector);
    let cur_selector = Selector::parse("span.is-current").unwrap();
    while let Some(n) = nav.next() {
        // ナビゲーション上の、現在開いているページ
        let cur = n.select(&cur_selector).next();
        if let Some(_) = cur {
            // 現在ページの次が、次に読み込むべきページ
            let next_nav = nav.next();
            return next_nav.map(|e| {
                let selector = Selector::parse("a").unwrap();
                let a = e.select(&selector).next().unwrap();
                a.value().attr("href").unwrap().to_string()
            });
        };
    }
    None
}
