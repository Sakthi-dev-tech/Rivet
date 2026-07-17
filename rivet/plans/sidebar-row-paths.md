# Sidebar Row Path Tracking Plan

## Context
- `App.collection_items` is a flattened list of only the currently visible `ApiCollectionItem`s, so a Ratatui row index cannot directly index the nested `App.collections` tree.
- `src/tui/main_logic.rs` already has an unfinished `SidebarRow { path: Vec<usize> }` and `App.sidebar_rows`; the vector is not populated or initialized yet.
- The intended mapping can represent a visible row with a root-to-item index path: e.g. `[2]` is `collections[2]`, while `[2, 0, 3]` is the fourth child under the first child folder of `collections[2]`.

## Approach
- Change the flattening helper to return both rendered items and row metadata from the same recursive traversal, keeping `collection_items[visible_index]` and `sidebar_rows[visible_index]` aligned by construction.
- Store one root-to-item `Vec<usize>` path for every visible folder or request row. For example, `[2, 0, 3]` means `collections[2] -> children[0] -> children[3]`.
- Include a collapsed folder's own path but omit its hidden descendants, exactly matching the rendered list.
- Rebuild and assign both vectors together in `App::refresh_collection_items`. This change will maintain the mapping only; consuming it for Enter behavior is intentionally deferred.

## Files to modify
- `src/tui/main_logic.rs`

## Reuse
- Extend the existing recursive `collection_items` traversal in `src/tui/main_logic.rs` rather than adding a second tree walk.
- Reuse the in-progress `SidebarRow`, `App.sidebar_rows`, `App.collections`, and `App.sidebar_state` definitions already present in the working tree.
- Keep `src/tui/sidebar_ui.rs` presentation-only; future key handling in `main_logic.rs` can look up `sidebar_rows[sidebar_state.selected()]` without teaching the renderer about the collection model.

## Steps
- [x] Document `SidebarRow.path` as indices from the root collection slice through each folder's `children` slice; fix the struct formatting while touching it.
- [x] Refactor `collection_items` to accept the current parent path and return a pair of vectors: rendered `ListItem`s and matching `SidebarRow`s.
- [x] For each enumerated item, append its local index to the parent path before emitting metadata; pass that path into expanded-folder recursion and extend both result vectors in the same order.
- [x] Update `refresh_collection_items` to destructure and assign both generated vectors together, assert their lengths remain equal in debug builds, and preserve the existing empty-list/selection-clamping behavior using the shared row count.
- [x] Initialize `sidebar_rows` to an empty vector in `tui_app` before the initial refresh.
- [x] Add focused unit tests in `main_logic.rs` that assert the exact visible-index-to-path sequence for top-level items, expanded nested folders, multiple siblings, and collapsed folders whose descendants must not receive rows.

## Verification
- [x] Run `cargo fmt --check` (the modified file passes `rustfmt --check`; the repository-wide command still reports pre-existing formatting differences in unrelated files).
- [x] Run `cargo test`.
- [x] Run `cargo check` and confirm the currently incomplete `sidebar_rows` initialization is resolved.
- [x] In unit tests, verify every generated visible row has exactly one metadata entry at the same index and that nested paths retain original child indices.
- [x] Launch the TUI to confirm rendering and `j`/`k` navigation are unchanged; Enter behavior is outside this change.
