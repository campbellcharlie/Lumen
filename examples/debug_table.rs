use lumen::parse_markdown;

fn main() {
    let markdown = "| A | B |\n|---|---|\n| 1 | 2 |";
    let doc = parse_markdown(markdown);

    println!("Document has {} blocks", doc.blocks.len());
    for (i, block) in doc.blocks.iter().enumerate() {
        println!("Block {}: {:#?}", i, block);
    }
}
