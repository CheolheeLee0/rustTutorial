use chrono::Local;
use dotenv::dotenv;
use google_sheets4::api::{Spreadsheet, ValueRange};
use google_sheets4::oauth2::{read_service_account_key, ServiceAccountAuthenticator};
use google_sheets4::Sheets;
use notify_rust::Notification;
use screenshots::Screen;
use serde_json::json;
use std::error::Error;
use tesseract::Tesseract;

// 이미지 캡처 및 텍스트 추출 함수
async fn capture_and_extract_text() -> Result<String, Box<dyn Error>> {
    // 스크린샷 캡처
    let screens = Screen::all()?;
    let screen = screens[0]; // 주 모니터
    let image = screen.capture()?;

    // 임시 파일로 저장
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let temp_path = format!("temp_capture_{}.png", timestamp);
    image.save(&temp_path)?;

    // Tesseract OCR로 텍스트 추출
    let text = Tesseract::new(None, Some("eng"))? // 일단 영어만 지원하도록 변경
        .set_image(&temp_path)?
        .recognize()?
        .get_text()?;

    // 임시 파일 삭제
    std::fs::remove_file(temp_path)?;

    Ok(text)
}

// 스프레드시트에 데이터를 추가하는 함수
async fn append_to_sheet(spreadsheet_id: &str, range: &str, values: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    // 환경변수에서 서비스 계정 키 파일 경로 가져오기
    let key_path = std::env::var("GOOGLE_SERVICE_ACCOUNT_KEY")?;
    let secret = read_service_account_key(&key_path).await?;

    // OAuth2 인증
    let auth = ServiceAccountAuthenticator::builder(secret).build().await?;

    // Google Sheets API 클라이언트 생성
    let hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_only().enable_http1().build()), auth);

    // String -> serde_json::Value 변환
    let values_json = values.into_iter().map(|row| row.into_iter().map(|cell| json!(cell)).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    // 요청용 ValueRange 생성
    let req = ValueRange { values: Some(values_json), ..Default::default() };

    // 스프레드시트에 데이터 추가
    hub.spreadsheets().values_append(req, spreadsheet_id, range).value_input_option("RAW").doit().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // .env 파일 로드
    dotenv().ok();

    // 시작 알림
    Notification::new().summary("이미지 분석 시작").body("화면 캡처 및 텍스트 추출을 시작합니다.").show()?;

    // 이미지 캡처 및 텍스트 추출
    let extracted_text = capture_and_extract_text().await?;

    // 현재 시간
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 스프레드시트에 저장할 데이터 준비
    let values = vec![vec![timestamp, "텍스트 추출".to_string(), extracted_text]];

    // 스프레드시트 ID와 범위
    let spreadsheet_id = std::env::var("SPREADSHEET_ID")?;
    let range = "test_sheet!A:C"; // 시트 이름을 Sheet1으로 변경

    // 스프레드시트에 데이터 추가
    append_to_sheet(&spreadsheet_id, range, values).await?;

    // 완료 알림
    Notification::new().summary("이미지 분석 완료").body("텍스트 추출 및 저장이 완료되었습니다.").show()?;

    Ok(())
}
