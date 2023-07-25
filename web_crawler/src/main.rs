use reqwest::Client;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client
    let client = Client::new();

    // Send a GET request to the website
    let res = client
        .get("https://www.forbeschina.com/lists/1783")
        .send()
        .await?;

    // println!("{:?}", res);

    // Extract the HTML from the response
    // println!("--------------------------------");
    let body = res.text().await?;
    // println!("{:?}", body);
    // println!("--------------------------------");

    // Parse the HTML into a document
    let document = Html::parse_document(&body);
    // println!("{:?}", document);

    // Create a selector for the book titles
    let name_selector = Selector::parse("tr > td").unwrap();

    // // Iterate over the book titles
    // 在迭代之前創建一個空的 Vec
    let mut names = Vec::new();

    for name_item in document.select(&name_selector) {
        // let name = name_item.text().collect::<Vec<_>>();
        // 將收集到的文本追加到 names Vec 中

        names.extend(name_item.text());
    }

for name, i in names.enumerate() {
    
}
    println!("names: {:?}", names);

    // // Create a selector for the book prices
    // let book_price_selector = Selector::parse(".price_color").unwrap();

    // // Iterate over the book prices
    // for book_price in document.select(&book_price_selector) {
    //     let price = book_price.text().collect::<Vec<_>>();
    //     println!("Price: {}", price[0]);
    // }

    Ok(())
}
