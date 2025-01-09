use dotenv::dotenv;
use google_sheets4::{
    api::ValueRange,
    oauth2::{read_service_account_key, ServiceAccountAuthenticator},
    Sheets,
};
use hyper;
use hyper_rustls;
use serde_json::json;
use std::env;
use std::error::Error;

async fn append_to_sheet(spreadsheet_id: &str, range: &str, values: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    // 환경변수에서 서비스 계정 키 파일 경로 가져오기
    let key_path = env::var("GOOGLE_SERVICE_ACCOUNT_KEY")?;
    let secret = read_service_account_key(&key_path).await?;

    // OAuth2 인증 (google_sheets4가 re-export하는 ServiceAccountAuthenticator)
    let auth = ServiceAccountAuthenticator::builder(secret).build().await?;

    // Sheets API 클라이언트 생성
    let hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_only().enable_http1().build()), auth);

    // String -> serde_json::Value 변환
    let values_json = values
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| json!(cell)) // 각 String을 json!("문자열")로
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    // 요청용 ValueRange 생성
    let req = ValueRange { values: Some(values_json), ..Default::default() };

    // 스프레드시트에 데이터 추가
    let result = hub.spreadsheets().values_append(req, spreadsheet_id, range).value_input_option("RAW").doit().await?;

    println!("데이터가 성공적으로 추가되었습니다: {:?}", result);
    Ok(())
}

async fn create_spreadsheet(title: &str) -> Result<String, Box<dyn Error>> {
    let key_path = env::var("GOOGLE_SERVICE_ACCOUNT_KEY")?;
    let secret = read_service_account_key(&key_path).await?;
    let auth = ServiceAccountAuthenticator::builder(secret).build().await?;
    let hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_only().enable_http1().build()), auth);

    // 새로운 스프레드시트 생성 요청
    let spreadsheet = hub
        .spreadsheets()
        .create(google_sheets4::api::Spreadsheet { properties: Some(google_sheets4::api::SpreadsheetProperties { title: Some(title.to_string()), ..Default::default() }), ..Default::default() })
        .doit()
        .await?;

    // 생성된 스프레드시트의 ID 반환
    Ok(spreadsheet.1.spreadsheet_id.unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // .env 파일 로드
    dotenv().ok();

    // 환경변수에서 스프레드시트 ID 가져오기
    let spreadsheet_id = env::var("SPREADSHEET_ID")?;

    // 범위를 test_sheet로 변경
    let range = "test_sheet!A1:B2";

    // 추가할 데이터 (String)
    let values = vec![vec!["이름".to_string(), "점수".to_string()], vec!["홍길동".to_string(), "100".to_string()]];

    // 스프레드시트에 데이터 추가
    append_to_sheet(&spreadsheet_id, &range, values).await?;

    println!("데이터가 스프레드시트에 추가되었습니다.");
    println!("스프레드시트 ID: {}", spreadsheet_id);

    Ok(())
}
