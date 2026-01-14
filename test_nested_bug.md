# Nested List Bug Test

## Test Case 1: Parent text with nested list

- Removed: Non-existent commands
  - models search (doesn't exist)
  - models status (doesn't exist)

## Test Case 2: Multiple nested items

- Parent item one
  - Child 1.1
  - Child 1.2
- Parent item two
  - Child 2.1
  - Child 2.2

## Test Case 3: Empty parent (edge case)

-
  - Child item
  - Another child

## Test Case 4: Ordered list with nesting

1. First item
   1. Nested 1.1
   2. Nested 1.2
2. Second item
   1. Nested 2.1

## Test Case 5: Deep nesting

- Level 1
  - Level 2
    - Level 3
      - Level 4
