use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lumen::layout::Viewport;
use lumen::search::SearchState;
use lumen::{layout_document, parse_markdown, Theme};

fn create_searchable_document(sections: usize) -> String {
    let mut doc = String::from("# Searchable Document\n\n");

    for i in 0..sections {
        doc.push_str(&format!("## Section {} about testing\n\n", i));
        doc.push_str("This paragraph contains the word test multiple times. ");
        doc.push_str("We test our code to ensure quality. Testing is important. ");
        doc.push_str("The test suite should be comprehensive.\n\n");

        doc.push_str("- test item one\n");
        doc.push_str("- another test item\n");
        doc.push_str("- final test entry\n\n");
    }

    doc
}

fn search_single_match(c: &mut Criterion) {
    let markdown = "# Title\n\nThis is a unique_word in the document.\n\n";
    let doc = parse_markdown(markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    c.bench_function("search_single_match", |b| {
        b.iter(|| {
            let mut state = SearchState::new();
            state.needle = black_box("unique_word").to_string();
            state.execute_search(&tree.root);
        })
    });
}

fn search_many_matches(c: &mut Criterion) {
    let markdown = create_searchable_document(50);
    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    c.bench_function("search_many_matches", |b| {
        b.iter(|| {
            let mut state = SearchState::new();
            state.needle = black_box("test").to_string();
            state.execute_search(&tree.root);
        })
    });
}

fn search_no_matches(c: &mut Criterion) {
    let markdown = create_searchable_document(50);
    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    c.bench_function("search_no_matches", |b| {
        b.iter(|| {
            let mut state = SearchState::new();
            state.needle = black_box("nonexistent_xyz").to_string();
            state.execute_search(&tree.root);
        })
    });
}

fn search_by_document_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_by_size");

    for sections in [10, 50, 100, 200].iter() {
        let markdown = create_searchable_document(*sections);
        let doc = parse_markdown(&markdown);
        let theme = Theme::builtin("docs").unwrap();
        let viewport = Viewport::new(80, 24);
        let tree = layout_document(&doc, &theme, viewport, false);

        group.bench_with_input(BenchmarkId::from_parameter(sections), sections, |b, _| {
            b.iter(|| {
                let mut state = SearchState::new();
                state.needle = black_box("test").to_string();
                state.execute_search(&tree.root);
            })
        });
    }

    group.finish();
}

fn search_navigation(c: &mut Criterion) {
    let markdown = create_searchable_document(50);
    let doc = parse_markdown(&markdown);
    let theme = Theme::builtin("docs").unwrap();
    let viewport = Viewport::new(80, 24);
    let tree = layout_document(&doc, &theme, viewport, false);

    let mut state = SearchState::new();
    state.needle = "test".to_string();
    state.execute_search(&tree.root);

    let mut group = c.benchmark_group("search_navigation");

    group.bench_function("next_match", |b| {
        b.iter(|| {
            let mut s = state.clone();
            s.next_match();
            black_box(s.current_match());
        })
    });

    group.bench_function("prev_match", |b| {
        b.iter(|| {
            let mut s = state.clone();
            s.prev_match();
            black_box(s.current_match());
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    search_single_match,
    search_many_matches,
    search_no_matches,
    search_by_document_size,
    search_navigation
);
criterion_main!(benches);
