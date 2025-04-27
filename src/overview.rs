use crate::deserialize::from_str;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Represents company overview (fundamentals)
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Overview {
    #[serde(rename = "Symbol")]
    symbol: String,

    #[serde(rename = "AssetType")]
    asset_type: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "CIK")]
    cik: String, // CIK is often treated as a string identifier

    #[serde(rename = "Exchange")]
    exchange: String,

    #[serde(rename = "Currency")]
    currency: String,

    #[serde(rename = "Country")]
    country: String,

    #[serde(rename = "Sector")]
    sector: String,

    #[serde(rename = "Industry")]
    industry: String,

    #[serde(rename = "Address")]
    address: String,

    #[serde(rename = "OfficialSite")]
    official_site: String, // Consider using url::Url type if validation needed

    #[serde(rename = "FiscalYearEnd")]
    fiscal_year_end: String,

    #[serde(rename = "LatestQuarter")]
    latest_quarter: NaiveDate,

    #[serde(rename = "MarketCapitalization", deserialize_with = "from_str")]
    market_capitalization: f64, // Use u64 for large positive integers

    #[serde(rename = "EBITDA", deserialize_with = "from_str")]
    ebitda: i64, // EBITDA can potentially be negative, use i64

    #[serde(rename = "PERatio", deserialize_with = "from_str")]
    pe_ratio: f64, // Use f64 for floating-point numbers

    #[serde(rename = "PEGRatio", deserialize_with = "from_str")]
    peg_ratio: f64,

    #[serde(rename = "BookValue", deserialize_with = "from_str")]
    book_value: f64,

    #[serde(rename = "DividendPerShare", deserialize_with = "from_str")]
    dividend_per_share: f64,

    #[serde(rename = "DividendYield", deserialize_with = "from_str")]
    dividend_yield: f64,

    #[serde(rename = "EPS", deserialize_with = "from_str")]
    eps: f64,

    #[serde(rename = "RevenuePerShareTTM", deserialize_with = "from_str")]
    revenue_per_share_ttm: f64,

    #[serde(rename = "ProfitMargin", deserialize_with = "from_str")]
    profit_margin: f64,

    #[serde(rename = "OperatingMarginTTM", deserialize_with = "from_str")]
    operating_margin_ttm: f64,

    #[serde(rename = "ReturnOnAssetsTTM", deserialize_with = "from_str")]
    return_on_assets_ttm: f64,

    #[serde(rename = "ReturnOnEquityTTM", deserialize_with = "from_str")]
    return_on_equity_ttm: f64,

    #[serde(rename = "RevenueTTM", deserialize_with = "from_str")]
    revenue_ttm: u64,

    #[serde(rename = "GrossProfitTTM", deserialize_with = "from_str")]
    gross_profit_ttm: u64,

    #[serde(rename = "DilutedEPSTTM", deserialize_with = "from_str")]
    diluted_eps_ttm: f64,

    #[serde(rename = "QuarterlyEarningsGrowthYOY", deserialize_with = "from_str")]
    quarterly_earnings_growth_yoy: f64,

    #[serde(rename = "QuarterlyRevenueGrowthYOY", deserialize_with = "from_str")]
    quarterly_revenue_growth_yoy: f64,

    #[serde(rename = "AnalystTargetPrice", deserialize_with = "from_str")]
    analyst_target_price: f64,

    // Note: The API returns analyst counts as strings. Serde can often
    // deserialize string representations of numbers into numeric types directly.
    #[serde(rename = "AnalystRatingStrongBuy", deserialize_with = "from_str")]
    analyst_rating_strong_buy: u32,

    #[serde(rename = "AnalystRatingBuy", deserialize_with = "from_str")]
    analyst_rating_buy: u32,

    #[serde(rename = "AnalystRatingHold", deserialize_with = "from_str")]
    analyst_rating_hold: u32,

    #[serde(rename = "AnalystRatingSell", deserialize_with = "from_str")]
    analyst_rating_sell: u32,

    #[serde(rename = "AnalystRatingStrongSell", deserialize_with = "from_str")]
    analyst_rating_strong_sell: u32,

    #[serde(rename = "TrailingPE", deserialize_with = "from_str")]
    trailing_pe: f64,

    #[serde(rename = "ForwardPE", deserialize_with = "from_str")]
    forward_pe: f64,

    #[serde(rename = "PriceToSalesRatioTTM", deserialize_with = "from_str")]
    price_to_sales_ratio_ttm: f64,

    #[serde(rename = "PriceToBookRatio", deserialize_with = "from_str")]
    price_to_book_ratio: f64,

    #[serde(rename = "EVToRevenue", deserialize_with = "from_str")]
    ev_to_revenue: f64,

    #[serde(rename = "EVToEBITDA", deserialize_with = "from_str")]
    ev_to_ebitda: f64,

    #[serde(rename = "Beta", deserialize_with = "from_str")]
    beta: f64,

    // Rust field names cannot start with a number, so prefix with underscore.
    // `serde(rename)` still maps to the correct JSON key.
    #[serde(rename = "52WeekHigh", deserialize_with = "from_str")]
    _52_week_high: f64,

    #[serde(rename = "52WeekLow", deserialize_with = "from_str")]
    _52_week_low: f64,

    #[serde(rename = "50DayMovingAverage", deserialize_with = "from_str")]
    _50_day_moving_average: f64,

    #[serde(rename = "200DayMovingAverage", deserialize_with = "from_str")]
    _200_day_moving_average: f64,

    #[serde(rename = "SharesOutstanding", deserialize_with = "from_str")]
    shares_outstanding: u64,

    #[serde(rename = "DividendDate")]
    dividend_date: NaiveDate,

    #[serde(rename = "ExDividendDate")]
    ex_dividend_date: NaiveDate,
}

