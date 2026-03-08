# UX_WORKFLOWS.md

This document describes the target user workflows for Photo Workroom.

Current repository status on March 8, 2026:

- a shell baseline now exists with a dark workspace-first layout, sidebar panels, tab strip, status bar, and a typed backend health-check action
- separate-window folder opening, thumbnail browsing, preview, metadata editing, and ingest workflows remain planned
- the workflows below describe the product behavior the UI should eventually support

## Workflow principles

- optimize for keyboard-driven speed
- show progress and state truthfully
- keep destructive actions explicit
- support bulk operations without ambiguity
- expose metadata source and limitations when they matter
- prefer predictable workflows over hidden behavior

## 0. Application shell

Target opening layout:

1. app opens into a dark, workspace-first shell rather than a marketing-style home screen
2. left sidebar exposes search plus stacked Favorites, Navigator, and Tasks panels
3. top bar exposes tabs for open contact sheets or other working contexts
4. center workspace opens as a blank `Untitled` sheet or last active contact sheet, depending on preferences
5. bottom status bar shows selection counts, lightweight task state, and storage or file-count context where useful

Layout rules:

- side panels should be resizable and persist their last useful state
- the shell should feel immediately usable for import and browsing, even before any images are open
- if the current tab is a blank `Untitled` workspace, opening a folder should convert that tab into a named contact sheet for the opened folder
- if a real contact sheet is already active, opening another folder should create a new tab by default
- opening a folder in a separate window should remain available as an explicit action or preference, not hidden behavior
- we are matching this workflow shape intentionally, not copying the exact reference UI chrome pixel for pixel

## 1. Card or folder ingest

Target flow:

1. user opens the import workflow
2. user selects a source such as folder, card, or camera
3. app shows a preview of candidate files and import options
4. user reviews primary destination, optional secondary destination, rename template, duplicate policy, and metadata preset
5. user confirms import
6. app shows progress and per-file issues in a dedicated task surface
7. app can open the destination contact sheet as soon as files begin arriving
8. app lands on the imported results or import summary

Variants this workflow must support:

- ingest from current selection only
- ingest from multiple disks or folders in a single session
- incremental ingest behavior to avoid recopying previously ingested files
- auto ingest on card mount if enabled
- live ingest from a watched folder when that workflow is active

## 2. Contact sheet browsing

Target flow:

1. user opens a folder from the blank shell or from Navigator
2. if the active tab is still `Untitled`, that tab becomes a named contact sheet for the folder
3. app shows a virtualized grid of thumbnails in a tabbed contact-sheet workspace
4. app exposes Navigator, Favorites, and Tasks side panels
5. user scrolls, sorts, filters, resizes thumbnails, and changes thumbnail labels without blocking
6. app loads thumbnails lazily and degrades gracefully when previews are missing
7. user can open additional folders in new tabs, reopen saved worksets, or explicitly open a folder in a separate window
8. user can drag items to move or copy them when that workflow is allowed

## 3. Selection and culling

Target flow:

1. user selects one or more assets
2. user assigns tags, star ratings, color classes, or rejects through keyboard or toolbar actions
3. app updates state immediately and persists the change
4. user filters or selects by tagged, untagged, rated, colored, rotated, or cropped state
5. app keeps selection movement fast enough for high-volume culling

## 4. Preview and compare

Target flow:

1. user opens a larger preview for the active asset
2. app loads the preview quickly or shows a clear loading state
3. user switches between one-up and two-up comparison modes
4. user can link or unlink comparison panes, zoom, pan, inspect highlights or shadows, and rate or tag directly in preview
5. app supports fullscreen review without losing the current selection context
6. user can launch a slide show for selection review when that workflow is preferred
7. when watching a live folder, the app can run a live slide show style review of arriving images

## 5. Crop and transforms

Target flow:

