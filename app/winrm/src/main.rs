use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use std::env;
use tokio;
use base64::{engine::general_purpose, Engine as _};
use uuid::Uuid;

#[derive(Deserialize)]
struct WinRmResponse {
    Caption: String,
    Name: String,
    CurrentTimeZone: i16,
    Debug: bool,
    EncryptionLevel: u32,
    ForegroundApplicationBoost: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let winrm_url = env::var("WINRM_URL")?;
    let winrm_user = env::var("WINRM_USER")?;
    let winrm_pass = env::var("WINRM_PASS")?;

    let auth = base64::encode(&format!("{}:{}", winrm_user, winrm_pass));
    let auth_header_value = format!("Basic {}", auth);

    let uuid = Uuid::new_v4();

    let wmi_query = format!(r#"
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" xmlns:a="http://schemas.xmlsoap.org/ws/2004/08/addressing" xmlns:w="http://schemas.dmtf.org/wbem/wsman/1/wsman.xsd">
  <s:Header>
    <a:Action s:mustUnderstand="1">http://schemas.xmlsoap.org/ws/2004/09/transfer/Get</a:Action>
    <a:To s:mustUnderstand="1">{}</a:To>
    <w:ResourceURI s:mustUnderstand="1">http://schemas.microsoft.com/wbem/wsman/1/wmi/root/cimv2/Win32_OperatingSystem</w:ResourceURI>
    <a:MessageID>uuid:{}</a:MessageID>
    <a:ReplyTo>
      <a:Address s:mustUnderstand="1">http://schemas.xmlsoap.org/ws/2004/08/addressing/role/anonymous</a:Address>
    </a:ReplyTo>
  </s:Header>
  <s:Body />
</s:Envelope>
    "#, winrm_url, uuid);

    let client = reqwest::Client::new();

    let response = client.post(winrm_url)
        .header(AUTHORIZATION, auth_header_value)
        .header(CONTENT_TYPE, "application/soap+xml;charset=UTF-8")
        .body(wmi_query)
        .send()
        .await?;

    let body_str = response.text().await?;

    let winrm_response: WinRmResponse = serde_xml_rs::from_str(&body_str)?;

    // 必要なデータをWinRmResponseから取得し、表示または処理します
    // 例: println!("OS Name: {}", winrm_response.os_name);

    Ok(())
}
