use std::{
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
};

use photo_workroom_core::SubsystemSnapshot;
use rusqlite::{params, Connection, OptionalExtension};

pub const DATABASE_DIRECTORY_NAME: &str = "catalog";
pub const DATABASE_FILE_NAME: &str = "photo-workroom.sqlite3";
pub const LATEST_SCHEMA_VERSION: u32 = 2;

struct Migration {
    version: u32,
    sql: &'static str,
}

const MIGRATIONS: [Migration; 2] = [
    Migration {
        version: 1,
        sql: include_str!("../migrations/0001_initial.sql"),
    },
    Migration {
        version: 2,
        sql: include_str!("../migrations/0002_keyword_and_preview.sql"),
    },
];

#[derive(Debug)]
pub enum DbError {
    Io(std::io::Error),
    Sql(rusqlite::Error),
    Validation(&'static str),
    Conversion(&'static str),
    Invariant(&'static str),
}

impl Display for DbError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Sql(error) => write!(formatter, "SQLite error: {error}"),
            Self::Validation(message) => write!(formatter, "validation error: {message}"),
            Self::Conversion(message) => write!(formatter, "conversion error: {message}"),
            Self::Invariant(message) => write!(formatter, "invariant error: {message}"),
        }
    }
}

impl std::error::Error for DbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Sql(error) => Some(error),
            Self::Validation(_) | Self::Conversion(_) | Self::Invariant(_) => None,
        }
    }
}

impl From<std::io::Error> for DbError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sql(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewAssetRecord {
    pub canonical_path: String,
    pub file_size_bytes: u64,
}

impl NewAssetRecord {
    pub fn new(canonical_path: impl Into<String>, file_size_bytes: u64) -> Self {
        Self {
            canonical_path: canonical_path.into(),
            file_size_bytes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetRecord {
    pub id: i64,
    pub canonical_path: String,
    pub file_size_bytes: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEventRecord {
    pub id: i64,
    pub event_type: String,
    pub details: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct CatalogDatabase {
    connection: Connection,
    database_path: PathBuf,
}

impl CatalogDatabase {
    pub fn open_at_path(database_path: &Path) -> Result<Self, DbError> {
        if let Some(parent) = database_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut connection = Connection::open(database_path)?;
        configure_connection(&connection)?;
        apply_migrations(&mut connection)?;

        Ok(Self {
            connection,
            database_path: database_path.to_path_buf(),
        })
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }

    pub fn schema_version(&self) -> Result<u32, DbError> {
        current_schema_version(&self.connection)
    }

    pub fn upsert_asset(&self, asset: NewAssetRecord) -> Result<AssetRecord, DbError> {
        let canonical_path = normalize_path(&asset.canonical_path)?;
        let file_size_bytes = i64::try_from(asset.file_size_bytes)
            .map_err(|_| DbError::Conversion("file_size_bytes exceeds i64 range"))?;

        self.connection.execute(
            "INSERT INTO assets (canonical_path, file_size_bytes)
             VALUES (?1, ?2)
             ON CONFLICT(canonical_path) DO UPDATE
             SET file_size_bytes = excluded.file_size_bytes,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')",
            params![canonical_path, file_size_bytes],
        )?;

        self.get_asset_by_path(canonical_path)?
            .ok_or(DbError::Invariant("upserted asset was not readable"))
    }

    pub fn get_asset_by_path(&self, canonical_path: &str) -> Result<Option<AssetRecord>, DbError> {
        let canonical_path = normalize_path(canonical_path)?;
        let result = self
            .connection
            .query_row(
                "SELECT id, canonical_path, file_size_bytes
                 FROM assets
                 WHERE canonical_path = ?1",
                params![canonical_path],
                |row| {
                    Ok(AssetRecord {
                        id: row.get(0)?,
                        canonical_path: row.get(1)?,
                        file_size_bytes: row.get(2)?,
                    })
                },
            )
            .optional()?;

        Ok(result)
    }

    pub fn delete_asset_by_path(&self, canonical_path: &str) -> Result<bool, DbError> {
        let canonical_path = normalize_path(canonical_path)?;
        let deleted = self.connection.execute(
            "DELETE FROM assets WHERE canonical_path = ?1",
            params![canonical_path],
        )?;

        Ok(deleted > 0)
    }

    pub fn asset_count(&self) -> Result<u64, DbError> {
        let count: i64 = self
            .connection
            .query_row("SELECT COUNT(*) FROM assets", [], |row| row.get(0))?;

        u64::try_from(count).map_err(|_| DbError::Conversion("asset count was negative"))
    }

    pub fn append_audit_event(
        &self,
        event_type: &str,
        details: &str,
    ) -> Result<AuditEventRecord, DbError> {
        let event_type = normalize_event_type(event_type)?;
        let details = normalize_event_details(details)?;

        self.connection.execute(
            "INSERT INTO audit_log (event_type, details)
             VALUES (?1, ?2)",
            params![event_type, details],
        )?;

        let event_id = self.connection.last_insert_rowid();
        self.get_audit_event_by_id(event_id)?
            .ok_or(DbError::Invariant("inserted audit event was not readable"))
    }

    pub fn audit_event_count(&self) -> Result<u64, DbError> {
        let count: i64 =
            self.connection
                .query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))?;

        u64::try_from(count).map_err(|_| DbError::Conversion("audit event count was negative"))
    }

    pub fn latest_audit_event(&self) -> Result<Option<AuditEventRecord>, DbError> {
        let event = self
            .connection
            .query_row(
                "SELECT id, event_type, details, created_at
                 FROM audit_log
                 ORDER BY id DESC
                 LIMIT 1",
                [],
                |row| {
                    Ok(AuditEventRecord {
                        id: row.get(0)?,
                        event_type: row.get(1)?,
                        details: row.get(2)?,
                        created_at: row.get(3)?,
                    })
                },
            )
            .optional()?;

        Ok(event)
    }

    fn get_audit_event_by_id(&self, event_id: i64) -> Result<Option<AuditEventRecord>, DbError> {
        let event = self
            .connection
            .query_row(
                "SELECT id, event_type, details, created_at
                 FROM audit_log
                 WHERE id = ?1",
                params![event_id],
                |row| {
                    Ok(AuditEventRecord {
                        id: row.get(0)?,
                        event_type: row.get(1)?,
                        details: row.get(2)?,
                        created_at: row.get(3)?,
                    })
                },
            )
            .optional()?;

        Ok(event)
    }

    #[cfg(test)]
    fn table_exists(&self, table_name: &str) -> Result<bool, DbError> {
        table_exists(&self.connection, table_name)
    }

    #[cfg(test)]
    fn journal_mode(&self) -> Result<String, DbError> {
        let mode: String = self
            .connection
            .query_row("PRAGMA journal_mode;", [], |row| row.get(0))?;
        Ok(mode)
    }
}

pub fn catalog_database_path(app_data_dir: &Path) -> PathBuf {
    app_data_dir
        .join(DATABASE_DIRECTORY_NAME)
        .join(DATABASE_FILE_NAME)
}

pub fn open_catalog_database(app_data_dir: &Path) -> Result<CatalogDatabase, DbError> {
    let database_path = catalog_database_path(app_data_dir);
    CatalogDatabase::open_at_path(&database_path)
}

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "db",
        "Phase 3",
        "SQLite migrations, WAL defaults, and a typed repository baseline are implemented in the db crate.",
    )
}

fn normalize_path(path: &str) -> Result<&str, DbError> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(DbError::Validation("canonical_path must not be empty"));
    }

    Ok(trimmed)
}

