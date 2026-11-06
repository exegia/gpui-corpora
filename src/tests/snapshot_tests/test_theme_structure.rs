#[allow(unused_imports)]
use super::*;

#[test]
pub(crate) fn test_theme_structure() {
    let theme_path = theme_dir().join("gruvbox.json");
    let raw = std::fs::read_to_string(&theme_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", theme_path.display()));
    let value: serde_json::Value =
        serde_json::from_str(&raw).expect("theme file must be valid JSON");

    // Extract top-level keys and the name of each sub-theme for a stable snapshot.
    #[derive(serde::Serialize)]
    struct ThemeSummary {
        name: String,
        author: String,
        theme_count: usize,
        theme_names: Vec<String>,
        theme_modes: Vec<String>,
        color_key_count_per_theme: Vec<usize>,
        highlight_key_count_per_theme: Vec<usize>,
    }

    let themes = value.get("themes").and_then(|t| t.as_array()).unwrap();
    let summary = ThemeSummary {
        name: value
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap()
            .to_string(),
        author: value
            .get("author")
            .and_then(|v| v.as_str())
            .unwrap()
            .to_string(),
        theme_count: themes.len(),
        theme_names: themes
            .iter()
            .map(|t| t.get("name").and_then(|v| v.as_str()).unwrap().to_string())
            .collect(),
        theme_modes: themes
            .iter()
            .map(|t| t.get("mode").and_then(|v| v.as_str()).unwrap().to_string())
            .collect(),
        color_key_count_per_theme: themes
            .iter()
            .map(|t| {
                t.get("colors")
                    .and_then(|c| c.as_object())
                    .map(|o| o.len())
                    .unwrap_or(0)
            })
            .collect(),
        highlight_key_count_per_theme: themes
            .iter()
            .map(|t| {
                t.get("highlight")
                    .and_then(|h| h.as_object())
                    .map(|o| o.len())
                    .unwrap_or(0)
            })
            .collect(),
    };

    insta::assert_yaml_snapshot!(
            &summary,
    @r#"
        name: Gruvbox
        author: Pavel Pertsev
        theme_count: 2
        theme_names:
          - Gruvbox Light
          - Gruvbox Dark
        theme_modes:
          - light
          - dark
        color_key_count_per_theme:
          - 46
          - 42
        highlight_key_count_per_theme:
          - 21
          - 21
        "#
        );
}
