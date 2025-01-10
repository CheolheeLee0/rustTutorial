use chrono::Local;
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

    // 새로운 시트 추가 요청 생성 (기본 필터 포함)
    let request = google_sheets4::api::BatchUpdateSpreadsheetRequest {
        requests: Some(vec![
            google_sheets4::api::Request {
                add_sheet: Some(google_sheets4::api::AddSheetRequest { properties: Some(google_sheets4::api::SheetProperties { title: Some(sheet_title.to_string()), ..Default::default() }) }),
                ..Default::default()
            },
            // 필터 추가
            google_sheets4::api::Request {
                set_basic_filter: Some(google_sheets4::api::SetBasicFilterRequest {
                    filter: Some(google_sheets4::api::BasicFilter {
                        range: Some(google_sheets4::api::GridRange {
                            sheet_id: None, // 새로 생성된 시트에 자동 적용
                            start_row_index: Some(0),
                            end_row_index: None,
                            start_column_index: Some(0),
                            end_column_index: Some(5), // A부터 E까지 (5개 컬럼)
                        }),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };

    // 스프레드시트 업데이트 실행
    hub.spreadsheets().batch_update(request, spreadsheet_id).doit().await?;

    Ok(())
}

async fn create_sheet_with_headers(spreadsheet_id: &str, sheet_title: &str, headers: Vec<String>) -> Result<(), Box<dyn Error>> {
    let key_path = env::var("GOOGLE_SERVICE_ACCOUNT_KEY")?;
    let secret = read_service_account_key(&key_path).await?;
    let auth = ServiceAccountAuthenticator::builder(secret).build().await?;
    let hub = Sheets::new(hyper::Client::builder().build(hyper_rustls::HttpsConnectorBuilder::new().with_native_roots().https_only().enable_http1().build()), auth);

    // 새로운 시트 추가 요청
    let request = google_sheets4::api::BatchUpdateSpreadsheetRequest {
        requests: Some(vec![google_sheets4::api::Request {
            add_sheet: Some(google_sheets4::api::AddSheetRequest { properties: Some(google_sheets4::api::SheetProperties { title: Some(sheet_title.to_string()), ..Default::default() }) }),
            ..Default::default()
        }]),
        ..Default::default()
    };

    // 시트 생성
    hub.spreadsheets().batch_update(request, spreadsheet_id).doit().await?;

    // 헤더 추가
    let range = format!("{}!A1:{}", sheet_title, get_column_letter(headers.len()));
    let values = vec![headers.iter().map(|h| json!(h)).collect::<Vec<_>>()];

    let value_range = ValueRange { values: Some(values), ..Default::default() };

    hub.spreadsheets().values_update(value_range, spreadsheet_id, &range).value_input_option("RAW").doit().await?;

    Ok(())
}

// 열 번호를 알파벳으로 변환하는 헬퍼 함수
fn get_column_letter(column_number: usize) -> String {
    let mut column = String::new();
    let mut n = column_number;

    while n > 0 {
        n -= 1;
        let c = ((n % 26) as u8 + b'A') as char;
        column.insert(0, c);
        n /= 26;
    }

    column
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let spreadsheet_id = env::var("SPREADSHEET_ID")?;
    // 프로젝트 이름 환경변수에서 가져오기
    let project_name = env::var("PROJECT_NAME")?;

    // 1. Project Overview 시트 생성
    let project_overview_headers =
        vec!["Project_ID", "Project_Name", "Project_Manager", "Start_Date", "End_Date", "Project_Goal", "Budget", "Key_Stakeholders", "Status", "Description"].into_iter().map(String::from).collect();

    create_sheet_with_headers(&spreadsheet_id, &format!("Project_Overview_{}", project_name), project_overview_headers).await?;

    // 2. Task Management 시트 생성
    let task_management_headers =
        vec!["Project_ID", "Task_ID", "Parent_Task_ID", "Task_Name", "Description", "Priority", "Assignee", "Accountable", "Start_Date", "End_Date", "Status"].into_iter().map(String::from).collect();

    create_sheet_with_headers(&spreadsheet_id, &format!("Task_Management_{}", project_name), task_management_headers).await?;

    // 3. Roles & Responsibilities 시트 생성
    let roles_headers = vec!["Name", "Role", "Department", "Skills", "Projects", "Availability", "Email", "Slack", "Notes"].into_iter().map(String::from).collect();

    create_sheet_with_headers(&spreadsheet_id, &format!("Roles_Responsibilities_{}", project_name), roles_headers).await?;

    println!("스프레드시트 구조가 성공적으로 생성되었습니다.");
    println!("스프레드시트 ID: {}", spreadsheet_id);
    println!("프로젝트 이름: {}", project_name);

    Ok(())
}
