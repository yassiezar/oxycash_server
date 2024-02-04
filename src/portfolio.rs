use std::collections::HashMap;

struct StockPrice {
    price: u32,
    date: u64,
}

pub struct Stock {
    id: u64,
    isin: String,
    market_name: String,
    display_name: String,
    ticker: String,
    current_price: StockPrice,
}

struct StockHolding {
    stock: Stock,
    owned: u32,
}

pub struct Portfolio {
    holdings: HashMap<u64, StockHolding>,
}

impl Stock {
    pub fn new(id: u64, isin: String, market_name: String, display_name: String, ticker: String) -> Stock {
        let price: StockPrice = StockPrice { price: 0, date: 0 };
        Stock { id, isin, market_name, display_name, ticker, current_price: price }
    }
}

impl Portfolio {
    pub fn new() -> Portfolio {
        Portfolio {
            holdings: HashMap::new(),
        }
    }

    pub fn purchase(&mut self, stock: Stock, amount: u32) {
        if let Some(x) = self.holdings.get_mut(&stock.id) {
            x.owned += amount;
        } else {
            self.holdings.insert(stock.id, StockHolding { stock, owned: amount });
        }
    }

    fn portfolio_value(&self) -> u64 {
        let mut value: u64 = 0;
        for (_, holding) in &self.holdings {
            value += (holding.owned * holding.stock.current_price.price) as u64;
        }

        return value;
    }

    pub fn print_value(&self) {
        println!("Value: ${}", self.portfolio_value());
    }
}
