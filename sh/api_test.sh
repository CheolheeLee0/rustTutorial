#!/bin/bash

# 로그 파일 설정
LOG_FILE="api_test_$(date +%Y%m%d_%H%M%S).log"
echo "API 테스트 시작 - $(date)" > $LOG_FILE

# 로그 작성 함수
log_request() {
    echo -e "\n=== $1 ===" >> $LOG_FILE
    echo "요청 시간: $(date)" >> $LOG_FILE
    echo "요청 URL: $2" >> $LOG_FILE
    echo "요청 메소드: $3" >> $LOG_FILE
    if [ ! -z "$4" ]; then
        echo "요청 본문: $4" >> $LOG_FILE
    fi
}

# 1. CREATE - 새 메모 생성
echo "메모 생성 테스트..." | tee -a $LOG_FILE
create_memo() {
    local title=$1
    local content=$2
    local data="{\"title\": \"$title\", \"content\": \"$content\"}"
    
    log_request "메모 생성" "http://localhost:8080/memos" "POST" "$data"
    
    response=$(curl -s -X POST \
        -H "Content-Type: application/json" \
        -d "$data" \
        http://localhost:8080/memos)
    
    echo "응답: $response" >> $LOG_FILE
    echo $response | jq -r '.id'
}

# 2. READ - 모든 메모 조회
test_get_all() {
    log_request "전체 메모 조회" "http://localhost:8080/memos" "GET"
    
    response=$(curl -s -X GET http://localhost:8080/memos)
    echo "응답: $response" >> $LOG_FILE
}

# 3. READ - 특정 메모 조회
test_get_one() {
    local id=$1
    log_request "단일 메모 조회" "http://localhost:8080/memos/$id" "GET"
    
    response=$(curl -s -X GET http://localhost:8080/memos/$id)
    echo "응답: $response" >> $LOG_FILE
}

# 4. UPDATE - 메모 수정
test_update() {
    local id=$1
    local title=$2
    local content=$3
    local data="{\"title\": \"$title\", \"content\": \"$content\"}"
    
    log_request "메모 수정" "http://localhost:8080/memos/$id" "PUT" "$data"
    
    response=$(curl -s -X PUT \
        -H "Content-Type: application/json" \
        -d "$data" \
        http://localhost:8080/memos/$id)
    
    echo "응답: $response" >> $LOG_FILE
}

# 5. DELETE - 메모 삭제
test_delete() {
    local id=$1
    log_request "메모 삭제" "http://localhost:8080/memos/$id" "DELETE"
    
    response=$(curl -s -X DELETE http://localhost:8080/memos/$id)
    echo "응답: $response" >> $LOG_FILE
}

# 테스트 실행
echo "API 테스트를 시작합니다..." | tee -a $LOG_FILE

# 메모 생성 테스트
memo_id=$(create_memo "테스트 제목" "테스트 내용")
echo "생성된 메모 ID: $memo_id" | tee -a $LOG_FILE

# 전체 메모 조회 테스트
test_get_all

# 특정 메모 조회 테스트
test_get_one $memo_id

# 메모 수정 테스트
test_update $memo_id "수정된 제목" "수정된 내용"

# 수정된 메모 확인
test_get_one $memo_id

# 메모 삭제 테스트
test_delete $memo_id

# 삭제 확인을 위한 전체 메모 조회
test_get_all

echo -e "\n테스트 완료! 로그 파일: $LOG_FILE" | tee -a $LOG_FILE 