enum AccountType {
    ASSET,
    LIABILITY,
    CASH,
}

struct Account<U: Into<f64>, T: Into<f64>>
where
    U: std::marker::Copy + std::ops::Sub<Output = U> + std::ops::Add<Output = U>,
    T: std::marker::Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    amount: U,
    growth_rate: T,
    account_type: AccountType,
}

impl<U: Into<f64> + Copy, T: Into<f64> + Copy> Account<U, T>
where
    U: std::marker::Copy + std::ops::Sub<Output = U> + std::ops::Add<Output = U>,
    T: std::marker::Copy + std::ops::Sub<Output = T> + std::ops::Add<Output = T>,
{
    fn calculate_annual_growth(&self) -> f64 {
        // Use compound interest
        let total = match self.account_type {
            AccountType::ASSET => {
                self.amount.into() * (1. + (self.growth_rate.into() / 100. / 12.)).powf(12.)
            }
            AccountType::CASH => {
                self.amount.into() * (1. + (-self.growth_rate.into() / 100. / 12.)).powf(12.)
            }
            AccountType::LIABILITY => {
                self.amount.into() * (1. + (-self.growth_rate.into() / 100. / 12.)).powf(12.)
            }
        };

        return total - self.amount.into();
    }

    fn withdraw(&mut self, amount: U) -> &mut Self {
        self.amount = match self.account_type {
            AccountType::CASH => self.amount - amount,
            AccountType::ASSET => self.amount - amount,
            AccountType::LIABILITY => self.amount + amount,
        };

        return self;
    }

    fn deposit(&mut self, amount: U) -> &mut Self {
        self.amount = match self.account_type {
            AccountType::CASH => self.amount + amount,
            AccountType::ASSET => self.amount + amount,
            AccountType::LIABILITY => self.amount - amount,
        };

        return self;
    }
}

fn main() {
    let cash_account = Account {
        amount: 10_000,
        growth_rate: 7.0,
        account_type: AccountType::CASH,
    };

    let first_direct_account = Account {
        amount: 0.00,
        growth_rate: 0.01,
        account_type: AccountType::ASSET,
    };

    let mortgage_account = Account {
        amount: 50_000,
        growth_rate: 5.0,
        account_type: AccountType::LIABILITY,
    };

    let cash_growth = cash_account.calculate_annual_growth();
    let mortgage_growth = mortgage_account.calculate_annual_growth();
    let savings_growth = first_direct_account.calculate_annual_growth();

    println!("Cash growth: £{:.2}", cash_growth);
    println!("Mortgage growth: £{:.2}", mortgage_growth);
    println!("Saving growth growth: £{:.2}", savings_growth);
}
