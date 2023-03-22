// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use num_cpus;
use porker::{Card, ResultData};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;

mod porker;

#[derive(Deserialize)]
pub struct Request {
    num_of_atempt: u32,
    use_cards: Vec<u32>,
}

/// それぞれの役が何回出たか保持する構造体です．
/// Response構造体の一部分でもあります．
#[derive(Serialize)]
#[allow(non_snake_case)]
struct ResultRole {
    nopair: u32,
    onepair: u32,
    twopair: u32,
    threepair: u32,
    fourpair: u32,
    fulhouse: u32,
    flush: u32,
    strait: u32,
    straitflush: u32,
    royalflush: u32,
}

/// 実行結果を保存する構造体です．
/// 総スコア，回数，それぞれの役の出現回数
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Response {
    score: u32,
    total_num_of_atempt: u32,
    time_ms: u128,
    result: ResultRole,
}

///必要なデータを渡すと，レスポンスを生成します．
// 処理時間(ミリ秒)を追加
impl Response {
    fn new(score: u32, total_num_of_atempt: u32, time_ms: u128, role_count: [u32; 10]) -> Response {
        Response {
            score,
            total_num_of_atempt,
            time_ms,
            result: ResultRole {
                nopair: role_count[0],
                onepair: role_count[1],
                twopair: role_count[2],
                threepair: role_count[3],
                fourpair: role_count[4],
                fulhouse: role_count[5],
                flush: role_count[6],
                strait: role_count[7],
                straitflush: role_count[8],
                royalflush: role_count[9],
            },
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn million_porker(request: Request) -> Response {
    //use_cards: Vec<u32>, num_of_atempt: u32
    //let use_cards = Card::all_cards_id();
    println!("use_cards: {:?}", request.use_cards);
    let instant = Instant::now();

    let use_cards = request.use_cards;
    let num_of_atempt = request.num_of_atempt;

    let ResultData {
        role_count,
        score,
        total_num_of_atempt,
    } = porker::million_porker_parallel(Arc::new(use_cards), num_of_atempt).unwrap();

    let time_ms = instant.elapsed().as_millis();
    println!("time: {:?}", instant.elapsed());

    porker::debug_judge_role(&role_count, total_num_of_atempt);
    Response::new(score, total_num_of_atempt, time_ms, role_count)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, million_porker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
