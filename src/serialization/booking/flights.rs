// Keep below here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flight_response() {
        // Load the test fixture
        let json = include_str!("fixture/response.json");

        // Parse the JSON
        let response: FlightSearchResponse = serde_json::from_str(json).unwrap();

        // Verify key fields from the response
        assert!(response.status);
        assert_eq!(response.message, "Success");

        // Check flight offers
        let offers = &response.data.flight_offers;
        assert!(!offers.is_empty());

        // Check first flight offer details
        let first_offer = &offers[0];
        let first_segment = &first_offer.segments[0];

        // Check airport details
        let departure = &first_segment.departure_airport;
        assert_eq!(departure.code, "SFO");
        assert_eq!(departure.city_name, "San Francisco");
        assert_eq!(departure.country, "US");
        assert_eq!(departure.province, "California");
        assert_eq!(departure.province_code.as_ref().unwrap(), "CA");

        let arrival = &first_segment.arrival_airport;
        assert_eq!(arrival.code, "OSL");
        assert_eq!(arrival.city_name, "Oslo");
        assert_eq!(arrival.country, "NO");
        assert_eq!(arrival.province, "Oslo");

        // Check leg details
        let first_leg = &first_segment.legs[0];
        assert_eq!(first_leg.cabin_class, "ECONOMY");
        assert_eq!(first_leg.flight_info.flight_number, 39);
        assert_eq!(first_leg.flight_info.carrier_info.operating_carrier, "LX");
        assert_eq!(first_leg.total_time, 40200);
    }
}
