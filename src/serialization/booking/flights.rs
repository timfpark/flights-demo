use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightResponse {
    pub data: FlightData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightData {
    pub flight_offers: Vec<FlightOffer>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightOffer {
    pub token: Option<String>,
    pub segments: Vec<Segment>,
    pub unified_price_breakdown: Option<UnifiedPriceBreakdown>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub departure_airport: Airport,
    pub arrival_airport: Airport,
    pub departure_time: String,
    pub arrival_time: String,
    pub legs: Vec<Leg>,
    pub total_time: i32,
    pub traveller_checked_luggage: Vec<LuggageInfo>,
    pub traveller_cabin_luggage: Vec<TravellerLuggage>,
    pub is_atol_protected: bool,
    pub show_warning_destination_airport: bool,
    pub show_warning_origin_airport: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Airport {
    #[serde(rename = "type")]
    pub airport_type: String,
    pub code: String,
    pub name: String,
    pub city: String,
    pub city_name: String,
    pub country: String,
    pub country_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leg {
    pub departure_time: String,
    pub arrival_time: String,
    pub departure_airport: Airport,
    pub arrival_airport: Airport,
    pub cabin_class: String,
    pub flight_info: FlightInfo,
    pub carriers: Vec<String>,
    pub carriers_data: Vec<CarrierData>,
    pub total_time: i32,
    pub flight_stops: Vec<String>,
    pub amenities: Vec<Amenity>,
    pub departure_terminal: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlightInfo {
    pub facilities: Vec<String>,
    pub flight_number: i32,
    pub plane_type: String,
    pub carrier_info: CarrierInfo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarrierInfo {
    pub operating_carrier: String,
    pub marketing_carrier: String,
    pub operating_carrier_disclosure_text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarrierData {
    pub name: String,
    pub code: String,
    pub logo: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amenity {
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub amenity_type: Option<serde_json::Value>, // Can be string or array of strings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legroom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch_unit: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TravellerLuggage {
    pub traveller_reference: String,
    pub luggage_allowance: LuggageAllowance,
    pub personal_item: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuggageAllowance {
    pub luggage_type: String,
    pub max_piece: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_weight_per_piece: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass_unit: Option<String>,
    pub size_restrictions: SizeRestrictions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SizeRestrictions {
    pub max_length: f64,
    pub max_width: f64,
    pub max_height: f64,
    pub size_unit: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedPriceBreakdown {
    pub items: Vec<PriceItem>,
    pub added_items: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceItem {
    pub scope: String,
    pub id: String,
    pub title: String,
    pub price: Price,
    pub items: Vec<PriceSubItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub currency_code: String,
    pub units: i64,
    pub nanos: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceSubItem {
    pub id: String,
    pub title: String,
    pub price: Price,
    pub items: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LuggageInfo {
    pub traveller_reference: String,
    pub luggage_allowance: LuggageAllowance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flight_response_deserialization() {
        let json = include_str!("../../../responses/sfo-osl.json");
        let response: FlightResponse = serde_json::from_str(json).expect("Failed to deserialize");

        let flight_offer = &response.data.flight_offers[0];
        let segment = &flight_offer.segments[0];

        // Test basic segment fields
        assert!(!segment.is_atol_protected);
        assert_eq!(segment.total_time, 69300);

        // Test airport details
        assert_eq!(segment.departure_airport.code, "SFO");
        assert_eq!(segment.departure_airport.country, "US");
        assert_eq!(segment.arrival_airport.code, "OSL");
        assert_eq!(segment.arrival_airport.country, "NO");

        // Test luggage allowance
        assert!(segment.traveller_checked_luggage.is_empty());
        assert_eq!(segment.traveller_cabin_luggage.len(), 3);

        let cabin_luggage = &segment.traveller_cabin_luggage[0];
        assert_eq!(cabin_luggage.traveller_reference, "1");
        assert_eq!(cabin_luggage.luggage_allowance.luggage_type, "HAND");
        assert_eq!(cabin_luggage.luggage_allowance.max_piece, 1);
        assert!(cabin_luggage.personal_item);
    }
}
