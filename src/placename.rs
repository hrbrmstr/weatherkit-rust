use std::fs;
use serde::{Serialize, Deserialize};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

use app_dirs2::{AppDataType, AppInfo, get_app_dir};

const WEATHERKIT_APP_INFO: AppInfo = AppInfo {
    name: "weatherkit-rust",
    author: "@hrbrmstr",
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaceName {
  #[serde(rename = "place_name")]
  pub place_name: String,
  
  #[serde(rename = "dmslat")]
  pub dms_lat: String,
  
  #[serde(rename = "dmslng")]
  pub dms_lng: String,
}

fn lookup_placename(lat: f64,
                    lng: f64) -> PlaceName {
  
  let url = "https://latlon.top/index/reverse";
  
  let builder =  reqwest::blocking::ClientBuilder::new();
  let client = builder.connection_verbose(true).build().expect("Could not build client");
  
  let bdy = format!("{{ \"lat\": {}, \"lng\": {} }}", lat, lng);
  let resp = client
    .post(url)
    .body(bdy)
    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.1 Safari/605.1.15")
    .header("Content-Type", "application/json")
    .header("Accept", "application/json")
    .header("Origin", "https://latlon.top")
    .header("Referer", "https://latlon.top/index.php")
    .send();

  match resp {
    Ok(response) => {
      let placename = response.json::<PlaceName>();
      match placename {
        Ok(pn) => return pn,
        Err(_) => return PlaceName {
          place_name:  format!("({}, {})", lat, lng),
          dms_lat: lat.to_string(),
          dms_lng: lng.to_string()
        }
      };
    },
    Err(_) => return PlaceName {
      place_name: format!("({}, {})", lat, lng),
      dms_lat: lat.to_string(),
      dms_lng: lng.to_string()
    }
  };

}

#[derive(Serialize, Deserialize)]
struct Place {
  name: String
}

/// Retrieve a place name from lat/lon pair via latlon.top
/// 
/// # Arguments
/// 
/// - `lat`: latitude (float)
/// - `lon`: longitude (float)
/// 
/// `"(lat, lon)"` will be returned if there were any issues retrieving the reverse
/// geocoded information.
/// 
/// This value is cached to prevent wasting CPU/bandwidth of latlon.top
pub fn get_placename(lat: f64, lon: f64) -> String {

  let ll_key = format!("{};{}", lat, lon);

  let app_dir = get_app_dir(AppDataType::UserCache, &WEATHERKIT_APP_INFO, "").expect("Error finding application cache directory.");

  let db_path = app_dir.with_file_name("weatherkit-rust/placenames.db");
  
  fs::create_dir_all(app_dir).expect("Error locating or creating application cache directory."); 

  let mut db: PickleDb;

  if !db_path.exists() {

    db = PickleDb::new(
      db_path,
      PickleDbDumpPolicy::DumpUponRequest,
      SerializationMethod::Json,
    );

  } else {

    db = PickleDb::load(
      db_path,
      PickleDbDumpPolicy::AutoDump,
      SerializationMethod::Json,
    ).expect("Error loading application cache.");

  }

  #[allow(unused_mut)]
  let mut place: String;

  if db.exists(ll_key.as_str()) {

    place = db.get::<String>(ll_key.as_str()).unwrap();

  } else {

    let resp = lookup_placename(lat, lon);

    place = resp.place_name;

    db.set(ll_key.as_str(), &place).unwrap();

  }

  if !place.contains("(") {
    let split = place.split(", ");
    let place_parts: Vec<&str> = split.collect();
    let len = place_parts.len();
    if len >= 4 {
      place = [ place_parts[len-4], place_parts[len-3] ].join(", ");
    }
  };

  place.to_owned()

}
