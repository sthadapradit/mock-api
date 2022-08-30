use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub city_search: CitySearch,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitySearch {
    pub search_result: SearchResult,
    pub properties: Vec<Property>,
    pub search_enrichment: SearchEnrichment,
    pub aggregation: Aggregation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub sort_matrix: SortMatrix,
    pub search_info: SearchInfo,
    pub urgency_detail: UrgencyDetail,
    pub histogram: Histogram,
    pub nha_probability: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortMatrix {
    pub result: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub field_id: String,
    pub sorting: Option<Sorting>,
    pub display: Display,
    #[serde(default)]
    pub child_matrix: Vec<ChildMatrix>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sorting {
    pub sort_field: String,
    pub sort_order: String,
    pub sort_params: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildMatrix {
    pub field_id: String,
    pub sorting: Value,
    pub display: Display2,
    pub child_matrix: Vec<ChildMatrix2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display2 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildMatrix2 {
    pub field_id: String,
    pub sorting: Sorting2,
    pub display: Display3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sorting2 {
    pub sort_field: String,
    pub sort_order: String,
    pub sort_params: Option<SortParams>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortParams {
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Display3 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchInfo {
    pub flexible_search: FlexibleSearch,
    pub has_secret_deal: bool,
    pub is_complete: bool,
    pub total_filtered_hotels: i64,
    pub has_escapes_package: bool,
    pub search_status: SearchStatus,
    pub object_info: ObjectInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleSearch {
    pub current_date: CurrentDate,
    pub alternative_dates: Vec<AlternativeDate>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentDate {
    pub check_in: String,
    pub price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlternativeDate {
    pub check_in: String,
    pub price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchStatus {
    pub search_criteria: SearchCriteria,
    pub search_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCriteria {
    pub check_in: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectInfo {
    pub object_name: String,
    pub city_name: String,
    pub city_english_name: String,
    pub country_id: i64,
    pub country_english_name: String,
    pub map_latitude: f64,
    pub map_longitude: f64,
    pub map_zoom_level: i64,
    pub wl_preferred_city_name: Value,
    pub wl_preferred_country_name: Value,
    pub city_id: i64,
    pub city_center_polygon: CityCenterPolygon,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityCenterPolygon {
    pub geo_points: Vec<GeoPoint>,
    pub tourist_area_center_point: TouristAreaCenterPoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoPoint {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TouristAreaCenterPoint {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrgencyDetail {
    pub urgency_score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Histogram {
    pub bins: Vec<Bin>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bin {
    pub num_of_elements: i64,
    pub upper_bound: UpperBound,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpperBound {
    pub per_night_per_room: i64,
    pub per_pax: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub property_id: i64,
    pub sponsored_detail: SponsoredDetail,
    pub property_result_type: String,
    pub content: Content,
    pub sold_out: Option<SoldOut>,
    pub pricing: Pricing,
    pub meta_lab: Option<MetaLab>,
    pub enrichment: Enrichment,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SponsoredDetail {
    pub sponsored_type: String,
    pub tracking_data: Value,
    pub is_show_sponsored_flag: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub information_summary: InformationSummary,
    pub property_engagement: PropertyEngagement,
    pub non_hotel_accommodation: NonHotelAccommodation,
    pub facilities: Vec<Value>,
    pub images: Images,
    pub reviews: Reviews,
    pub family_features: FamilyFeatures,
    pub personalized_information: Value,
    pub local_information: LocalInformation,
    pub highlight: Highlight,
    pub rate_categories: RateCategories,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InformationSummary {
    pub hotel_character: Option<HotelCharacter>,
    pub property_links: PropertyLinks,
    pub atmospheres: Vec<Atmosphere>,
    pub locale_name: String,
    pub default_name: String,
    pub display_name: String,
    pub accommodation_type: i64,
    pub award_year: Option<String>,
    pub has_host_experience: bool,
    pub address: Address,
    pub property_type: String,
    pub rating: f64,
    pub agoda_guarantee_program: bool,
    pub remarks: Option<Remarks>,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub geo_info: GeoInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelCharacter {
    pub hotel_tag: Option<HotelTag>,
    pub hotel_view: Option<HotelView>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelTag {
    pub name: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelView {
    pub name: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyLinks {
    pub property_page: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Atmosphere {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country_code: String,
    pub country: Country,
    pub city: City,
    pub area: Area,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remarks {
    pub renovation_info: RenovationInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenovationInfo {
    pub renovation_type: i64,
    pub year: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpokenLanguage {
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoInfo {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyEngagement {
    pub last_booking: String,
    pub people_looking: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonHotelAccommodation {
    pub master_rooms: Vec<MasterRoom>,
    pub host_level: Value,
    pub supported_long_stay: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MasterRoom {
    pub no_of_bathrooms: Option<i64>,
    pub no_of_bedrooms: Option<i64>,
    pub no_of_beds: Option<i64>,
    pub room_size_sqm: f64,
    pub highlighted_facilities: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    pub hotel_images: Vec<HotelImage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HotelImage {
    pub id: i64,
    pub caption: String,
    pub provider_id: i64,
    pub urls: Vec<Url>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub key: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reviews {
    pub content_review: Vec<ContentReview>,
    pub cumulative: Cumulative2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentReview {
    pub is_default: bool,
    pub provider_id: i64,
    pub demographics: Option<Demographics>,
    pub summaries: Option<Summaries>,
    pub cumulative: Cumulative,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Demographics {
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: i64,
    pub grades: Vec<Grade>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Grade {
    pub id: String,
    pub score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summaries {
    pub recommendation_scores: Vec<RecommendationScore>,
    pub snippets: Vec<Snippet>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationScore {
    pub recommendation_score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub country_id: i64,
    pub country_code: String,
    pub country_name: String,
    pub date: String,
    pub demographic_id: i64,
    pub demographic_name: String,
    pub reviewer: String,
    pub review_rating: f64,
    pub snippet: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cumulative {
    pub review_count: i64,
    pub score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cumulative2 {
    pub review_count: i64,
    pub score: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyFeatures {
    pub has_children_free_policy: bool,
    pub is_family_room: bool,
    pub has_more_than_one_bedroom: bool,
    pub is_inter_connecting_room: bool,
    pub is_infant_cottage_available: bool,
    pub has_kids_pool: bool,
    pub has_kids_club: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalInformation {
    pub landmarks: Landmarks,
    pub has_airport_transfer: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Landmarks {
    pub transportation: Vec<Transportation>,
    #[serde(default)]
    pub top_landmark: Vec<TopLandmark>,
    pub beach: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transportation {
    pub landmark_name: String,
    pub distance_in_m: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopLandmark {
    pub landmark_name: String,
    pub distance_in_m: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Highlight {
    pub city_center: CityCenter,
    pub favorite_features: FavoriteFeatures,
    pub has_nearby_public_transportation: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityCenter {
    pub distance_from_city_center: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteFeatures {
    pub features: Vec<Feature>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    pub id: i64,
    pub title: String,
    pub category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateCategories {
    #[serde(default)]
    pub escape_rate_categories: Vec<EscapeRateCategory>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EscapeRateCategory {
    pub rate_category_id: i64,
    pub localized_rate_category_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoldOut {
    pub sold_out_price: SoldOutPrice,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoldOutPrice {
    pub average_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing {
    pub pulse_campaign_metadata: Option<PulseCampaignMetadata>,
    pub is_available: bool,
    pub is_ready: bool,
    pub benefits: Vec<i64>,
    pub cheapest_room_offer: Option<CheapestRoomOffer>,
    pub is_easy_cancel: Option<bool>,
    pub is_insider_deal: Option<bool>,
    pub is_multi_hotel_eligible: bool,
    pub suggest_price_type: SuggestPriceType,
    pub room_bundle: Value,
    pub pointmax: Value,
    pub price_change: Value,
    pub payment: Option<Payment>,
    #[serde(default)]
    pub cheapest_stay_package_rate_plans: Vec<CheapestStayPackageRatePlan>,
    pub pricing_messages: Vec<PricingMessage>,
    pub suppliers_summaries: Vec<SuppliersSummary>,
    pub supplier_info: Value,
    pub offers: Vec<Offer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PulseCampaignMetadata {
    pub promotion_type_id: i64,
    pub web_campaign_id: i64,
    pub campaign_type_id: i64,
    pub campaign_badge_text: String,
    pub campaign_badge_desc_text: String,
    pub deal_expiry_time: Option<String>,
    pub show_pulse_merchandise: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheapestRoomOffer {
    pub agoda_cash: AgodaCash,
    pub cashback: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgodaCash {
    pub show_badge: bool,
    pub giftcard_guid: String,
    pub day_to_earn: i64,
    pub earn_id: i64,
    pub percentage: f64,
    pub expiry_day: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuggestPriceType {
    pub suggest_price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub cancellation: Cancellation,
    pub pay_later: PayLater,
    pub pay_at_hotel: PayAtHotel,
    pub no_credit_card: NoCreditCard,
    pub tax_receipt: TaxReceipt,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cancellation {
    pub cancellation_type: String,
    pub free_cancellation_date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayLater {
    pub is_eligible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayAtHotel {
    pub is_eligible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoCreditCard {
    pub is_eligible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaxReceipt {
    pub is_eligible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheapestStayPackageRatePlan {
    pub stay_package_type: i64,
    pub rate_plan_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingMessage {
    pub location: i64,
    pub ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppliersSummary {
    pub id: i64,
    pub supplier_hotel_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub room_offers: Vec<RoomOffer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomOffer {
    pub room: Room,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub extra_price_info: ExtraPriceInfo,
    pub available_rooms: i64,
    pub is_promo_eligible: bool,
    pub promotions: Option<Promotions>,
    pub booking_duration: Value,
    pub supplier_id: i64,
    pub cor_summary: CorSummary,
    pub local_voucher: Value,
    pub packaging: Option<Packaging>,
    pub pricing: Vec<Pricing2>,
    pub uid: String,
    pub payment: Payment2,
    pub discount: Discount,
    pub save_up_to: SaveUpTo,
    pub benefits: Vec<Benefit>,
    pub channel: Channel,
    pub mse_room_summaries: Vec<MseRoomSummary>,
    pub cashback: Value,
    pub agoda_cash: AgodaCash2,
    pub cor_info: CorInfo,
    pub loyalty_display: Value,
    pub capacity: Capacity,
    pub pricing_messages: Option<PricingMessages>,
    pub campaign: Value,
    pub stay_package_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraPriceInfo {
    #[serde(rename = "displayPriceWithSurchargesPRPN")]
    pub display_price_with_surcharges_prpn: Value,
    #[serde(rename = "corDisplayPriceWithSurchargesPRPN")]
    pub cor_display_price_with_surcharges_prpn: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Promotions {
    pub type_id: i64,
    pub promotion_discount: PromotionDiscount,
    pub is_rate_plan_as_promotion: bool,
    pub cms_type_id: i64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionDiscount {
    pub value: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorSummary {
    pub has_cor: bool,
    pub cor_type: String,
    pub is_original: bool,
    #[serde(rename = "hasOwnCOR")]
    pub has_own_cor: bool,
    pub is_blacklisted_cor: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Packaging {
    pub token: Token,
    pub products: Vec<Value>,
    pub pricing: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub client_token: String,
    pub inter_system_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing2 {
    pub currency: String,
    pub price: Price,
    pub aps_peek: Value,
    pub promotion_price_peek: Value,
    pub channel_discount_summary: ChannelDiscountSummary,
    pub promotions_cumulative: Vec<PromotionsCumulative>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub per_night: PerNight,
    pub per_book: PerBook,
    pub per_room_per_night: PerRoomPerNight,
    pub total_discount: i64,
    pub price_after_applied_agoda_cash: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerNight {
    pub exclusive: Exclusive,
    pub inclusive: Inclusive,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusive {
    pub display: f64,
    pub cashback_price: Value,
    pub display_after_cashback: Value,
    pub original_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inclusive {
    pub display: f64,
    pub cashback_price: Value,
    pub display_after_cashback: Value,
    pub original_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerBook {
    pub exclusive: Exclusive2,
    pub inclusive: Inclusive2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusive2 {
    pub display: f64,
    pub cashback_price: Value,
    pub display_after_cashback: Value,
    pub rebate_price: f64,
    pub original_price: f64,
    pub auto_applied_promo_discount: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inclusive2 {
    pub display: f64,
    pub cashback_price: Value,
    pub display_after_cashback: Value,
    pub rebate_price: f64,
    pub original_price: f64,
    pub auto_applied_promo_discount: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerRoomPerNight {
    pub exclusive: Exclusive3,
    pub inclusive: Inclusive3,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusive3 {
    pub display: f64,
    pub crossed_out_price: f64,
    pub cashback_price: Value,
    pub display_after_cashback: Value,
    pub rebate_price: f64,
    pub pseudo_coupon_price: f64,
    pub original_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inclusive3 {
    pub display: f64,
    pub crossed_out_price: f64,
    pub cashback_price: Value,
    pub display_after_cashback: Value,
    pub rebate_price: f64,
    pub pseudo_coupon_price: f64,
    pub original_price: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDiscountSummary {
    pub channel_discount_breakdown: Vec<ChannelDiscountBreakdown>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDiscountBreakdown {
    pub display: bool,
    pub discount_percent: f64,
    pub channel_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromotionsCumulative {
    pub promotion_cumulative_type: i64,
    pub amount_percentage: f64,
    pub min_nights_stay: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment2 {
    pub payment_model: String,
    pub cancellation: Cancellation2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cancellation2 {
    pub cancellation_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discount {
    pub deals: Vec<String>,
    pub channel_discount: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveUpTo {
    pub per_room_per_night: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Benefit {
    pub id: i64,
    pub target_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MseRoomSummary {
    pub supplier_id: i64,
    pub sub_supplier_id: i64,
    pub pricing_summaries: Vec<PricingSummary>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingSummary {
    pub currency: String,
    pub channel_discount_summary: ChannelDiscountSummary2,
    pub price: Price2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDiscountSummary2 {
    pub channel_discount_breakdown: Vec<ChannelDiscountBreakdown2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelDiscountBreakdown2 {
    pub channel_id: i64,
    pub discount_percent: f64,
    pub display: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price2 {
    pub per_room_per_night: PerRoomPerNight2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerRoomPerNight2 {
    pub exclusive: Exclusive4,
    pub inclusive: Inclusive4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusive4 {
    pub display: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inclusive4 {
    pub display: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgodaCash2 {
    pub show_badge: bool,
    pub giftcard_guid: String,
    pub day_to_earn: i64,
    pub expiry_day: i64,
    pub percentage: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorInfo {
    pub cor_info: CorInfo2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorInfo2 {
    pub cor_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capacity {
    pub extra_beds_available: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingMessages {
    pub formatted: Vec<Formatted>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Formatted {
    pub location: i64,
    pub texts: Vec<Text>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub index: i64,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaLab {
    pub attributes: Vec<Attribute>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub attribute_id: i64,
    pub data_type: String,
    pub value: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enrichment {
    pub top_selling_point: Vec<TopSellingPoint>,
    pub pricing_badges: Option<PricingBadges>,
    pub unique_selling_point: Vec<UniqueSellingPoint>,
    pub booking_history: Value,
    pub show_review_snippet: bool,
    pub is_popular: bool,
    pub room_information: RoomInformation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopSellingPoint {
    pub tsp_type: String,
    pub value: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingBadges {
    pub badges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UniqueSellingPoint {
    pub rank: i64,
    pub segment: Option<String>,
    pub usp_type: String,
    pub usp_property_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInformation {
    pub cheapest_room_size_sqm: Option<f64>,
    pub facilities: Vec<Facility>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Facility {
    pub id: i64,
    pub property_facility_name: String,
    pub symbol: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchEnrichment {
    pub suppliers_information: Vec<SuppliersInformation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuppliersInformation {
    pub supplier_id: i64,
    pub supplier_name: String,
    pub is_agoda_band: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aggregation {
    pub matrix_group_results: Vec<MatrixGroupResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatrixGroupResult {
    pub matrix_group: String,
    pub matrix_item_results: Vec<MatrixItemResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatrixItemResult {
    pub id: i64,
    pub name: String,
    pub count: i64,
    pub filter_key: String,
    pub filter_request_type: String,
    #[serde(default)]
    pub extra_data_results: Vec<ExtraDataResult>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraDataResult {
    pub text: String,
    pub matrix_extra_data_type: String,
}
