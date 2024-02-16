// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use serde_json::{from_str, to_string};
use serde::{Deserialize, Serialize};
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize)]
struct KvPair {
    k: String,
    v: String,
}

#[tauri::command]
fn list_vars() -> String {
    let mut vec = Vec::new();
    for (k, v) in env::vars() {
        vec.push(KvPair { k, v });
    }
    serde_json::to_string(&vec).expect("转JSON失败")
}

#[tauri::command]
fn list_path(name: &str) -> String {
    let vec = vec![
        KvPair {
            k: String::from("JAVA_HOME"),
            v: String::from("D://software//java"),
        },
        KvPair {
            k: String::from("GRADLE_HOME"),
            v: String::from("D://software//gradle"),
        },
    ];
    println!("name={}", name);
    serde_json::to_string(&vec).expect("转JSON失败")
    // println!("json_str={}", json_str);
    // for (k, v) in env::vars() {
    //     println!("{}={}", k, v);
    // }

    // String::from("hhh") + name
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_vars, list_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
