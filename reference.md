# Reference
## Auth
<details><summary><code>client.auth.<a href="/src/api/resources/auth/client.rs">login</a>(request: LoginAuthRequest) -> Result&lt;(), ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Authenticates a user session to allow viewing private indoor sensors (`tempin`, `humin`, `heatin`) for the user's station.
This endpoint expects form urlencoded data and returns a `302 Found` redirect on successful login.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .auth
        .login(
            &LoginAuthRequest {
                login_form_entity: "LoginForm[entity]".to_string(),
                login_form_password: "LoginForm[password]".to_string(),
                login_form_remember_me: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**login_form_entity:** `String` — Username or email address
    
</dd>
</dl>

<dl>
<dd>

**login_form_password:** `String` — Account password
    
</dd>
</dl>

<dl>
<dd>

**login_form_remember_me:** `Option<LoginAuthRequestLoginFormRememberMe>` — Keep the user logged in
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## DeviceLive
<details><summary><code>client.device_live.<a href="/src/api/resources/device_live/client.rs">get_values</a>(device_id: String) -> Result&lt;DeviceValues, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns the latest sensor values for a device. **No CSRF token needed.**
This is the primary endpoint for a Home Assistant sensor integration.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_live
        .get_values(&"5726468552".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_live.<a href="/src/api/resources/device_live/client.rs">get_stats</a>(code: Option&lt;String&gt;) -> Result&lt;DeviceStats, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns current values plus day/month/year min and max for all sensors.
Each value is a `[unix_timestamp, value]` tuple.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_live
        .get_stats(
            &GetStatsQueryRequest {
                code: "5726468552".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_live.<a href="/src/api/resources/device_live/client.rs">get_info</a>(device_id: String) -> Result&lt;DeviceInfo, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Station name, location, elevation, equipment info.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_live
        .get_info(&"5726468552".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_live.<a href="/src/api/resources/device_live/client.rs">get_wind_rose</a>(code: Option&lt;String&gt;) -> Result&lt;WindData, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Wind direction distribution data for the wind rose chart.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_live
        .get_wind_rose(
            &GetWindRoseQueryRequest {
                code: "5726468552".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**code:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_live.<a href="/src/api/resources/device_live/client.rs">get_update_status</a>(request: GetUpdateStatusDeviceLiveRequest) -> Result&lt;GetUpdateStatusDeviceLiveResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns seconds since last update and device online status.

> ⚠️ **Requires `X-Requested-With: XMLHttpRequest`** header — without it the server returns an empty 200.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_live
        .get_update_status(
            &GetUpdateStatusDeviceLiveRequest {
                d: "5726468552".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**d:** `String` — Device ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.device_live.<a href="/src/api/resources/device_live/client.rs">get_owner_profile</a>(request: GetOwnerProfileDeviceLiveRequest) -> Result&lt;DeviceProfile, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns observer name, follower count, and device brand/model.

> ⚠️ **Requires `X-Requested-With: XMLHttpRequest`** header — without it the server returns an empty 200.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_live
        .get_owner_profile(
            &GetOwnerProfileDeviceLiveRequest {
                d: "5726468552".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**d:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## DeviceHistory
<details><summary><code>client.device_history.<a href="/src/api/resources/device_history/client.rs">get_evolution</a>(request: GetEvolutionDeviceHistoryRequest) -> Result&lt;EvolutionResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns hourly aggregated history for a given variable and period.

> ⚠️ **Requires `X-Requested-With: XMLHttpRequest`** header — without it the server returns an empty 200.

**Variable codes:**
| Code | Sensor |
|------|--------|
| 101  | Temperature (°C) |
| 201  | Humidity (%) |
| 541  | Dew point (°C) |
| 641  | Barometric pressure (hPa) |
| 701  | Wind speed (m/s) |
| 6001 | Wind direction (°) |
| 6501 | Wind gust / high speed (m/s) |
| 801  | Rain (mm) |
| 811  | Rain rate (mm/h) |
| 1001 | Solar radiation (W/m²) |
| 1101 | UV index |

**Period values:** `day`, `week`, `month`, `year`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .device_history
        .get_evolution(
            &GetEvolutionDeviceHistoryRequest {
                device: "5726468552".to_string(),
                variable: 101,
                period: GetEvolutionDeviceHistoryRequestPeriod::Day,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**variable:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `GetEvolutionDeviceHistoryRequestPeriod` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Forecast
<details><summary><code>client.forecast.<a href="/src/api/resources/forecast/client.rs">get_daily</a>(id: Option&lt;String&gt;) -> Result&lt;ForecastResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .forecast
        .get_daily(
            &GetDailyQueryRequest {
                id: "5726468552".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Map
<details><summary><code>client.map.<a href="/src/api/resources/map/client.rs">get_devices</a>(request: GetDevicesMapRequest) -> Result&lt;MapDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns stations visible on the map for a given location bounding box.

> ⚠️ **Requires `X-Requested-With: XMLHttpRequest`** header — without it the server returns an empty 200.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .map
        .get_devices(
            &GetDevicesMapRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user:** `Option<String>` — Filter by user (empty = all)
    
</dd>
</dl>

<dl>
<dd>

**location:** `Option<String>` — lat,lon,zoom format
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.map.<a href="/src/api/resources/map/client.rs">get_background_devices</a>(request: GetBackgroundDevicesMapRequest) -> Result&lt;MapDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .map
        .get_background_devices(
            &GetBackgroundDevicesMapRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.map.<a href="/src/api/resources/map/client.rs">get_metars</a>(request: std::collections::HashMap&lt;String, serde_json::Value&gt;) -> Result&lt;GetMetarsMapResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .map
        .get_metars(
            &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Stations
<details><summary><code>client.stations.<a href="/src/api/resources/stations/client.rs">get_nearby</a>(lat: f64, lon: f64, km: i64) -> Result&lt;PageDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns nearby stations as JSON (despite `text/html` content-type header).
All `page/*` endpoints return `PageDevice` objects that include the **station name**.
Values are scaled integers — divide by 10 (e.g. `temp: 281` = 28.1°C).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.stations.get_nearby(1.1, 1.1, 1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lat:** `f64` 
    
</dd>
</dl>

<dl>
<dd>

**lon:** `f64` 
    
</dd>
</dl>

<dl>
<dd>

**km:** `i64` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stations.<a href="/src/api/resources/stations/client.rs">get_popular</a>(country: String, period: GetPopularStationsRequestPeriod) -> Result&lt;PageDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .stations
        .get_popular(
            &"BE".to_string(),
            &GetPopularStationsRequestPeriod::Day,
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**period:** `GetPopularStationsRequestPeriod` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stations.<a href="/src/api/resources/stations/client.rs">get_newest</a>(country: String) -> Result&lt;PageDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.stations.get_newest(&"BE".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stations.<a href="/src/api/resources/stations/client.rs">get_most_followed</a>(country: String) -> Result&lt;PageDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .stations
        .get_most_followed(&"BE".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**country:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stations.<a href="/src/api/resources/stations/client.rs">get_last_views</a>() -> Result&lt;PageDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.stations.get_last_views(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stations.<a href="/src/api/resources/stations/client.rs">get_own</a>() -> Result&lt;PageDevicesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.stations.get_own(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.stations.<a href="/src/api/resources/stations/client.rs">get_station_page</a>(device_id: String) -> Result&lt;String, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

The station **name** is not available from any JSON API endpoint.
The easiest way to get it is to fetch the station's HTML page and extract
the name from the `<title>` or `og:title` meta tag.

**Example response title:**
```
WeatherStation Skyline - Weathercloud | Global network of weather stations
```

Strip everything from ` - Weathercloud` onward to get the clean station name.

> This is a plain HTML page, not a JSON API. Use it for scraping only.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .stations
        .get_station_page(&"deviceId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `String` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Metar
<details><summary><code>client.metar.<a href="/src/api/resources/metar/client.rs">get_values</a>(device_id: String) -> Result&lt;DeviceValues, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Most `device/*` routes work identically for METAR (airport) stations by replacing the `device` prefix with `metar`.

METAR station IDs are **ICAO codes** (4 letters), e.g. `EBBR` for Brussels Airport.

**Supported `metar/*` routes (same request/response as their `device/*` counterparts):**
- `GET /metar/values/{icao}` — current readings
- `GET /metar/stats?code={icao}` — statistics
- `GET /metar/wind?code={icao}` — wind rose
- `GET /metar/info/{icao}` — station metadata
- `POST /metar/ajaxupdatedate` — last update time
- `POST /metar/ajaxprofile` — station profile
- `POST /metar/evolution` — time-series history

> **Not supported for METAR:** `/device/ajaxdevicestats`
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use weathercloud_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.metar.get_values(&"EBBR".to_string(), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**device_id:** `String` — ICAO airport code
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

