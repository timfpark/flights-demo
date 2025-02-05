use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct FlightResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_sales_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_products_by_segment: Option<Vec<Vec<TravellerProduct>>>,
    pub included_products: IncludedProducts,
    pub extra_products: Vec<ExtraProduct>,
    pub offer_extras: OfferExtras,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TravellerProduct {
    pub traveller_reference: String,
    pub traveller_products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "type")]
    pub product_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<LuggageProduct>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LuggageProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub luggage_type: Option<String>,
    pub max_piece: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_weight_per_piece: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_restrictions: Option<SizeRestrictions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SizeRestrictions {
    pub max_length: f64,
    pub max_width: f64,
    pub max_height: f64,
    pub size_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncludedProducts {
    pub are_all_segments_identical: bool,
    pub segments: Vec<Vec<LuggageProduct>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraProduct {
    #[serde(rename = "type")]
    pub product_type: String,
    pub price_breakdown: PriceBreakdown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceBreakdown {
    pub total: Money,
    pub base_fare: Money,
    pub fee: Money,
    pub tax: Money,
    pub more_taxes_and_fees: HashMap<String, Money>,
    pub discount: Money,
    pub total_without_discount: Money,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rounded: Option<Money>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    pub units: i64,
    pub nanos: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferExtras {
    pub flexible_ticket: FlexibleTicket,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlexibleTicket {
    pub air_product_reference: String,
    pub travellers: Vec<String>,
    pub recommendation: Recommendation,
    pub price_breakdown: PriceBreakdown,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_info: Option<SupplierInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    pub recommended: bool,
    pub confidence: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierInfo {
    // Add fields as needed based on the supplier_info structure
    // This field appears to be optional and its contents aren't fully shown in the JSON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_money() {
        let json = r#"{
            "currencyCode": "USD",
            "units": 195,
            "nanos": 880000000
        }"#;

        let money: Money = serde_json::from_str(json).expect("Failed to deserialize Money");
        assert_eq!(money.currency_code, None);
        assert_eq!(money.units, 195);
        assert_eq!(money.nanos, 880000000);
    }

    #[test]
    fn test_deserialize_luggage_product() {
        let json = r#"{
            "luggageType": "HAND",
            "maxPiece": 1,
            "maxWeightPerPiece": 17.6,
            "massUnit": "LB",
            "sizeRestrictions": {
                "maxLength": 21.7,
                "maxWidth": 15.8,
                "maxHeight": 9.1,
                "sizeUnit": "INCH"
            }
        }"#;

        let product: LuggageProduct =
            serde_json::from_str(json).expect("Failed to deserialize LuggageProduct");
        assert_eq!(product.luggage_type, None);
        assert_eq!(product.max_piece, None);
        assert_eq!(product.max_weight_per_piece, None);
        assert_eq!(product.mass_unit, None);
    }
}
