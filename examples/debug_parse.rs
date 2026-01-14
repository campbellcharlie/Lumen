//! Debug parser output for nested lists

use lumen::parse_markdown;

fn main() {
    let markdown = r#"### 3.2 Training Objective
- **Yours:** Cosine/MSE between `F(h_A)` and `c_B`.
- **Research:**
  - Task-aware MI objectives (InfoPrompt).
  - Distance & smoothness constraints (SpaceFusion).
  - Token-level cross-entropy on decoded hidden messages (translator models).
"#;

    let doc = parse_markdown(markdown);

    println!("Parsed document:");
    println!("{:#?}", doc);
}
