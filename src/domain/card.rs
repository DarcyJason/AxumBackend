use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum CardBrand {
    Visa,
    Mastercard,
    AmericanExpress,
    Discover,
    DinerClub,
    JCB,
    Maestro,
    UnionPay,
    RuPay,
    Others,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Month {
    January,
    Februray,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: Thing,
    pub owner_name: Option<String>,
    pub card_number: Option<String>,
    pub card_brand: Option<CardBrand>,
    pub expired_month: Option<Vec<Month>>,
    pub expired_year: Option<i32>,
    pub safe_code: Option<String>,
}