fn normalize_event_type(event_type: &str) -> Result<&str, DbError> {
    let trimmed = event_type.trim();
    if trimmed.is_empty() {
        return Err(DbError::Validation("event_type must not be empty"));
    }

    Ok(trimmed)
}

fn normalize_event_details(details: &str) -> Result<&str, DbError> {
    let trimmed = details.trim();
    if trimmed.is_empty() {
        return Err(DbError::Validation("event details must not be empty"));
    }

    Ok(trimmed)
}

fn configure_connection(connection: &Connection) -> Result<(), DbError> {
    connection.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        ",
    )?;

    Ok(())
}

fn current_schema_version(connection: &Connection) -> Result<u32, DbError> {
    let version: u32 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    Ok(version)
}

#[cfg(test)]
fn table_exists(connection: &Connection, table_name: &str) -> Result<bool, DbError> {
    let exists: i64 = connection.query_row(
        "SELECT EXISTS(
            SELECT 1
            FROM sqlite_master
            WHERE type = 'table' AND name = ?1
        )",
        params![table_name],
        |row| row.get(0),
    )?;

    Ok(exists == 1)
}

fn apply_migrations(connection: &mut Connection) -> Result<(), DbError> {
    let mut current_version = current_schema_version(connection)?;

    for migration in MIGRATIONS.iter() {
        if migration.version <= current_version {
            continue;
        }

        apply_migration(connection, migration)?;
        current_version = migration.version;
    }

    Ok(())
}

