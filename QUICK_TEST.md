# Quick Test

## Sidebar Mode Test

This tests that code blocks don't overflow into the sidebar when images are present.

![Test image in sidebar](test_images/nature.jpg)

Here's a code block that should stay within the content area:

```rust
fn main() {
    println!("This code block should not extend into the sidebar!");
    println!("All lines should be properly contained.");
    println!("Even really long lines that might otherwise extend beyond the normal width.");
}
```

More text after the code block.

## Inline Images Test

Run with `--inline-images` flag to test inline rendering:

```bash
./target/release/lumen QUICK_TEST.md --inline-images
```

When in inline mode, the image below should appear inline:

![Inline test image](test_images/tech.jpg)

This text should appear after the inline image, with proper spacing.

Another inline image:

![Second inline image](test_images/abstract.jpg)

End of test.
