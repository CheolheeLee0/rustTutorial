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

async fn create_sheet(spreadsheet_id: &str, sheet_title: &str) -> Result<(), Box<dyn Error>> {
    let key_path = env::var("GOOGLE_SERVICE_ACCOUNT_KEY")?;
    let secret = read_service_account_key(&key_path).await?;
    let auth = ServiceAccountAuthenticator::builder(secret).build().await?;
    let hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_only().enable_http1().build()), auth);

    // 새로운 시트 추가 요청 생성
    let request = google_sheets4::api::BatchUpdateSpreadsheetRequest {
        requests: Some(vec![google_sheets4::api::Request {
            add_sheet: Some(google_sheets4::api::AddSheetRequest { properties: Some(google_sheets4::api::SheetProperties { title: Some(sheet_title.to_string()), ..Default::default() }) }),
            ..Default::default()
        }]),
        ..Default::default()
    };

    // 스프레드시트 업데이트 실행
    hub.spreadsheets().batch_update(request, spreadsheet_id).doit().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // .env 파일 로드
    dotenv().ok();

    // 환경변수에서 스프레드시트 ID 가져오기
    let spreadsheet_id = env::var("SPREADSHEET_ID")?;
    let new_sheet_name = "employee_data";

    // 새로운 시트 생성
    create_sheet(&spreadsheet_id, new_sheet_name).await?;
    println!("새로운 시트가 생성되었습니다: {}", new_sheet_name);

    // 범위를 새로운 데이터 크기에 맞게 조정
    let range = "employee_data!A1:D7";

    // 헤더와 데이터 추가
    let values = vec![
        // 헤더
        vec!["No".to_string(), "employee_id".to_string(), "banker_url".to_string(), "customer_url".to_string()],
        // 데이터 행
        vec![
            "1".to_string(),
            "00000001".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/user/02/00000001?pass=00000001".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/chat?ref=5a957ef73bfa445fadbed5859fa152f4&branch=kyungnam".to_string(),
        ],
        vec![
            "2".to_string(),
            "00000002".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/user/02/00000002?pass=00000002".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/chat?ref=f01775bff0f84d8aaa54ab145b26811f&branch=kyungnam".to_string(),
        ],
        vec![
            "3".to_string(),
            "0000001".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/user/01/0000001?pass=0000001".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/chat?ref=46066d803a454cf4a634b067929967ba&branch=busan".to_string(),
        ],
        vec![
            "4".to_string(),
            "0000002".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/user/01/0000002?pass=0000002".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/chat?ref=338768b69f6749129dcf563ed5ba5644&branch=busan".to_string(),
        ],
        vec![
            "5".to_string(),
            "0000003".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/user/01/0000003?pass=0000003".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/chat?ref=083984fd0fca457d87d21e453e1d8e7f&branch=busan".to_string(),
        ],
        vec![
            "6".to_string(),
            "0000004".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/user/01/0000004?pass=0000004".to_string(),
            "https://rtt-client-alpha.43.201.242.240.sslip.io/chat?ref=ed6d6bd4841e43e6b87206bb7493cbc1&branch=busan".to_string(),
        ],
    ];

    // 스프레드시트에 데이터 추가
    append_to_sheet(&spreadsheet_id, &range, values).await?;

    println!("데이터가 스프레드시트에 추가되었습니다.");
    println!("스프레드시트 ID: {}", spreadsheet_id);

    Ok(())
}