fn apply_migration(connection: &mut Connection, migration: &Migration) -> Result<(), DbError> {
    let transaction = connection.transaction()?;
    transaction.execute_batch(migration.sql)?;
    transaction.pragma_update(None, "user_version", migration.version)?;
    transaction.commit()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        path::{Path, PathBuf},
        process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;
    use rusqlite::Connection;

    struct TempDirectory {
        path: PathBuf,
    }

    impl TempDirectory {
        fn create(label: &str) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after unix epoch")
                .as_nanos();
            let path = env::temp_dir().join(format!(
                "photo-workroom-db-{label}-{nanos}-{}",
                process::id()
            ));
            fs::create_dir_all(&path).expect("temporary directory should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn exposes_the_database_boundary_snapshot() {
        let snapshot = bootstrap_snapshot();

        assert_eq!(snapshot.name, "db");
        assert!(snapshot.summary.contains("SQLite"));
        assert!(snapshot.summary.contains("typed repository"));
    }

    #[test]
    fn initializes_fresh_database_with_latest_schema_and_wal() {
        let app_data_dir = TempDirectory::create("fresh-bootstrap");
        let database =
            open_catalog_database(app_data_dir.path()).expect("database should initialize");

        assert!(database.database_path().exists());
        assert_eq!(
            database
                .journal_mode()
                .expect("journal mode should be queryable")
                .to_lowercase(),
            "wal"
        );
        assert_eq!(
            database
                .schema_version()
                .expect("schema version should be queryable"),
            LATEST_SCHEMA_VERSION
        );

        for required_table in [
            "assets",
            "asset_variants",
            "tags",
            "asset_tags",
            "ingest_sessions",
            "task_runs",
            "favorite_targets",
            "audit_log",
            "keyword_lists",
            "keyword_terms",
            "preview_cache_entries",
        ] {
            assert!(
                database
                    .table_exists(required_table)
                    .expect("table query should succeed"),
                "missing required table: {required_table}"
            );
        }
    }

    #[test]
    fn applies_pending_migrations_to_an_older_schema() {
        let app_data_dir = TempDirectory::create("upgrade-path");
        let database_path = catalog_database_path(app_data_dir.path());
        fs::create_dir_all(
            database_path
                .parent()
                .expect("catalog_database_path should include a parent directory"),
        )
        .expect("database parent should be created");

        {
            let mut connection = Connection::open(&database_path).expect("database should open");
            configure_connection(&connection).expect("connection should configure");
            apply_migration(&mut connection, &MIGRATIONS[0]).expect("migration 0001 should apply");
            assert_eq!(
                current_schema_version(&connection).expect("version should be queryable"),
                1
            );
        }

        let upgraded = open_catalog_database(app_data_dir.path()).expect("upgrade should succeed");
        assert_eq!(
            upgraded
                .schema_version()
                .expect("version should be queryable after upgrade"),
            LATEST_SCHEMA_VERSION
        );
        assert!(upgraded
            .table_exists("keyword_lists")
            .expect("keyword_lists table should exist"));
        assert!(upgraded
            .table_exists("preview_cache_entries")
            .expect("preview_cache_entries table should exist"));
    }

    #[test]
    fn supports_typed_asset_crud_round_trip() {
        let app_data_dir = TempDirectory::create("typed-crud");
        let database =
            open_catalog_database(app_data_dir.path()).expect("database should initialize");

        let created = database
            .upsert_asset(NewAssetRecord::new("/photos/assignment-001.cr3", 2_048))
            .expect("asset should be inserted");
        assert_eq!(created.canonical_path, "/photos/assignment-001.cr3");
        assert_eq!(created.file_size_bytes, 2_048);
        assert_eq!(database.asset_count().expect("count should succeed"), 1);

        let loaded = database
            .get_asset_by_path("/photos/assignment-001.cr3")
            .expect("asset lookup should succeed")
            .expect("asset should exist");
        assert_eq!(loaded.id, created.id);
        assert_eq!(loaded.file_size_bytes, 2_048);

        let updated = database
            .upsert_asset(NewAssetRecord::new("/photos/assignment-001.cr3", 4_096))
            .expect("asset should be updated");
        assert_eq!(updated.id, created.id);
        assert_eq!(updated.file_size_bytes, 4_096);
        assert_eq!(database.asset_count().expect("count should succeed"), 1);

        let deleted = database
            .delete_asset_by_path("/photos/assignment-001.cr3")
            .expect("delete should succeed");
        assert!(deleted);
        assert_eq!(database.asset_count().expect("count should succeed"), 0);
    }

    #[test]
    fn appends_and_reads_audit_events() {
        let app_data_dir = TempDirectory::create("audit-events");
        let database =
            open_catalog_database(app_data_dir.path()).expect("database should initialize");

        let first = database
            .append_audit_event("scan_result", "root_path=/photos/assets; assets=10")
            .expect("first audit event should insert");
        let second = database
            .append_audit_event("scan_result", "root_path=/photos/assets; assets=12")
            .expect("second audit event should insert");

        assert_eq!(
            database
                .audit_event_count()
                .expect("audit event count should be queryable"),
            2
        );
        assert!(first.id < second.id);

        let latest = database
            .latest_audit_event()
            .expect("latest audit event query should succeed")
            .expect("latest audit event should exist");
        assert_eq!(latest.id, second.id);
        assert_eq!(latest.event_type, "scan_result");
        assert!(latest.details.contains("assets=12"));
        assert!(!latest.created_at.trim().is_empty());
    }

    #[test]
    fn rejects_invalid_audit_event_payloads() {
        let app_data_dir = TempDirectory::create("audit-events-invalid");
        let database =
            open_catalog_database(app_data_dir.path()).expect("database should initialize");

        let invalid_event_type = database
            .append_audit_event("   ", "root_path=/photos/assets; assets=10")
            .expect_err("empty event type should fail");
        let invalid_details = database
            .append_audit_event("scan_result", "   ")
            .expect_err("empty details should fail");

        assert!(matches!(
            invalid_event_type,
            DbError::Validation("event_type must not be empty")
        ));
        assert!(matches!(
            invalid_details,
            DbError::Validation("event details must not be empty")
        ));
    }
}