1. user activates crop from preview
2. app shows a non-destructive crop overlay
3. user resizes, moves, rotates, or nudges the crop
4. user previews the crop result without altering the source file destructively
5. user can copy or paste crop settings between related assets when appropriate

## 6. Metadata editing

Target flow:

1. user opens either the batch metadata template flow or the single-image metadata info flow
2. app shows normalized fields and warns about mixed values when relevant
3. user edits caption, tags, location, time, rights, or other supported fields
4. user can move next or previous quickly when captioning one image at a time
5. app writes changes to the DB and file or sidecar according to policy
6. app shows success, warning, or partial failure truthfully

## 7. Variables and code replacements

Target flow:

1. user edits a metadata template, rename mask, watermark, or output setting
2. user inserts variables from a known list
3. user optionally uses code replacements for shorthand entry
4. app previews expanded values before applying them broadly
5. expansion errors are explicit and actionable

## 8. Keyword management

Target flow:

1. user edits flat keywords for a quick workflow or opens the keyword panel
2. user can apply terms from a controlled vocabulary
3. user can browse or assign structured keyword paths where the hierarchy matters
4. app keeps the vocabulary and file metadata synchronized

## 9. Batch rename

Target flow:

1. user selects assets and opens the rename dialog
2. user enters a template
3. app previews the resulting filenames before commit
4. app highlights collisions or invalid output
5. user confirms rename
6. app updates the filesystem and DB consistently

## 10. Search, filter, sort, refresh, and find

Target flow:

1. user enters free-text search, opens advanced filters, or runs a Find command in the active contact sheet
2. app updates results quickly and can optionally switch the sheet to the matching selection
3. user combines filters such as date, rating, tag, color class, camera, rotated state, or cropped state
4. user can refresh metadata display or rescan folders for newly added files
5. sort changes preserve the current workflow context without confusing jumps
6. if manual arrangement is enabled, the user can drag thumbnails into a custom order

## 11. Export, Save As, and delivery

Target flow:

1. user selects assets and opens the export dialog
2. user chooses Save As or another delivery mode such as uploader, print, email, or gallery output
3. user chooses format, destination, naming rules, resize options, and optional watermark settings
4. app previews key settings and starts the job
5. app shows progress and per-file failures honestly
6. app produces a final summary with output location or delivery status

## 12. GPS and capture-time correction

Target flow:

1. user imports GPS logs or opens a location workflow for already geotagged assets
2. app matches or displays coordinates safely
3. app can reverse geocode coordinates into location fields when the workflow enables it
4. user adjusts capture times when multiple cameras need synchronization
5. downstream sorting, rename, and location workflows reflect the updated values

## 13. External edit handoff

Target flow:

1. user chooses to open a file in an external editor
2. app launches the configured editor or OS default
3. app detects updated files after the external edit completes
4. app rescans or refreshes metadata safely
5. paired RAW and JPEG launch behavior follows explicit preferences

## 14. Preferences, snapshots, and shortcuts

Target flow:

1. user customizes color-class names, keyboard shortcuts, dialog layouts, metadata compatibility settings, and workflow preferences
2. user saves or restores a preference snapshot when supported
3. app keeps customizations portable where practical and documents machine-specific limits clearly

## 15. Help, about, and trust signals

Target flow:

1. user can see that the app is local-first
2. user can reach offline documentation and workflow help
3. user can find build version, known limitations, and troubleshooting information
4. user can understand which Photo Mechanic-style workflows are already implemented and which are still planned

## Cross-cutting UX rules

Required behavior:

- destructive actions require clear confirmation when data loss is possible
- long-running work must show progress and cancellation or retry behavior where appropriate
- empty states and errors must explain what the user can do next
- keyboard shortcuts should accelerate expert workflows without hiding core controls from new users
- preference customization must not make workflows ambiguous or untestable
- background helpers for ingest, rename, metadata, preview generation, and delivery must not block contact-sheet scrolling or active preview navigation
- all workflow changes that affect behavior should be covered by tests or explicit acceptance criteria
