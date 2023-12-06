use regex::Regex;
use scraper::{Html, Selector};

const URL: &str = "https://www.pricecharting.com/console/pal-nintendo-switch?sort=highest-price&exclude-hardware=true";

fn main() {
    let document = reqwest::blocking::get(URL).unwrap().text().unwrap();
    let html_document = Html::parse_document(&document);

    let table_selector = Selector::parse("#games_table").unwrap();
    let table = html_document.select(&table_selector).next().unwrap();

    let re = Regex::new(r#"[tT]itle"#).unwrap();
    let header_title_selector = Selector::parse("th span").unwrap();
    let header_title = table
        .select(&header_title_selector)
        .next()
        .unwrap()
        .text()
        .find(|&i| re.is_match(i))
        .map(|i| re.find(i).unwrap().as_str())
        .unwrap();

    let re = Regex::new(r#"[pP]rice"#).unwrap();
    let header_price_selector = Selector::parse("#js-loose-price").unwrap();
    let header_price = table
        .select(&header_price_selector)
        .next()
        .unwrap()
        .text()
        .find(|&i| re.is_match(i))
        .map(|i| re.find(i).unwrap().as_str())
        .unwrap();

    let table_row_selector = Selector::parse("#games_table tbody tr").unwrap();
    let table_rows = table.select(&table_row_selector);

    let videogame_title_selector = Selector::parse(".title a").unwrap();
    let videogame_price_selector = Selector::parse(".js-price").unwrap();

    println!("{header_title} - {header_price}");
    println!("-------------");

    for product in table_rows {
        let videogame_title: &str = product
            .select(&videogame_title_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .first()
            .unwrap();

        let videogame_price: &str = product
            .select(&videogame_price_selector)
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .first()
            .unwrap();

        println!("{videogame_title} - {videogame_price}");
    }
}
