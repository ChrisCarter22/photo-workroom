use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{Display, Formatter},
    fs,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::UNIX_EPOCH,
};

use photo_workroom_core::SubsystemSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    Raw,
    Jpeg,
    Image,
    Video,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedAsset {
    pub canonical_path: String,
    pub parent_folder: String,
    pub file_name: String,
    pub stem: String,
    pub media_type: MediaType,
    pub file_size_bytes: u64,
    pub modified_unix_timestamp_secs: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawJpegPair {
    pub raw_path: String,
    pub jpeg_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarLink {
    pub asset_path: String,
    pub sidecar_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataExtractionJob {
    pub asset_path: String,
    pub sidecar_path: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScanProgress {
    pub directories_scanned: u64,
    pub files_scanned: u64,
    pub assets_detected: u64,
    pub sidecars_detected: u64,
    pub pairs_detected: u64,
    pub unsupported_files: u64,
    pub hidden_entries_skipped: u64,
    pub excluded_entries_skipped: u64,
}

#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    pub include_hidden: bool,
    pub excluded_names: Vec<String>,
    pub cancellation: Option<ScanCancellation>,
}

#[derive(Debug, Clone, Default)]
pub struct ScanCancellation {
    cancelled: Arc<AtomicBool>,
}

impl ScanCancellation {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

#[derive(Debug)]
pub enum ScanError {
    InvalidRoot(PathBuf),
    Cancelled,
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl Display for ScanError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRoot(path) => {
                write!(
                    formatter,
                    "scan root must be an existing directory: {}",
                    path.display()
                )
            }
            Self::Cancelled => write!(formatter, "scan was cancelled"),
            Self::Io { path, source } => {
                write!(formatter, "I/O error at {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for ScanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::InvalidRoot(_) | Self::Cancelled => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanResult {
    pub root_path: String,
    pub assets: Vec<ScannedAsset>,
    pub raw_jpeg_pairs: Vec<RawJpegPair>,
    pub sidecar_links: Vec<SidecarLink>,
    pub metadata_jobs: Vec<MetadataExtractionJob>,
    pub warnings: Vec<String>,
    pub progress: ScanProgress,
}

#[derive(Debug, Default)]
struct PairBucket {
    raw_paths: Vec<String>,
    jpeg_paths: Vec<String>,
}

#[derive(Debug)]
struct SidecarCandidate {
    sidecar_path: String,
    key: (String, String),
}

#[derive(Debug, Clone)]
struct PreferredAsset {
    path: String,
    media_type: MediaType,
}

pub fn scan_folder(root: &Path) -> Result<ScanResult, ScanError> {
    scan_folder_with_options(root, &ScanOptions::default())
}

pub fn scan_folder_with_options(
    root: &Path,
    options: &ScanOptions,
) -> Result<ScanResult, ScanError> {
    scan_folder_with_progress(root, options, |_| {})
}

pub fn scan_folder_with_progress<F>(
    root: &Path,
    options: &ScanOptions,
    mut on_progress: F,
) -> Result<ScanResult, ScanError>
where
    F: FnMut(&ScanProgress),
{
    if !root.is_dir() {
        return Err(ScanError::InvalidRoot(root.to_path_buf()));
    }

    let canonical_root = root.canonicalize().map_err(|error| ScanError::Io {
        path: root.to_path_buf(),
        source: error,
    })?;
    let root_path = normalize_path(&canonical_root);
    let excluded_names: BTreeSet<String> = options
        .excluded_names
        .iter()
        .map(|name| name.trim().to_lowercase())
        .filter(|name| !name.is_empty())
        .collect();

    let mut progress = ScanProgress::default();
    let mut assets = Vec::new();
    let mut sidecar_candidates = Vec::new();
    let mut directory_stack = vec![canonical_root.clone()];

    while let Some(directory) = directory_stack.pop() {
        ensure_not_cancelled(options)?;
        progress.directories_scanned += 1;

        let entries = fs::read_dir(&directory).map_err(|error| ScanError::Io {
            path: directory.clone(),
            source: error,
        })?;

        for entry in entries {
            ensure_not_cancelled(options)?;
            let entry = entry.map_err(|error| ScanError::Io {
                path: directory.clone(),
                source: error,
            })?;

            let file_name = entry.file_name().to_string_lossy().to_string();
            let name_lowercase = file_name.to_lowercase();
            let entry_path = entry.path();
            let file_type = entry.file_type().map_err(|error| ScanError::Io {
                path: entry_path.clone(),
                source: error,
            })?;

            if !options.include_hidden && is_hidden_name(&file_name) {
                progress.hidden_entries_skipped += 1;
                continue;
            }

            if excluded_names.contains(&name_lowercase) {
                progress.excluded_entries_skipped += 1;
                continue;
            }

            if file_type.is_dir() {
                directory_stack.push(entry_path);
                continue;
            }

            if !file_type.is_file() {
                continue;
            }

            progress.files_scanned += 1;
            let metadata = entry.metadata().map_err(|error| ScanError::Io {
                path: entry_path.clone(),
                source: error,
            })?;
            let modified_unix_timestamp_secs = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs());
            let file_size_bytes = metadata.len();

            let Some((stem, extension)) = file_stem_and_extension(&file_name) else {
                progress.unsupported_files += 1;
                on_progress(&progress);
                continue;
            };

            let canonical_path = entry_path.canonicalize().map_err(|error| ScanError::Io {
                path: entry_path.clone(),
                source: error,
            })?;
            let canonical_path_string = normalize_path(&canonical_path);
            let parent_folder = canonical_path
                .parent()
                .map(normalize_path)
                .unwrap_or_else(String::new);
            let key = (parent_folder.to_lowercase(), stem.to_lowercase());

            if extension == "xmp" {
                sidecar_candidates.push(SidecarCandidate {
                    sidecar_path: canonical_path_string,
                    key,
                });
                progress.sidecars_detected += 1;
                on_progress(&progress);
                continue;
            }

            let Some(media_type) = classify_asset_extension(&extension) else {
                progress.unsupported_files += 1;
                on_progress(&progress);
                continue;
            };

            assets.push(ScannedAsset {
                canonical_path: canonical_path_string,
                parent_folder,
                file_name,
                stem,
                media_type,
                file_size_bytes,
                modified_unix_timestamp_secs,
            });
            progress.assets_detected += 1;
            on_progress(&progress);
        }
    }

    assets.sort_by(|left, right| left.canonical_path.cmp(&right.canonical_path));
    let mut warnings = Vec::new();
    let mut raw_jpeg_pairs = detect_raw_jpeg_pairs(&assets, &mut warnings);
    progress.pairs_detected = raw_jpeg_pairs.len() as u64;
    raw_jpeg_pairs.sort_by(|left, right| left.raw_path.cmp(&right.raw_path));

    let mut sidecar_links = link_sidecars(&assets, &sidecar_candidates, &mut warnings);
    sidecar_links.sort_by(|left, right| left.sidecar_path.cmp(&right.sidecar_path));

    let sidecar_by_asset = build_sidecar_index(&sidecar_links, &mut warnings);
    let metadata_jobs = assets
        .iter()
        .map(|asset| MetadataExtractionJob {
            asset_path: asset.canonical_path.clone(),
            sidecar_path: sidecar_by_asset.get(&asset.canonical_path).cloned(),
        })
        .collect();

    Ok(ScanResult {
        root_path,
        assets,
        raw_jpeg_pairs,
        sidecar_links,
        metadata_jobs,
        warnings,
        progress,
    })
}

pub fn bootstrap_snapshot() -> SubsystemSnapshot {
    SubsystemSnapshot::new(
        "ingest",
        "Phase 4",
        "Filesystem scan baseline now covers recursion, hidden-file policy, media classification, pairing, sidecar linking, and metadata-queue planning.",
    )
}

fn ensure_not_cancelled(options: &ScanOptions) -> Result<(), ScanError> {
    if options
        .cancellation
        .as_ref()
        .is_some_and(ScanCancellation::is_cancelled)
    {
        return Err(ScanError::Cancelled);
    }

    Ok(())
}

fn is_hidden_name(file_name: &str) -> bool {
    file_name.starts_with('.')
}

fn file_stem_and_extension(file_name: &str) -> Option<(String, String)> {
    let path = Path::new(file_name);
    let stem = path.file_stem()?.to_string_lossy().to_string();
    let extension = path.extension()?.to_string_lossy().to_lowercase();

    Some((stem, extension))
}

fn classify_asset_extension(extension: &str) -> Option<MediaType> {
    const RAW_EXTENSIONS: &[&str] = &[
        "3fr", "arw", "cr2", "cr3", "dng", "erf", "kdc", "mos", "mrw", "nef", "nrw", "orf", "pef",
        "raf", "raw", "rw2", "sr2", "srf", "srw",
    ];
    const IMAGE_EXTENSIONS: &[&str] = &["gif", "heic", "png", "tif", "tiff", "webp"];
    const VIDEO_EXTENSIONS: &[&str] = &["avi", "m4v", "mov", "mp4", "mts", "mxf"];

    if RAW_EXTENSIONS.contains(&extension) {
        return Some(MediaType::Raw);
    }
    if matches!(extension, "jpg" | "jpeg") {
        return Some(MediaType::Jpeg);
    }
    if IMAGE_EXTENSIONS.contains(&extension) {
        return Some(MediaType::Image);
    }
    if VIDEO_EXTENSIONS.contains(&extension) {
        return Some(MediaType::Video);
    }

    None
}

fn detect_raw_jpeg_pairs(assets: &[ScannedAsset], warnings: &mut Vec<String>) -> Vec<RawJpegPair> {
    let mut pair_buckets: BTreeMap<(String, String), PairBucket> = BTreeMap::new();

    for asset in assets {
        if !matches!(asset.media_type, MediaType::Raw | MediaType::Jpeg) {
            continue;
        }

        let key = (
            asset.parent_folder.to_lowercase(),
            asset.stem.to_lowercase(),
        );
        let bucket = pair_buckets.entry(key).or_default();
        if asset.media_type == MediaType::Raw {
            bucket.raw_paths.push(asset.canonical_path.clone());
        } else {
            bucket.jpeg_paths.push(asset.canonical_path.clone());
        }
    }

    let mut pairs = Vec::new();
    for ((folder, stem), bucket) in pair_buckets {
        if bucket.raw_paths.len() > 1 {
            warnings.push(format!(
                "multiple RAW candidates for stem '{stem}' in '{folder}'"
            ));
        }
        if bucket.jpeg_paths.len() > 1 {
            warnings.push(format!(
                "multiple JPEG candidates for stem '{stem}' in '{folder}'"
            ));
        }

        if let (Some(raw_path), Some(jpeg_path)) =
            (bucket.raw_paths.first(), bucket.jpeg_paths.first())
        {
            pairs.push(RawJpegPair {
                raw_path: raw_path.clone(),
                jpeg_path: jpeg_path.clone(),
            });
        }
    }

    pairs
}

fn link_sidecars(
    assets: &[ScannedAsset],
    sidecar_candidates: &[SidecarCandidate],
    warnings: &mut Vec<String>,
) -> Vec<SidecarLink> {
    let mut preferred_assets: BTreeMap<(String, String), PreferredAsset> = BTreeMap::new();

    for asset in assets {
        let key = (
            asset.parent_folder.to_lowercase(),
            asset.stem.to_lowercase(),
        );
        let candidate = PreferredAsset {
            path: asset.canonical_path.clone(),
            media_type: asset.media_type,
        };
        match preferred_assets.get(&key) {
            Some(existing)
                if sidecar_priority(existing.media_type) <= sidecar_priority(asset.media_type) => {}
            _ => {
                preferred_assets.insert(key, candidate);
            }
        }
    }

    let mut links = Vec::new();
    for sidecar in sidecar_candidates {
        if let Some(asset) = preferred_assets.get(&sidecar.key) {
            links.push(SidecarLink {
                asset_path: asset.path.clone(),
                sidecar_path: sidecar.sidecar_path.clone(),
            });
        } else {
            warnings.push(format!(
                "unmatched sidecar without asset pair: {}",
                sidecar.sidecar_path
            ));
        }
    }

    links
}

fn build_sidecar_index(
    sidecar_links: &[SidecarLink],
    warnings: &mut Vec<String>,
) -> BTreeMap<String, String> {
    let mut by_asset = BTreeMap::new();

    for link in sidecar_links {
        if let Some(existing_path) =
            by_asset.insert(link.asset_path.clone(), link.sidecar_path.clone())
        {
            if existing_path != link.sidecar_path {
                warnings.push(format!(
                    "multiple sidecars linked to asset '{}'; using '{}'",
                    link.asset_path, link.sidecar_path
                ));
            }
        }
    }

    by_asset
}

fn sidecar_priority(media_type: MediaType) -> u8 {
    match media_type {
        MediaType::Raw => 0,
        MediaType::Jpeg => 1,
        MediaType::Image => 2,
        MediaType::Video => 3,
    }
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
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
                "photo-workroom-ingest-{label}-{nanos}-{}",
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
    fn exposes_the_ingest_boundary_snapshot() {
        let snapshot = bootstrap_snapshot();

        assert_eq!(snapshot.name, "ingest");
        assert!(snapshot.summary.contains("Filesystem scan baseline"));
    }

    #[test]
    fn scans_recursively_with_hidden_and_excluded_policies() {
        let temp_dir = TempDirectory::create("scan-recursive");
        let root = temp_dir.path().join("root");
        let session = root.join("session-01");
        let hidden_folder = root.join(".hidden-cache");
        let excluded_folder = root.join("@eaDir");

        fs::create_dir_all(&session).expect("session folder should be created");
        fs::create_dir_all(&hidden_folder).expect("hidden folder should be created");
        fs::create_dir_all(&excluded_folder).expect("excluded folder should be created");

        fs::write(session.join("IMG_0001.CR3"), b"raw").expect("raw file should be written");
        fs::write(session.join("IMG_0001.JPG"), b"jpeg").expect("jpeg file should be written");
        fs::write(session.join("IMG_0001.XMP"), b"sidecar").expect("xmp file should be written");
        fs::write(session.join("IMG_0002.NEF"), b"raw").expect("nef file should be written");
        fs::write(session.join("notes.txt"), b"note").expect("unsupported file should be written");
        fs::write(hidden_folder.join("hidden.CR3"), b"raw").expect("hidden file should be written");
        fs::write(excluded_folder.join("ignored.JPG"), b"jpeg")
            .expect("excluded file should be written");

        let result = scan_folder_with_options(
            &root,
            &ScanOptions {
                include_hidden: false,
                excluded_names: vec!["@eadir".to_string()],
                cancellation: None,
            },
        )
        .expect("scan should succeed");

        assert_eq!(result.assets.len(), 3);
        assert_eq!(result.raw_jpeg_pairs.len(), 1);
        assert_eq!(result.sidecar_links.len(), 1);
        assert_eq!(result.metadata_jobs.len(), 3);

        assert_eq!(result.progress.assets_detected, 3);
        assert_eq!(result.progress.sidecars_detected, 1);
        assert_eq!(result.progress.pairs_detected, 1);
        assert_eq!(result.progress.unsupported_files, 1);
        assert_eq!(result.progress.hidden_entries_skipped, 1);
        assert_eq!(result.progress.excluded_entries_skipped, 1);

        let linked_asset = &result.sidecar_links[0].asset_path;
        assert!(linked_asset.ends_with("IMG_0001.CR3"));
        assert!(result
            .metadata_jobs
            .iter()
            .any(|job| job.asset_path.ends_with("IMG_0001.CR3") && job.sidecar_path.is_some()));
    }

    #[test]
    fn reports_pair_collisions_for_duplicate_stems() {
        let temp_dir = TempDirectory::create("scan-collision");
        let root = temp_dir.path().join("root");
        fs::create_dir_all(&root).expect("root folder should be created");

        fs::write(root.join("scene.CR3"), b"raw").expect("raw file should be written");
        fs::write(root.join("scene.NEF"), b"raw").expect("second raw file should be written");
        fs::write(root.join("scene.JPG"), b"jpeg").expect("jpeg file should be written");
        fs::write(root.join("scene.JPEG"), b"second jpeg")
            .expect("second jpeg file should be written");

        let result = scan_folder(&root).expect("scan should succeed");

        assert_eq!(result.raw_jpeg_pairs.len(), 1);
        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("multiple RAW candidates")));
        assert!(result
            .warnings
            .iter()
            .any(|warning| warning.contains("multiple JPEG candidates")));
    }

    #[test]
    fn supports_cancellation_and_progress_callbacks() {
        let temp_dir = TempDirectory::create("scan-cancel");
        let root = temp_dir.path().join("root");
        fs::create_dir_all(&root).expect("root folder should be created");
        fs::write(root.join("A.CR3"), b"raw").expect("file A should be written");
        fs::write(root.join("B.CR3"), b"raw").expect("file B should be written");
        fs::write(root.join("C.CR3"), b"raw").expect("file C should be written");

        let cancellation = ScanCancellation::new();
        let mut callback_count = 0_u64;
        let scan_result = scan_folder_with_progress(
            &root,
            &ScanOptions {
                include_hidden: false,
                excluded_names: Vec::new(),
                cancellation: Some(cancellation.clone()),
            },
            |progress| {
                callback_count += 1;
                if progress.files_scanned >= 1 {
                    cancellation.cancel();
                }
            },
        );

        assert!(matches!(scan_result, Err(ScanError::Cancelled)));
        assert!(callback_count >= 1);
    }
}
