mod parse;

#[tokio::main]
async fn main() {
    for entry in walkdir::WalkDir::new("samples")
        .into_iter()
        .filter_map(|e| e.ok() )
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".log"))
    {
        let filepath = format!("{}", entry.path().display());
        parse::parse_text(&tokio::fs::read_to_string(filepath).await.unwrap());
        break
    }
    // let text = tokio::fs::read_to_string("samples/EXCEL/App1679038290952831100_0A1CF4DA-6DA3-472D-9E98-6B7434440AFC.log").await;
    // if let
}
