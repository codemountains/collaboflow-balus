use collaboflow_rs::request::document::documents_search::PostDocumentsSearchRequest;
use collaboflow_rs::{Authorization, CollaboflowClient, Query};
use dotenv::dotenv;
use serde_json::json;
use std::env;

const BASE_URL: &str = "BASE_URL";
const USER_ID: &str = "USER_ID";
const API_KEY: &str = "API_KEY";

const APP_CD: i32 = 1;
const LIMIT: i32 = 100;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("申請書の削除を開始します...");

    // .env から REST API 実行に必要な情報を取得する
    let secret = init();

    // API KEY 認証を利用する
    let authorization = Authorization::with_api_key(&secret.user_id, &secret.api_key);

    // クライアントを生成する
    let client = CollaboflowClient::new(&secret.base_url, authorization);

    // 削除した申請書をカウントする
    let mut deleted = 0;

    loop {
        // 検索条件を生成する
        let request =
            PostDocumentsSearchRequest::new(json!({"app_cd": APP_CD,"offset": 0, "limit": LIMIT}));
        let resp = client.documents_search.post(request).await?;
        let documents = resp.body.records;

        // 申請書の検索結果が 0 件の場合はループを抜ける
        if documents.is_empty() {
            break;
        }

        // 申請書を削除する
        for document in documents {
            // 削除 API では app_cd の指定が必要
            let query = Query::builder().app_cd(1);

            // 削除を実行する
            let _ = client.document.delete(document.document_id, query).await?;
            deleted += 1;

            // 待機する
            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;
        }
    }

    println!("{} 件の申請書を削除しました \\(°∀° )/", deleted);

    Ok(())
}

struct Secret {
    base_url: String,
    user_id: String,
    api_key: String,
}

fn init() -> Secret {
    dotenv().ok();

    let base_url = env::var(BASE_URL).unwrap_or_else(|_| panic!("{} is undefined.", BASE_URL));
    let user_id = env::var(USER_ID).unwrap_or_else(|_| panic!("{} is undefined.", USER_ID));
    let api_key = env::var(API_KEY).unwrap_or_else(|_| panic!("{} is undefined.", API_KEY));

    Secret {
        base_url,
        user_id,
        api_key,
    }
}
