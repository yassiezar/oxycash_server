use std::collections::HashMap;

enum CommonAssetId {
    CASH = 0,
}

pub struct StockPrice {
    price: u32,
    date: u64,
}

impl StockPrice {
    pub fn new(price: u32, date: u64) -> StockPrice {
        return StockPrice { price, date };
    }
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

enum AssetType {
    STOCK(Stock),
    CASH(Cash),
}

trait Asset {}

impl Asset for Cash {}
impl Asset for Stock {}
impl Asset for AssetType {}

trait HoldingProperties {
    fn get_value(&self) -> u32;
}

struct AssetHolding<AssetT: Asset> {
    asset: AssetT,
    owned: u32,
}

impl HoldingProperties for AssetHolding<Cash> {
    fn get_value(&self) -> u32 {
        return self.owned;
    }
}

impl HoldingProperties for AssetHolding<AssetType> {
    fn get_value(&self) -> u32 {
        let asset = &self.asset;
        return match asset {
            AssetType::CASH(_) => self.owned,
            AssetType::STOCK(asset) => self.owned * asset.current_price.price,
        };
    }
}

impl HoldingProperties for AssetHolding<Stock> {
    fn get_value(&self) -> u32 {
        return self.owned * self.asset.current_price.price;
    }
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
        price: StockPrice,
    ) -> Stock {
        return Stock {
            id,
            isin,
            market_name,
            display_name,
            ticker,
            current_price: price,
        };
    }
}

impl Portfolio {
    pub fn new(starting_cash: Option<u32>) -> Portfolio {
        let cash = AssetHolding {
            asset: AssetType::CASH(Cash {
                currency: "GBP".to_string(),
            }),
            owned: starting_cash.unwrap_or(0),
        };

        return Portfolio {
            holdings: HashMap::from([(CommonAssetId::CASH as u64 /* Cash ID */, cash)]),
        };
    }

    pub fn purchase(&mut self, stock: Stock, amount: u32) -> Result<(), String> {
        if stock.current_price.price * amount
            > self
                .holdings
                .get(&(CommonAssetId::CASH as u64))
                .unwrap()
                .get_value()
        {
            // Not enough cash available
            return Err(String::from("Not enough cash"));
        }

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

        Ok(())
    }

    fn portfolio_value(&self) -> u64 {
        let mut value: u64 = 0;
        for (_, holding) in &self.holdings {
            let asset = &holding.asset;
            match asset {
                AssetType::CASH(_) => {
                    value += holding.owned as u64;
                }
                AssetType::STOCK(asset) => {
                    value += (asset.current_price.price * holding.owned) as u64;
                }
            }
        }

        return value;
    }

    pub fn print_value(&self) {
        println!("Value: ${}", self.portfolio_value());
    }
}
