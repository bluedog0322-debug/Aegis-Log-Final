fn main() {
    // アイコンの自動検索を完全にスキップする設定です
    let attrs = tauri_build::Attributes::new()
        .windows_attributes(tauri_build::WindowsAttributes::new());
    
    // 標準のビルドではなく、属性を指定したビルドを行います
    tauri_build::build_with_attributes(attrs);
}