pub(crate) mod parser {
    use super::*;
    use crate::error::Error;
    use std::io::Read;

    pub(crate) fn parse(reader: impl Read) -> Result<Overview, Error> {
        Ok(serde_json::from_reader(reader)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize::parse_time;
    use std::io::BufReader;

    #[test]
    fn parse_ibm() {
        let data: &[u8] = include_bytes!("../tests/json/overview_ibm.json");

        let overview =
            parser::parse(BufReader::new(data)).expect("failed to parse ibm overview results");

        assert_eq!(
            overview,
            Overview {
                symbol: "IBM".to_string(),
        asset_type: "Common Stock".to_string(),
        name: "International Business Machines".to_string(),
        description: "International Business Machines Corporation (IBM) is an American multinational technology company headquartered in Armonk, New York, with operations in over 170 countries. The company began in 1911, founded in Endicott, New York, as the Computing-Tabulating-Recording Company (CTR) and was renamed International Business Machines in 1924. IBM is incorporated in New York. IBM produces and sells computer hardware, middleware and software, and provides hosting and consulting services in areas ranging from mainframe computers to nanotechnology. IBM is also a major research organization, holding the record for most annual U.S. patents generated by a business (as of 2020) for 28 consecutive years. Inventions by IBM include the automated teller machine (ATM), the floppy disk, the hard disk drive, the magnetic stripe card, the relational database, the SQL programming language, the UPC barcode, and dynamic random-access memory (DRAM). The IBM mainframe, exemplified by the System/360, was the dominant computing platform during the 1960s and 1970s.".to_string(),
        cik: "51143".to_string(),
        exchange: "NYSE".to_string(),
        currency: "USD".to_string(),
        country: "USA".to_string(),
        sector: "TECHNOLOGY".to_string(),
        industry: "COMPUTER & OFFICE EQUIPMENT".to_string(),
        address: "1 NEW ORCHARD ROAD, ARMONK, NY, US".to_string(),
        official_site: "https://www.ibm.com".to_string(),
        fiscal_year_end: "December".to_string(),
        latest_quarter: "2025-03-31".parse::<NaiveDate>().unwrap(),
        market_capitalization: 216001151000.0,
        ebitda: 13926000000,
        pe_ratio: 39.66,
        peg_ratio: 1.644,
        book_value: 28.92,
        dividend_per_share: 6.68,
        dividend_yield: 0.0287,
        eps: 5.86,
        revenue_per_share_ttm: 67.97,
        profit_margin: 0.0871,
        operating_margin_ttm: 0.124,
        return_on_assets_ttm: 0.0447,
        return_on_equity_ttm: 0.218,
        revenue_ttm: 62832001000,
        gross_profit_ttm: 35840000000,
        diluted_eps_ttm: 5.86,
        quarterly_earnings_growth_yoy: -0.349,
        quarterly_revenue_growth_yoy: 0.005,
        analyst_target_price: 253.01,
        analyst_rating_strong_buy: 3,
        analyst_rating_buy: 8,
        analyst_rating_hold: 7,
        analyst_rating_sell: 2,
        analyst_rating_strong_sell: 1,
        trailing_pe: 39.66,
        forward_pe: 21.46,
        price_to_sales_ratio_ttm: 3.438,
        price_to_book_ratio: 8.04,
        ev_to_revenue: 4.223,
        ev_to_ebitda: 21.52,
        beta: 0.662,
        _52_week_high: 266.45, // Remember the underscore prefix
        _52_week_low: 157.33,
        _50_day_moving_average: 247.13,
        _200_day_moving_average: 223.4,
        shares_outstanding: 929397000,
        dividend_date: "2025-03-10".parse::<NaiveDate>().unwrap(),
        ex_dividend_date: "2025-02-10".parse::<NaiveDate>().unwrap(),
            }
        );
        /*
        match serde_json::from_str::<StockData>(json_data) {
            Ok(stock_info) => {
                println!("Successfully parsed stock data for: {}", stock_info.symbol);
                println!("Name: {}", stock_info.name);
                println!("Market Cap: {}", stock_info.market_capitalization);
                println!("Trailing PE: {}", stock_info.trailing_pe);
                println!("52 Week High: {}", stock_info._52_week_high); // Access using the underscore prefix
                println!("Analyst Strong Buy Ratings: {}", stock_info.analyst_rating_strong_buy);
                // Print other fields as needed...
                 println!("\nFull Debug Output:\n{:?}", stock_info);
            }
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
            }
            */
    }
}
