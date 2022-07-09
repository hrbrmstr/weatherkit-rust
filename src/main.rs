use std::time::{UNIX_EPOCH, SystemTime};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::env;
use serde_json::json;
use jsonwebtokens as jwt;
use jwt::{Algorithm, AlgorithmID, encode};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

  let wxkit_keyid = env::var("WEATHERKIT_KEY_ID").expect("Please set WEATHERKIT_KEY_ID");
  let wxkit_service_id = env::var("WEATHERKIT_SERVICE_ID").expect("Please set WEATHERKIT_SERVICE_ID");
  let wxkit_teamid = env::var("WEATHERKIT_TEAM_ID").expect("Please set WEATHERKIT_TEAM_ID");
  let wxkit_key_path = env::var("WEATHERKIT_KEY_PATH").expect("Please set WEATHERKIT_KEY_PATH");

  let f = File::open(wxkit_key_path.as_str()).expect("Error opening key file at WEATHERKIT_KEY_PATH");
  let mut reader = BufReader::new(f);
  let mut buffer = Vec::new();    
  reader.read_to_end(&mut buffer).expect("Error reading key file at WEATHERKIT_KEY_PATH");
  let key_buf: &[u8] = &buffer;

  let alg = Algorithm::new_ecdsa_pem_signer(AlgorithmID::ES256, key_buf).expect("Error decoding PEM");

  let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Timey wimey").as_secs();
  let expires = now + 3600;

  let claims = json!({
    "iss": wxkit_teamid.to_owned(),
    "iat": now,
    "exp": expires,
    "sub": wxkit_service_id.to_owned()
  });

  let header = json!({
    "alg": "ES256",
    "kid": wxkit_keyid.to_owned(),
    "id": format!("{}.{}", wxkit_teamid.to_owned(), wxkit_service_id.to_owned())
  });

  // println!("{}\n{}", claims, header);

  let token = encode(&header, &claims, &alg).expect("Error creating JWT");

  let url = format!("https://weatherkit.apple.com/api/v1/weather/{}/{}/{}?timezone=America/New_York&dataSets=currentWeather,forecastDaily,forecastHourly,forecastNextHour,weatherAlerts", "en", 43.2, -70.8);

  let client = reqwest::Client::new();

  let mut call = client.get(url);
  call = call
    .header("Authorization", format!("Bearer {}", token))
    .header("Accept", "application/json");

  let resp = call.send().await?;

  let body = resp.text().await?;

  println!("{}", body);

  Ok(())

}
