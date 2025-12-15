# Image Rendering Test Document

This document tests inline image rendering in Lumen.

## Overview

Lumen now supports two image rendering modes:
- **Sidebar mode** (default): Images appear in a right sidebar
- **Inline mode** (`--inline-images` or `-i`): Images render inline at their position

## Test Images

### Nature Photography

Here's a beautiful nature scene captured in stunning detail:

![Nature landscape with mountains and water](test_images/nature.jpg)

The image above showcases natural beauty with vibrant colors and serene composition. Nature photography helps us appreciate the world around us.

### Technology and Innovation

![Modern technology and digital circuits](test_images/tech.jpg)

This image represents the cutting edge of technological advancement. From circuit boards to digital interfaces, technology shapes our modern world.

### Abstract Art

Here's an example of abstract artistic expression:

![Abstract geometric patterns and colors](test_images/abstract.jpg)

Abstract art challenges our perception and invites interpretation. Each viewer may see something different in these patterns and colors.

## Multiple Images in Sequence

Sometimes documents need to show multiple related images:

![First test image](test_images/nature.jpg)

![Second test image](test_images/tech.jpg)

![Third test image](test_images/abstract.jpg)

All three images should render with proper spacing between them.

## Images in Different Contexts

### Within a List

1. First item with an image:

   ![Inline in list item](test_images/nature.jpg)

   The image should render within the list context.

2. Second item without an image

3. Third item with another image:

   ![Another list image](test_images/tech.jpg)

### Within a Blockquote

> This is a blockquote that contains an image:
>
> ![Image in blockquote](test_images/abstract.jpg)
>
> The image should render properly within the quoted section.

## Testing Commands

To view this document in **sidebar mode** (default):
```bash
lumen TEST_IMAGES.md
```

To view this document in **inline mode**:
```bash
lumen TEST_IMAGES.md --inline-images
# or
lumen TEST_IMAGES.md -i
```

## Image Captions

Each image displays a caption below it in gray text showing the alt text. This helps identify images even when they can't be displayed.

---

**Note**: This test file uses relative paths to local images in the `test_images/` directory. Make sure you run Lumen from the project root directory.

## Expected Behavior

### Sidebar Mode
- Images appear in a right sidebar aligned with their text position
- Multiple images stack vertically in the sidebar
- Images may reposition to avoid overlap
- Document text uses full width minus sidebar

### Inline Mode
- Images render directly in the document flow
- Each image takes approximately 12 rows of terminal space
- Caption appears below each image
- Document content flows around images naturally

## Technical Details

The inline image feature uses:
- `ratatui_image` for terminal image rendering
- Automatic protocol detection (iTerm2, Kitty, Sixel)
- Fallback to alt text if images can't load
- 12-row height allocation per image

Happy testing! 🎨
