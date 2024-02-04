use std::collections::HashMap;

enum AssetType {
    STOCK(Stock),
    CASH(Cash),
}

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

struct Cash {
    currency: String,
}

trait Asset {}

impl Asset for Cash {}
impl Asset for Stock {}
impl Asset for AssetType {}

struct AssetHolding<AssetT: Asset> {
    asset: AssetT,
    owned: u32,
}

pub struct Portfolio {
    holdings: HashMap<u64 /* asset ID */, AssetHolding<AssetType> /* Holding type */>,
}

impl Stock {
    pub fn new(
        id: u64,
        isin: String,
        market_name: String,
        display_name: String,
        ticker: String,
    ) -> Stock {
        let price: StockPrice = StockPrice { price: 0, date: 0 };
        Stock {
            id,
            isin,
            market_name,
            display_name,
            ticker,
            current_price: price,
        }
    }
}

impl Portfolio {
    pub fn new() -> Portfolio {
        let cash = AssetHolding {
            asset: AssetType::CASH(Cash {
                currency: "GBP".to_string(),
            }),
            owned: 0,
        };

        Portfolio {
            holdings: HashMap::from([(0, cash)]),
        }
    }

    pub fn purchase(&mut self, stock: Stock, amount: u32) {
        if let Some(x) = self.holdings.get_mut(&stock.id) {
            x.owned += amount;
        } else {
            self.holdings.insert(
                stock.id,
                AssetHolding {
                    asset: AssetType::STOCK(stock),
                    owned: amount,
                },
            );
        }
    }

    fn portfolio_value(&self) -> u64 {
        let mut value: u64 = 0;
        for (_, holding) in &self.holdings {
            let asset= &holding.asset;
            match asset{
                AssetType::CASH(_) => {
                    value += holding.owned as u64;
                },
                AssetType::STOCK(asset) => {
                    value += (asset.current_price.price * holding.owned) as u64;
                },
            }
        }

        return value;
    }

    pub fn print_value(&self) {
        println!("Value: ${}", self.portfolio_value());
    }
}
