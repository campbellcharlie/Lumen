use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lumen::layout::Viewport;
use lumen::{layout_document, parse_markdown, Theme};

fn create_test_document(sections: usize) -> String {
    let mut doc = String::from("# Test Document\n\n");

    for i in 0..sections {
        doc.push_str(&format!("## Section {}\n\n", i));
        doc.push_str("This is a paragraph with **bold** and *italic* text. ");
        doc.push_str("It also has `inline code` and [links](https://example.com).\n\n");

        doc.push_str("- Item 1\n");
        doc.push_str("- Item 2\n");
        doc.push_str("- Item 3\n\n");
    }

    doc
}

fn layout_small(c: &mut Criterion) {
    let markdown = create_test_document(5);
    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    c.bench_function("layout_small", |b| {
        b.iter(|| layout_document(black_box(&doc), black_box(&theme), viewport, false))
    });
}

fn layout_medium(c: &mut Criterion) {
    let markdown = create_test_document(50);
    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    c.bench_function("layout_medium", |b| {
        b.iter(|| layout_document(black_box(&doc), black_box(&theme), viewport, false))
    });
}

fn layout_large(c: &mut Criterion) {
    let markdown = create_test_document(200);
    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    c.bench_function("layout_large", |b| {
        b.iter(|| layout_document(black_box(&doc), black_box(&theme), viewport, false))
    });
}

fn layout_different_widths(c: &mut Criterion) {
    let markdown = create_test_document(50);
    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();

    let mut group = c.benchmark_group("layout_by_width");

    for width in [40, 60, 80, 100, 120].iter() {
        let viewport = Viewport::new(*width, 24);
        group.bench_with_input(BenchmarkId::from_parameter(width), width, |b, _| {
            b.iter(|| layout_document(black_box(&doc), black_box(&theme), viewport, false))
        });
    }

    group.finish();
}

fn layout_with_inline_images(c: &mut Criterion) {
    let mut markdown = String::from("# Image Test\n\n");
    for i in 0..20 {
        markdown.push_str(&format!("![Image {}](test{}.png)\n\n", i, i));
        markdown.push_str("Some text between images.\n\n");
    }

    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);

    let mut group = c.benchmark_group("layout_images");

    group.bench_function("sidebar_mode", |b| {
        b.iter(|| layout_document(black_box(&doc), black_box(&theme), viewport, false))
    });

    group.bench_function("inline_mode", |b| {
        b.iter(|| layout_document(black_box(&doc), black_box(&theme), viewport, true))
    });

    group.finish();
}

criterion_group!(
    benches,
    layout_small,
    layout_medium,
    layout_large,
    layout_different_widths,
    layout_with_inline_images
);
criterion_main!(benches);
