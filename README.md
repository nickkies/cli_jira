# Jira Clone

## Jira clone for the terminal

build two primary features in Jira:

1. Epic CRUD
2. Story CRUD

## Pages

Home

```
----------------------------- EPICS -----------------------------
     id     |               name               |      status
1           | Epic - Project 1                 | IN PROGRESS
4           | Epic - Project 2                 | OPEN


[q] quit | [c] create epic | [:id:] navigate to epic
```

Epic Detail

```
------------------------------ EPIC ------------------------------
  id  |     name     |         description         |    status
1     | Epic - Pr... | This is Project 1 for th... | IN PROGRESS

---------------------------- STORIES ----------------------------
     id     |               name               |      status
2           | Story - Project 1 Solution       | CLOSED
3           | Story - Project 1 README         | RESOLVED


[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story
```

Story Detail

```
------------------------------ STORY ------------------------------
  id  |     name     |         description         |    status
2     | Story - P... | Please provide full impl... | CLOSED


[p] previous | [u] update story | [d] delete story
```

## Objective

- Building CLI apps in Rust
- Reading & writing to disk
- Using third-party crates (like `serde`, `anyhow`, `itertools`, etc.)
- Writing testable code (`TDD`)
- Organizing code using modules
- Navigating and contributing to an existing code base
- Trunk-based development(그런데 이제 Issue-based를 곁들인🧐)
