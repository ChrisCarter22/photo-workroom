CREATE TABLE IF NOT EXISTS keyword_lists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);

CREATE TABLE IF NOT EXISTS keyword_terms (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    keyword_list_id INTEGER NOT NULL,
    term TEXT NOT NULL,
    parent_term_id INTEGER,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE(keyword_list_id, term),
    FOREIGN KEY(keyword_list_id) REFERENCES keyword_lists(id) ON DELETE CASCADE,
    FOREIGN KEY(parent_term_id) REFERENCES keyword_terms(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS preview_cache_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    asset_id INTEGER NOT NULL,
    cache_path TEXT NOT NULL UNIQUE,
    width INTEGER NOT NULL CHECK (width > 0),
    height INTEGER NOT NULL CHECK (height > 0),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    last_accessed_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    FOREIGN KEY(asset_id) REFERENCES assets(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_keyword_terms_keyword_list_id ON keyword_terms(keyword_list_id);
CREATE INDEX IF NOT EXISTS idx_preview_cache_entries_asset_id ON preview_cache_entries(asset_id);
