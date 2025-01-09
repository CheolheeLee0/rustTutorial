use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// 메모 구조체 정의
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Memo {
    id: u64,
    title: String,
    content: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

// 메모 생성을 위한 요청 구조체
#[derive(Debug, Deserialize)]
struct CreateMemoRequest {
    title: String,
    content: String,
}

// 애플리케이션 상태를 저장할 구조체
struct AppState {
    memos: Mutex<HashMap<u64, Memo>>,
    counter: Mutex<u64>,
}

// CREATE - 새 메모 생성
async fn create_memo(
    data: web::Data<AppState>,
    memo_req: web::Json<CreateMemoRequest>,
) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    let mut memos = data.memos.lock().unwrap();
    
    let memo = Memo {
        id: *counter,
        title: memo_req.title.clone(),
        content: memo_req.content.clone(),
        created_at: chrono::Utc::now(),
    };
    
    memos.insert(*counter, memo.clone());
    *counter += 1;
    
    HttpResponse::Ok().json(memo)
}

// READ - 모든 메모 조회
async fn get_memos(data: web::Data<AppState>) -> impl Responder {
    let memos = data.memos.lock().unwrap();
    let memo_list: Vec<&Memo> = memos.values().collect();
    HttpResponse::Ok().json(memo_list)
}

// READ - 특정 메모 조회
async fn get_memo(
    data: web::Data<AppState>,
    id: web::Path<u64>,
) -> impl Responder {
    let memos = data.memos.lock().unwrap();
    
    match memos.get(&id.into_inner()) {
        Some(memo) => HttpResponse::Ok().json(memo),
        None => HttpResponse::NotFound().body("메모를 찾을 수 없습니다"),
    }
}

// UPDATE - 메모 수정
async fn update_memo(
    data: web::Data<AppState>,
    id: web::Path<u64>,
    memo_req: web::Json<CreateMemoRequest>,
) -> impl Responder {
    let mut memos = data.memos.lock().unwrap();
    
    match memos.get_mut(&id.into_inner()) {
        Some(memo) => {
            memo.title = memo_req.title.clone();
            memo.content = memo_req.content.clone();
            HttpResponse::Ok().json(memo)
        }
        None => HttpResponse::NotFound().body("메모를 찾을 수 없습니다"),
    }
}

// DELETE - 메모 삭제
async fn delete_memo(
    data: web::Data<AppState>,
    id: web::Path<u64>,
) -> impl Responder {
    let mut memos = data.memos.lock().unwrap();
    
    match memos.remove(&id.into_inner()) {
        Some(_) => HttpResponse::Ok().body("메모가 삭제되었습니다"),
        None => HttpResponse::NotFound().body("메모를 찾을 수 없습니다"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 애플리케이션 상태 초기화
    let app_state = web::Data::new(AppState {
        memos: Mutex::new(HashMap::new()),
        counter: Mutex::new(0),
    });

    println!("서버가 http://localhost:8080 에서 실행 중입니다");

    // 서버 실행
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/memos", web::post().to(create_memo))
            .route("/memos", web::get().to(get_memos))
            .route("/memos/{id}", web::get().to(get_memo))
            .route("/memos/{id}", web::put().to(update_memo))
            .route("/memos/{id}", web::delete().to(delete_memo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/* 결과
Hello, world!
mod_string::function()
mod_number::add_number() : 5
*/