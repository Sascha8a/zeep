//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.11
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetWeatherInformationSoapIn")]
    pub struct GetWeatherInformationSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetWeatherInformation,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetWeatherInformationSoapOut")]
    pub struct GetWeatherInformationSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetWeatherInformationResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityForecastByZIPSoapIn")]
    pub struct GetCityForecastByZIPSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCityForecastByZIP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityForecastByZIPSoapOut")]
    pub struct GetCityForecastByZIPSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCityForecastByZIPResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityWeatherByZIPSoapIn")]
    pub struct GetCityWeatherByZIPSoapIn {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCityWeatherByZIP,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityWeatherByZIPSoapOut")]
    pub struct GetCityWeatherByZIPSoapOut {
        #[yaserde(flatten, default)]
        pub parameters: types::GetCityWeatherByZIPResponse,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetWeatherInformationHttpGetIn")]
    pub struct GetWeatherInformationHttpGetIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetWeatherInformationHttpGetOut")]
    pub struct GetWeatherInformationHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::ArrayOfWeatherDescription,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityForecastByZIPHttpGetIn")]
    pub struct GetCityForecastByZIPHttpGetIn {
        #[yaserde(rename = "ZIP", default)]
        pub zip: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityForecastByZIPHttpGetOut")]
    pub struct GetCityForecastByZIPHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::ForecastReturn,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityWeatherByZIPHttpGetIn")]
    pub struct GetCityWeatherByZIPHttpGetIn {
        #[yaserde(rename = "ZIP", default)]
        pub zip: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityWeatherByZIPHttpGetOut")]
    pub struct GetCityWeatherByZIPHttpGetOut {
        #[yaserde(flatten, default)]
        pub body: types::WeatherReturn,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetWeatherInformationHttpPostIn")]
    pub struct GetWeatherInformationHttpPostIn {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetWeatherInformationHttpPostOut")]
    pub struct GetWeatherInformationHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::ArrayOfWeatherDescription,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityForecastByZIPHttpPostIn")]
    pub struct GetCityForecastByZIPHttpPostIn {
        #[yaserde(rename = "ZIP", default)]
        pub zip: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityForecastByZIPHttpPostOut")]
    pub struct GetCityForecastByZIPHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::ForecastReturn,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityWeatherByZIPHttpPostIn")]
    pub struct GetCityWeatherByZIPHttpPostIn {
        #[yaserde(rename = "ZIP", default)]
        pub zip: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "GetCityWeatherByZIPHttpPostOut")]
    pub struct GetCityWeatherByZIPHttpPostOut {
        #[yaserde(flatten, default)]
        pub body: types::WeatherReturn,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetWeatherInformation",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetWeatherInformation {}
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetWeatherInformationResponse",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetWeatherInformationResponse {
        #[yaserde(rename = "GetWeatherInformationResult", prefix = "tns", default)]
        pub get_weather_information_result: Option<ArrayOfWeatherDescription>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfWeatherDescription",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        prefix = "tns"
    )]
    pub struct ArrayOfWeatherDescription {
        #[yaserde(rename = "WeatherDescription", prefix = "tns", default)]
        pub weather_description: Vec<WeatherDescription>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WeatherDescription",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        prefix = "tns"
    )]
    pub struct WeatherDescription {
        #[yaserde(rename = "WeatherID", prefix = "tns", default)]
        pub weather_id: i16,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "PictureURL", prefix = "tns", default)]
        pub picture_url: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCityForecastByZIP",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCityForecastByZIP {
        #[yaserde(rename = "ZIP", prefix = "tns", default)]
        pub zip: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCityForecastByZIPResponse",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCityForecastByZIPResponse {
        #[yaserde(rename = "GetCityForecastByZIPResult", prefix = "tns", default)]
        pub get_city_forecast_by_zip_result: Option<ForecastReturn>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ForecastReturn",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        prefix = "tns"
    )]
    pub struct ForecastReturn {
        #[yaserde(rename = "Success", prefix = "tns", default)]
        pub success: bool,
        #[yaserde(rename = "ResponseText", prefix = "tns", default)]
        pub response_text: Option<String>,
        #[yaserde(rename = "State", prefix = "tns", default)]
        pub state: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "WeatherStationCity", prefix = "tns", default)]
        pub weather_station_city: Option<String>,
        #[yaserde(rename = "ForecastResult", prefix = "tns", default)]
        pub forecast_result: Option<ArrayOfForecast>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "ArrayOfForecast",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        prefix = "tns"
    )]
    pub struct ArrayOfForecast {
        #[yaserde(rename = "Forecast", prefix = "tns", default)]
        pub forecast: Vec<Forecast>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "Forecast",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        prefix = "tns"
    )]
    pub struct Forecast {
        #[yaserde(rename = "Date", prefix = "tns", default)]
        pub date: String,
        #[yaserde(rename = "WeatherID", prefix = "tns", default)]
        pub weather_id: i16,
        #[yaserde(rename = "Desciption", prefix = "tns", default)]
        pub desciption: Option<String>,
        #[yaserde(rename = "Temperatures", prefix = "tns", default)]
        pub temperatures: Temp,
        #[yaserde(rename = "ProbabilityOfPrecipiation", prefix = "tns", default)]
        pub probability_of_precipiation: Pop,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "temp", namespace = "tns: http://ws.cdyne.com/WeatherWS/", prefix = "tns")]
    pub struct Temp {
        #[yaserde(rename = "MorningLow", prefix = "tns", default)]
        pub morning_low: Option<String>,
        #[yaserde(rename = "DaytimeHigh", prefix = "tns", default)]
        pub daytime_high: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "POP", namespace = "tns: http://ws.cdyne.com/WeatherWS/", prefix = "tns")]
    pub struct Pop {
        #[yaserde(rename = "Nighttime", prefix = "tns", default)]
        pub nighttime: Option<String>,
        #[yaserde(rename = "Daytime", prefix = "tns", default)]
        pub daytime: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCityWeatherByZIP",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCityWeatherByZIP {
        #[yaserde(rename = "ZIP", prefix = "tns", default)]
        pub zip: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "GetCityWeatherByZIPResponse",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "tns"
    )]
    pub struct GetCityWeatherByZIPResponse {
        #[yaserde(rename = "GetCityWeatherByZIPResult", prefix = "tns", default)]
        pub get_city_weather_by_zip_result: WeatherReturn,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "WeatherReturn",
        namespace = "tns: http://ws.cdyne.com/WeatherWS/",
        prefix = "tns"
    )]
    pub struct WeatherReturn {
        #[yaserde(rename = "Success", prefix = "tns", default)]
        pub success: bool,
        #[yaserde(rename = "ResponseText", prefix = "tns", default)]
        pub response_text: Option<String>,
        #[yaserde(rename = "State", prefix = "tns", default)]
        pub state: Option<String>,
        #[yaserde(rename = "City", prefix = "tns", default)]
        pub city: Option<String>,
        #[yaserde(rename = "WeatherStationCity", prefix = "tns", default)]
        pub weather_station_city: Option<String>,
        #[yaserde(rename = "WeatherID", prefix = "tns", default)]
        pub weather_id: i16,
        #[yaserde(rename = "Description", prefix = "tns", default)]
        pub description: Option<String>,
        #[yaserde(rename = "Temperature", prefix = "tns", default)]
        pub temperature: Option<String>,
        #[yaserde(rename = "RelativeHumidity", prefix = "tns", default)]
        pub relative_humidity: Option<String>,
        #[yaserde(rename = "Wind", prefix = "tns", default)]
        pub wind: Option<String>,
        #[yaserde(rename = "Pressure", prefix = "tns", default)]
        pub pressure: Option<String>,
        #[yaserde(rename = "Visibility", prefix = "tns", default)]
        pub visibility: Option<String>,
        #[yaserde(rename = "WindChill", prefix = "tns", default)]
        pub wind_chill: Option<String>,
        #[yaserde(rename = "Remarks", prefix = "tns", default)]
        pub remarks: Option<String>,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    pub type GetWeatherInformationSoapIn = messages::GetWeatherInformationSoapIn;

    pub type GetWeatherInformationSoapOut = messages::GetWeatherInformationSoapOut;

    pub type GetCityForecastByZIPSoapIn = messages::GetCityForecastByZIPSoapIn;

    pub type GetCityForecastByZIPSoapOut = messages::GetCityForecastByZIPSoapOut;

    pub type GetCityWeatherByZIPSoapIn = messages::GetCityWeatherByZIPSoapIn;

    pub type GetCityWeatherByZIPSoapOut = messages::GetCityWeatherByZIPSoapOut;

    #[async_trait]
    pub trait WeatherSoap {
        async fn get_weather_information(
            &self,
            get_weather_information_soap_in: GetWeatherInformationSoapIn,
        ) -> Result<GetWeatherInformationSoapOut, Option<SoapFault>>;
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_soap_in: GetCityForecastByZIPSoapIn,
        ) -> Result<GetCityForecastByZIPSoapOut, Option<SoapFault>>;
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_soap_in: GetCityWeatherByZIPSoapIn,
        ) -> Result<GetCityWeatherByZIPSoapOut, Option<SoapFault>>;
    }
    pub type GetWeatherInformationHttpGetIn = messages::GetWeatherInformationHttpGetIn;

    pub type GetWeatherInformationHttpGetOut = messages::GetWeatherInformationHttpGetOut;

    pub type GetCityForecastByZIPHttpGetIn = messages::GetCityForecastByZIPHttpGetIn;

    pub type GetCityForecastByZIPHttpGetOut = messages::GetCityForecastByZIPHttpGetOut;

    pub type GetCityWeatherByZIPHttpGetIn = messages::GetCityWeatherByZIPHttpGetIn;

    pub type GetCityWeatherByZIPHttpGetOut = messages::GetCityWeatherByZIPHttpGetOut;

    #[async_trait]
    pub trait WeatherHttpGet {
        async fn get_weather_information(
            &self,
            get_weather_information_http_get_in: GetWeatherInformationHttpGetIn,
        ) -> Result<GetWeatherInformationHttpGetOut, Option<SoapFault>>;
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_get_in: GetCityForecastByZIPHttpGetIn,
        ) -> Result<GetCityForecastByZIPHttpGetOut, Option<SoapFault>>;
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_get_in: GetCityWeatherByZIPHttpGetIn,
        ) -> Result<GetCityWeatherByZIPHttpGetOut, Option<SoapFault>>;
    }
    pub type GetWeatherInformationHttpPostIn = messages::GetWeatherInformationHttpPostIn;

    pub type GetWeatherInformationHttpPostOut = messages::GetWeatherInformationHttpPostOut;

    pub type GetCityForecastByZIPHttpPostIn = messages::GetCityForecastByZIPHttpPostIn;

    pub type GetCityForecastByZIPHttpPostOut = messages::GetCityForecastByZIPHttpPostOut;

    pub type GetCityWeatherByZIPHttpPostIn = messages::GetCityWeatherByZIPHttpPostIn;

    pub type GetCityWeatherByZIPHttpPostOut = messages::GetCityWeatherByZIPHttpPostOut;

    #[async_trait]
    pub trait WeatherHttpPost {
        async fn get_weather_information(
            &self,
            get_weather_information_http_post_in: GetWeatherInformationHttpPostIn,
        ) -> Result<GetWeatherInformationHttpPostOut, Option<SoapFault>>;
        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_post_in: GetCityForecastByZIPHttpPostIn,
        ) -> Result<GetCityForecastByZIPHttpPostOut, Option<SoapFault>>;
        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_post_in: GetCityWeatherByZIPHttpPostIn,
        ) -> Result<GetCityWeatherByZIPHttpPostOut, Option<SoapFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};

    impl WeatherSoap {
        async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(credentials.0.to_string(), Some(credentials.1.to_string()));
            }
            trace!("SOAP Request: {:?}", req);
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationSoapIn {
        #[yaserde(rename = "tns:GetWeatherInformation", default)]
        pub body: ports::GetWeatherInformationSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationSoapIn,
    }

    impl GetWeatherInformationSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetWeatherInformationSoapIn) -> Self {
            GetWeatherInformationSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationSoapOut {
        #[yaserde(rename = "GetWeatherInformationResponse", default)]
        pub body: Option<ports::GetWeatherInformationSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationSoapOut,
    }

    impl GetWeatherInformationSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetWeatherInformationSoapOut) -> Self {
            GetWeatherInformationSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPSoapIn {
        #[yaserde(rename = "tns:GetCityForecastByZIP", default)]
        pub body: ports::GetCityForecastByZIPSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPSoapIn,
    }

    impl GetCityForecastByZIPSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityForecastByZIPSoapIn) -> Self {
            GetCityForecastByZIPSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPSoapOut {
        #[yaserde(rename = "GetCityForecastByZIPResponse", default)]
        pub body: Option<ports::GetCityForecastByZIPSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPSoapOut,
    }

    impl GetCityForecastByZIPSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityForecastByZIPSoapOut) -> Self {
            GetCityForecastByZIPSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPSoapIn {
        #[yaserde(rename = "tns:GetCityWeatherByZIP", default)]
        pub body: ports::GetCityWeatherByZIPSoapIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPSoapInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPSoapIn,
    }

    impl GetCityWeatherByZIPSoapInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityWeatherByZIPSoapIn) -> Self {
            GetCityWeatherByZIPSoapInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPSoapOut {
        #[yaserde(rename = "GetCityWeatherByZIPResponse", default)]
        pub body: Option<ports::GetCityWeatherByZIPSoapOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPSoapOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPSoapOut,
    }

    impl GetCityWeatherByZIPSoapOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityWeatherByZIPSoapOut) -> Self {
            GetCityWeatherByZIPSoapOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for WeatherSoap {
        fn default() -> Self {
            WeatherSoap {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: None,
            }
        }
    }
    impl WeatherSoap {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherSoap {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct WeatherSoap {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::WeatherSoap for WeatherSoap {
        async fn get_weather_information(
            &self,
            get_weather_information_soap_in: ports::GetWeatherInformationSoapIn,
        ) -> Result<ports::GetWeatherInformationSoapOut, Option<SoapFault>> {
            let __request = GetWeatherInformationSoapInSoapEnvelope::new(SoapGetWeatherInformationSoapIn {
                body: get_weather_information_soap_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS/GetWeatherInformation")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_soap_in: ports::GetCityForecastByZIPSoapIn,
        ) -> Result<ports::GetCityForecastByZIPSoapOut, Option<SoapFault>> {
            let __request = GetCityForecastByZIPSoapInSoapEnvelope::new(SoapGetCityForecastByZIPSoapIn {
                body: get_city_forecast_by_zip_soap_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS/GetCityForecastByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_soap_in: ports::GetCityWeatherByZIPSoapIn,
        ) -> Result<ports::GetCityWeatherByZIPSoapOut, Option<SoapFault>> {
            let __request = GetCityWeatherByZIPSoapInSoapEnvelope::new(SoapGetCityWeatherByZIPSoapIn {
                body: get_city_weather_by_zip_soap_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS/GetCityWeatherByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl WeatherSoap12 {
        async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(credentials.0.to_string(), Some(credentials.1.to_string()));
            }
            trace!("SOAP Request: {:?}", req);
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    impl Default for WeatherSoap12 {
        fn default() -> Self {
            WeatherSoap12 {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: None,
            }
        }
    }
    impl WeatherSoap12 {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherSoap12 {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct WeatherSoap12 {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::WeatherSoap for WeatherSoap12 {
        async fn get_weather_information(
            &self,
            get_weather_information_soap_in: ports::GetWeatherInformationSoapIn,
        ) -> Result<ports::GetWeatherInformationSoapOut, Option<SoapFault>> {
            let __request = GetWeatherInformationSoapInSoapEnvelope::new(SoapGetWeatherInformationSoapIn {
                body: get_weather_information_soap_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS/GetWeatherInformation")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_soap_in: ports::GetCityForecastByZIPSoapIn,
        ) -> Result<ports::GetCityForecastByZIPSoapOut, Option<SoapFault>> {
            let __request = GetCityForecastByZIPSoapInSoapEnvelope::new(SoapGetCityForecastByZIPSoapIn {
                body: get_city_forecast_by_zip_soap_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS/GetCityForecastByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_soap_in: ports::GetCityWeatherByZIPSoapIn,
        ) -> Result<ports::GetCityWeatherByZIPSoapOut, Option<SoapFault>> {
            let __request = GetCityWeatherByZIPSoapInSoapEnvelope::new(SoapGetCityWeatherByZIPSoapIn {
                body: get_city_weather_by_zip_soap_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS/GetCityWeatherByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl WeatherHttpGet {
        async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(credentials.0.to_string(), Some(credentials.1.to_string()));
            }
            trace!("SOAP Request: {:?}", req);
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpGetIn {
        #[yaserde(rename = "tns:GetWeatherInformation", default)]
        pub body: ports::GetWeatherInformationHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpGetInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpGetIn,
    }

    impl GetWeatherInformationHttpGetInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetWeatherInformationHttpGetIn) -> Self {
            GetWeatherInformationHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpGetOut {
        #[yaserde(rename = "ArrayOfWeatherDescription", default)]
        pub body: Option<ports::GetWeatherInformationHttpGetOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpGetOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpGetOut,
    }

    impl GetWeatherInformationHttpGetOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetWeatherInformationHttpGetOut) -> Self {
            GetWeatherInformationHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpGetIn {
        #[yaserde(rename = "tns:GetCityForecastByZIP", default)]
        pub body: ports::GetCityForecastByZIPHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpGetInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpGetIn,
    }

    impl GetCityForecastByZIPHttpGetInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityForecastByZIPHttpGetIn) -> Self {
            GetCityForecastByZIPHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpGetOut {
        #[yaserde(rename = "ForecastReturn", default)]
        pub body: Option<ports::GetCityForecastByZIPHttpGetOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpGetOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpGetOut,
    }

    impl GetCityForecastByZIPHttpGetOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityForecastByZIPHttpGetOut) -> Self {
            GetCityForecastByZIPHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpGetIn {
        #[yaserde(rename = "tns:GetCityWeatherByZIP", default)]
        pub body: ports::GetCityWeatherByZIPHttpGetIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpGetInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpGetIn,
    }

    impl GetCityWeatherByZIPHttpGetInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityWeatherByZIPHttpGetIn) -> Self {
            GetCityWeatherByZIPHttpGetInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpGetOut {
        #[yaserde(rename = "WeatherReturn", default)]
        pub body: Option<ports::GetCityWeatherByZIPHttpGetOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpGetOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpGetOut,
    }

    impl GetCityWeatherByZIPHttpGetOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityWeatherByZIPHttpGetOut) -> Self {
            GetCityWeatherByZIPHttpGetOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for WeatherHttpGet {
        fn default() -> Self {
            WeatherHttpGet {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: None,
            }
        }
    }
    impl WeatherHttpGet {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherHttpGet {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct WeatherHttpGet {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::WeatherHttpGet for WeatherHttpGet {
        async fn get_weather_information(
            &self,
            get_weather_information_http_get_in: ports::GetWeatherInformationHttpGetIn,
        ) -> Result<ports::GetWeatherInformationHttpGetOut, Option<SoapFault>> {
            let __request = GetWeatherInformationHttpGetInSoapEnvelope::new(SoapGetWeatherInformationHttpGetIn {
                body: get_weather_information_http_get_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS//GetWeatherInformation")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_get_in: ports::GetCityForecastByZIPHttpGetIn,
        ) -> Result<ports::GetCityForecastByZIPHttpGetOut, Option<SoapFault>> {
            let __request = GetCityForecastByZIPHttpGetInSoapEnvelope::new(SoapGetCityForecastByZIPHttpGetIn {
                body: get_city_forecast_by_zip_http_get_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS//GetCityForecastByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_get_in: ports::GetCityWeatherByZIPHttpGetIn,
        ) -> Result<ports::GetCityWeatherByZIPHttpGetOut, Option<SoapFault>> {
            let __request = GetCityWeatherByZIPHttpGetInSoapEnvelope::new(SoapGetCityWeatherByZIPHttpGetIn {
                body: get_city_weather_by_zip_http_get_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS//GetCityWeatherByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPHttpGetOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl WeatherHttpPost {
        async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(credentials.0.to_string(), Some(credentials.1.to_string()));
            }
            trace!("SOAP Request: {:?}", req);
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpPostIn {
        #[yaserde(rename = "tns:GetWeatherInformation", default)]
        pub body: ports::GetWeatherInformationHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpPostInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpPostIn,
    }

    impl GetWeatherInformationHttpPostInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetWeatherInformationHttpPostIn) -> Self {
            GetWeatherInformationHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetWeatherInformationHttpPostOut {
        #[yaserde(rename = "ArrayOfWeatherDescription", default)]
        pub body: Option<ports::GetWeatherInformationHttpPostOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetWeatherInformationHttpPostOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetWeatherInformationHttpPostOut,
    }

    impl GetWeatherInformationHttpPostOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetWeatherInformationHttpPostOut) -> Self {
            GetWeatherInformationHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpPostIn {
        #[yaserde(rename = "tns:GetCityForecastByZIP", default)]
        pub body: ports::GetCityForecastByZIPHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpPostInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpPostIn,
    }

    impl GetCityForecastByZIPHttpPostInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityForecastByZIPHttpPostIn) -> Self {
            GetCityForecastByZIPHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityForecastByZIPHttpPostOut {
        #[yaserde(rename = "ForecastReturn", default)]
        pub body: Option<ports::GetCityForecastByZIPHttpPostOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityForecastByZIPHttpPostOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityForecastByZIPHttpPostOut,
    }

    impl GetCityForecastByZIPHttpPostOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityForecastByZIPHttpPostOut) -> Self {
            GetCityForecastByZIPHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpPostIn {
        #[yaserde(rename = "tns:GetCityWeatherByZIP", default)]
        pub body: ports::GetCityWeatherByZIPHttpPostIn,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpPostInSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpPostIn,
    }

    impl GetCityWeatherByZIPHttpPostInSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityWeatherByZIPHttpPostIn) -> Self {
            GetCityWeatherByZIPHttpPostInSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetCityWeatherByZIPHttpPostOut {
        #[yaserde(rename = "WeatherReturn", default)]
        pub body: Option<ports::GetCityWeatherByZIPHttpPostOut>,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetCityWeatherByZIPHttpPostOutSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: Option<String>,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetCityWeatherByZIPHttpPostOut,
    }

    impl GetCityWeatherByZIPHttpPostOutSoapEnvelope {
        #[must_use]
        pub fn new(body: SoapGetCityWeatherByZIPHttpPostOut) -> Self {
            GetCityWeatherByZIPHttpPostOutSoapEnvelope {
                encoding_style: Some(SOAP_ENCODING.to_string()),
                tnsattr: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for WeatherHttpPost {
        fn default() -> Self {
            WeatherHttpPost {
                client: reqwest::Client::new(),
                url: "http://ws.cdyne.com/WeatherWS/".to_string(),
                credentials: None,
            }
        }
    }
    impl WeatherHttpPost {
        #[must_use]
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            WeatherHttpPost {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct WeatherHttpPost {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::WeatherHttpPost for WeatherHttpPost {
        async fn get_weather_information(
            &self,
            get_weather_information_http_post_in: ports::GetWeatherInformationHttpPostIn,
        ) -> Result<ports::GetWeatherInformationHttpPostOut, Option<SoapFault>> {
            let __request = GetWeatherInformationHttpPostInSoapEnvelope::new(SoapGetWeatherInformationHttpPostIn {
                body: get_weather_information_http_post_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS//GetWeatherInformation")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetWeatherInformationHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_forecast_by_zip(
            &self,
            get_city_forecast_by_zip_http_post_in: ports::GetCityForecastByZIPHttpPostIn,
        ) -> Result<ports::GetCityForecastByZIPHttpPostOut, Option<SoapFault>> {
            let __request = GetCityForecastByZIPHttpPostInSoapEnvelope::new(SoapGetCityForecastByZIPHttpPostIn {
                body: get_city_forecast_by_zip_http_post_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS//GetCityForecastByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityForecastByZIPHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }

        async fn get_city_weather_by_zip(
            &self,
            get_city_weather_by_zip_http_post_in: ports::GetCityWeatherByZIPHttpPostIn,
        ) -> Result<ports::GetCityWeatherByZIPHttpPostOut, Option<SoapFault>> {
            let __request = GetCityWeatherByZIPHttpPostInSoapEnvelope::new(SoapGetCityWeatherByZIPHttpPostIn {
                body: get_city_weather_by_zip_http_post_in,
                xmlns: Some("http://ws.cdyne.com/WeatherWS/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://ws.cdyne.com/WeatherWS//GetCityWeatherByZIP")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetCityWeatherByZIPHttpPostOutSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body.expect("missing body"))
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::{de::from_str, ser::to_string, YaDeserialize, YaSerialize};
    pub struct Weather {}
    impl Weather {
        #[must_use]
        pub fn new_client(credentials: Option<(String, String)>) -> bindings::WeatherSoap {
            Self::new_client_with_url("http://wsf.cdyne.com/WeatherWS/Weather.asmx", credentials)
        }

        #[must_use]
        pub fn new_client_with_url(url: &str, credentials: Option<(String, String)>) -> bindings::WeatherSoap {
            bindings::WeatherSoap::new(url, credentials)
        }
    }
}

pub mod multiref {
    //! This module contains the `MultiRef` type which is a wrapper around `Rc<RefCell<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
    //! Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
    //! Needs `xml-rs` and `yaserde` as dependencies.

    use std::{cell::RefCell, ops::Deref, rc::Rc};
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Rc<RefCell<T>>,
    }

    impl<T: YaDeserialize + YaSerialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self {
                inner: Rc::new(RefCell::new(inner)),
            })
        }
    }

    impl<T: YaDeserialize + YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.as_ref().borrow().serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
            self.inner.as_ref().borrow().serialize_attributes(attributes, namespace)
        }
    }

    impl<T: YaDeserialize + YaSerialize + Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self { inner: Rc::default() }
        }
    }

    impl<T: YaDeserialize + YaSerialize> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: YaDeserialize + YaSerialize + std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.as_ref().borrow().fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
