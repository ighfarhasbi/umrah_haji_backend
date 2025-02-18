use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryParams {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize)]
pub struct PrayerTimes {
    pub imsak: String,
    pub subuh: String,
    pub terbit: String,
    pub dhuha: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProvinceDetail {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub coordinate: Coordinate,
    pub id: String,
    pub name: String,
    pub slug: String,
    pub provinceId: String,
    pub province: ProvinceDetail,
    pub prayers: Vec<PrayerEntry>
}

#[derive(Serialize, Deserialize)]
pub struct PrayerEntry {
    pub time: PrayerTimes,
    pub id: String,
    pub cityId: String,
    pub date: String,
}