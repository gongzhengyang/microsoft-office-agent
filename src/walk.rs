use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

use tokio::sync::OnceCell;

use super::parse;

static HAD_READ_LOGS: OnceCell<Mutex<HashSet<String>>> = OnceCell::const_new();
const HAD_READ_FILEPATH: &str = "read-filepath";

async fn get_had_read_log_filepath_handle() -> &'static Mutex<HashSet<String>> {
    HAD_READ_LOGS
        .get_or_init(|| async {
            let mut values = HashSet::new();
            let text = std::fs::read_to_string(HAD_READ_FILEPATH).unwrap_or("".to_owned());
            let lines = text.lines().map(|x| x.to_owned());
            values.extend(lines);
            Mutex::new(values)
        })
        .await
}

async fn is_filepath_has_read(filepath: String) -> bool {
    let handle = get_had_read_log_filepath_handle().await.lock().unwrap();
    handle.contains(&filepath)
}

async fn append_has_read_log(filepath: &str) {
    let mut file = std::fs::File::options()
        .create(true)
        .write(true)
        .append(true)
        .open(HAD_READ_FILEPATH)
        .unwrap();
    file.write_all(format!("{filepath}\n").as_bytes()).unwrap();
    let mut handle = get_had_read_log_filepath_handle().await.lock().unwrap();
    handle.insert(filepath.to_owned());
}

pub async fn walk_for_logs(dir: PathBuf) -> Vec<HashMap<String, String>> {
    let mut full_log_results = vec![];

    for entry in walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".log"))
    {
        let filepath = format!("{}", entry.path().display());
        if is_filepath_has_read(filepath.clone()).await {
            continue;
        }
        tracing::info!("read {filepath}");
        let text = tokio::fs::read_to_string(&filepath)
            .await
            .unwrap_or("".to_owned());
        let results = parse::parse_text(&text);
        if !results.is_empty() {
            append_has_read_log(&filepath).await;
            full_log_results.extend(results);
        }
    }
    full_log_results
}
