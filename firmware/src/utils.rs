use anyhow::Result;
use esp_idf_svc::wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi};

const SSID: &str = env!("WIFI_SSID");
const PASSWORD: &str = env!("WIFI_PASS");

pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> Result<()> {
    let config: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        bssid: None,
        auth_method: esp_idf_svc::wifi::AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&config)?;
    wifi.start()?;
    log::info!("Attempt start wifi");

    wifi.connect()?;
    log::info!("Connected");

    wifi.wait_netif_up()?;

    log::info!("Wifi netif up");

    Ok(())
}
