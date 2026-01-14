use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use lumen::parse_markdown;

fn small_document() -> &'static str {
    "# Hello World\n\nThis is a **test** document with *formatting*."
}

fn medium_document() -> &'static str {
    r#"# Markdown Benchmark

This is a test document with various markdown features.

## Features

- Lists
- **Bold text**
- *Italic text*
- `inline code`

### Code Blocks

```rust
fn main() {
    println!("Hello, world!");
}
```

## Tables

| Feature | Status |
|---------|--------|
| Lists   | ✓      |
| Tables  | ✓      |
| Code    | ✓      |

> This is a blockquote with some content.

## Links

Check out [Rust](https://www.rust-lang.org/) for more info.

---

End of document.
"#
}

fn large_document() -> String {
    let mut doc = String::from("# Large Document\n\n");

    for i in 0..100 {
        doc.push_str(&format!("## Section {}\n\n", i));
        doc.push_str("This is a paragraph with **bold** and *italic* text. ");
        doc.push_str("It contains `code` and [links](https://example.com).\n\n");

        doc.push_str("- List item 1\n");
        doc.push_str("- List item 2\n");
        doc.push_str("- List item 3\n\n");

        if i % 10 == 0 {
            doc.push_str("```rust\n");
            doc.push_str("fn example() {\n");
            doc.push_str("    println!(\"test\");\n");
            doc.push_str("}\n");
            doc.push_str("```\n\n");
        }
    }

    doc
}

fn parse_small(c: &mut Criterion) {
    let markdown = small_document();
    c.bench_function("parse_small", |b| {
        b.iter(|| parse_markdown(black_box(markdown)))
    });
}

fn parse_medium(c: &mut Criterion) {
    let markdown = medium_document();
    c.bench_function("parse_medium", |b| {
        b.iter(|| parse_markdown(black_box(markdown)))
    });
}

fn parse_large(c: &mut Criterion) {
    let markdown = large_document();

    let mut group = c.benchmark_group("parse_large");
    group.throughput(Throughput::Bytes(markdown.len() as u64));
    group.bench_function("parse", |b| b.iter(|| parse_markdown(black_box(&markdown))));
    group.finish();
}

fn parse_by_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_by_size");

    for size in [100, 500, 1000, 5000, 10000].iter() {
        let markdown = "# Test\n\n".repeat(*size / 10);
        group.throughput(Throughput::Bytes(markdown.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &markdown, |b, md| {
            b.iter(|| parse_markdown(black_box(md)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    parse_small,
    parse_medium,
    parse_large,
    parse_by_size
);
criterion_main!(benches);
