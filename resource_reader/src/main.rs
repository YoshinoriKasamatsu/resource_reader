use std::{clone, fs::read};
use std::path::{Path, PathBuf};

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
    resource_blocks: Vec<ResourceBlock>,
    dialogs: Vec<Dialog>,
}

// リソースブロック
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ResourceBlock {
    resource_type: ResourceType,
    lines: Vec<String>,
}

// リソースタイプの列挙型
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum ResourceType {
    DIALOG,
    MENU,
    STRING,
    ACCELERATORS,
    CURSOR,
    ICON,
    BITMAP,
    HTML,
    MANIFEST,
    VERSION,
    UNKNOWN,
}


fn main() {
    
    // リソースファイル一覧
    let sample_rc_path = r"../example/app/**/*.rc";
    let rc_files = glob(sample_rc_path).unwrap();

    // リソースファイルの読み込み
    let mut resource_files = read_resource_files(rc_files);

    for resource_file in &resource_files {
        for resource_block in &resource_file.resource_blocks {
            println!("{:?}", resource_block);
        }
    }


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

        // ファイル読み込み

        // ファイルのパスを取得
        let rc_file: Option<std::path::PathBuf> = rc_file.ok();
        let rc_file_path = rc_file.as_ref().unwrap();
        let utf16_text = read_utf16_file(rc_file_path);
        // \r\nを\nに変換、\rを\nに変換
        let rc_text = utf16_text.replace("\r\n", "\n").replace("\r", "\n");
        // \nで分割
        let rc_lines = rc_text.split("\n");

        // リソースブロック作成

        // let mut dialogs = Vec::new();
        // let mut string_tables = Vec::new();
        let mut resource_blocks = Vec::new();
        let mut resource_type = ResourceType::UNKNOWN;
        let mut resource_block: ResourceBlock = ResourceBlock{
            resource_type: ResourceType::UNKNOWN,
            lines: Vec::new(),
        };

        let lines = rc_lines.clone();

        for line in rc_lines {
            // DIALOGの検出
            if line.contains(" DIALOG") {
                resource_type = ResourceType::DIALOG;
                resource_block.resource_type = ResourceType::DIALOG;
            }
            // STRINGTABLEの検出
            if line.contains("STRINGTABLE") {
                resource_type = ResourceType::STRING;
            }

            // リソースブロックの追加
            if resource_type != ResourceType::UNKNOWN {
                resource_block.lines.push(String::from(line.clone()));
            }

            // END検出でリソースタイプをUNKNOWNに戻す
            if line.contains("END") {
                resource_blocks.push(resource_block.clone());
                resource_type = ResourceType::UNKNOWN;
            }
            println!("{}", line);
        }


    
        let resource_file = ResourceFile{
            path: rc_file_path.to_str().unwrap().to_string(),
            lines: lines.map(|s| s.to_string()).collect(),
            resource_blocks: resource_blocks,
            dialogs: Vec::new(),
        };
        resource_files.push(resource_file);
    }
    resource_files
    }

fn read_utf16_file(rc_file_path: &PathBuf) -> String {
    // ファイルをUTF16で開く
    let path = Path::new(rc_file_path);
    let contents = read(path).expect("Failed to read file");
    let mut wchars = Vec::new();
    for i in 0..contents.len() / 2 {
        let c = u16::from_le_bytes([contents[i * 2], contents[i * 2 + 1]]);
        wchars.push(c);
    }
    let utf16_text = String::from_utf16(&wchars).unwrap();
    utf16_text
}
