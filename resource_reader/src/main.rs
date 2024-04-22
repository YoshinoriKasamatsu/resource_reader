use std::fs::read;
use std::path::Path;

use glob::glob;
use serde::{Serialize, Deserialize};

// コントロール
#[derive(Debug, Serialize, Deserialize)]
struct Control {
    id: String,
    class: String,
    style: String,
    text: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dialog {
    id: String,
    controls: Vec<Control>,
    header_files: Vec<String>,
}


// シリアル化用の構造体
#[derive(Debug, Serialize, Deserialize)]
struct ResourceFile {
    path: String,
    lines: Vec<String>,
    dialogs: Vec<Dialog>,

    
}


fn main() {
    
    // リソースファイル一覧
    let sample_rc_path = r"../example/app/**/*.rc";
    let rc_files = glob(sample_rc_path).unwrap();

    // リソースファイルの読み込み
    let resource_files = read_resource_files(rc_files);

    // ヘッダーファイルの読み込み
    let sample_rc_path = r"../example/app/**/*.h";
    let rc_files = glob(sample_rc_path).unwrap();

    // cppファイルの読み込み

    // コード解析
}

fn read_resource_files(rc_files: glob::Paths) -> Vec<ResourceFile> {
    let mut resource_files = Vec::new();
    
    // リソースファイルの列挙
    for rc_file in rc_files {
        // ファイルのパスを取得
        let rc_file = rc_file.ok();
        let rc_file_path = rc_file.as_ref().unwrap();
    
        // ファイルをUTF16で開く
        let path = Path::new(rc_file_path);
        let contents = read(path).expect("Failed to read file");
        let mut wchars = Vec::new();
        for i in 0..contents.len() / 2 {
            let c = u16::from_le_bytes([contents[i * 2], contents[i * 2 + 1]]);
            wchars.push(c);
        }
        let utf16_text = String::from_utf16(&wchars).unwrap();
        // \r\nを\nに変換、\rを\nに変換
        let rc_text = utf16_text.replace("\r\n", "\n").replace("\r", "\n");
        // \nで分割
        let rc_lines = rc_text.split("\n");
    
        let resource_file = ResourceFile{
            path: rc_file_path.to_str().unwrap().to_string(),
            lines: rc_lines.map(|s| s.to_string()).collect(),
            dialogs: Vec::new(),
            
        };
        resource_files.push(resource_file);
    }
    resource_files
    }
