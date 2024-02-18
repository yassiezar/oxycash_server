use crate::portfolio::Portfolio;
use crate::portfolio::Stock;
use crate::portfolio::StockPrice;

mod portfolio;

fn main() {
    let price: StockPrice = StockPrice::new(50, 0);
    let test_stock: Stock = Stock::new(
        123,
        "ISIN".to_string(),
        "LSE".to_string(),
        "DISPLAY_NAME".to_string(),
        "TICKER".to_string(),
        price,
    );

    let mut portfolio: Portfolio = Portfolio::new(None);

    match portfolio.purchase(test_stock, 50) {
        Err(res) => println!("{}", res),
        Ok(_) => (),
    };

    portfolio.print_value();
}
