# Mermaid Diagram Demo

Lumen now renders mermaid flowcharts as ASCII art natively.

## Simple Flow (Left to Right)

```mermaid
graph LR
    A[Input] --> B[Process] --> C[Output]
```

## Top-Down Flow

```mermaid
graph TD
    Start[Start] --> Validate[Validate Input]
    Validate --> Process[Process Data]
    Process --> Save[Save Results]
    Save --> Done[Done]
```

## Branching Decision

```mermaid
graph TD
    A[Request] --> B[Auth Check]
    B --> C[Authorized]
    B --> D[Denied]
```

## Pipeline

```mermaid
flowchart LR
    Src[Source] --> Parse[Parse] --> Transform[Transform] --> Render[Render] --> Out[Display]
```

## Non-mermaid code blocks still render normally

```python
def hello():
    print("This is regular code, not mermaid")
```

## Unsupported diagram types show raw code

```mermaid
sequenceDiagram
    Alice->>Bob: Hello
    Bob-->>Alice: Hi!
```
