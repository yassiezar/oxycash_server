use crate::portfolio::Portfolio;
use crate::portfolio::Stock;

mod portfolio;

fn main() {
    let test_stock: Stock = Stock::new(
        123,
        "ISIN".to_string(),
        "LSE".to_string(),
        "DISPLAY_NAME".to_string(),
        "TICKER".to_string(),
    );

    let mut portfolio: Portfolio = Portfolio::new();

    portfolio.purchase(test_stock, 50);

    portfolio.print_value();
}
