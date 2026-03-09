use std::{
    collections::HashMap,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};

use photo_workroom_core::{
    ConsumeWindowFolderOpenRequestResponse, HealthCheckRequest, HealthCheckResponse,
    OpenFolderInSeparateWindowRequest, OpenFolderInSeparateWindowResponse,
};
use tauri::Manager;

#[derive(Default)]
struct PendingWindowFolderOpenRequests {
    by_window_label: Mutex<HashMap<String, String>>,
}

impl PendingWindowFolderOpenRequests {
    fn queue(&self, window_label: String, folder_path: String) -> Result<(), String> {
        let mut state = self
            .by_window_label
            .lock()
            .map_err(|_| "pending window-folder state is poisoned".to_string())?;
        state.insert(window_label, folder_path);
        Ok(())
    }

    fn consume(&self, window_label: &str) -> Result<Option<String>, String> {
        let mut state = self
            .by_window_label
            .lock()
            .map_err(|_| "pending window-folder state is poisoned".to_string())?;
        Ok(state.remove(window_label))
    }
}

fn folder_basename(folder_path: &str) -> String {
    let normalized = folder_path.trim().trim_end_matches(['/', '\\']);

    normalized
        .rsplit(['/', '\\'])
        .find(|segment| !segment.is_empty())
        .unwrap_or("Folder")
        .to_string()
}

fn slugify_window_label(folder_path: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_dash = false;

    for c in folder_basename(folder_path).chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            previous_was_dash = false;
        } else if !previous_was_dash {
            slug.push('-');
            previous_was_dash = true;
        }
    }

    let normalized = slug.trim_matches('-');
    if normalized.is_empty() {
        "folder".to_string()
    } else {
        normalized.to_string()
    }
}

fn next_folder_window_label(app: &tauri::AppHandle, folder_path: &str) -> Result<String, String> {
    let epoch_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "system clock must not be before UNIX epoch".to_string())?
        .as_millis();
    let slug = slugify_window_label(folder_path);

    let mut collision_index = 0_u32;
    loop {
        let candidate = if collision_index == 0 {
            format!("workspace-{slug}-{epoch_millis}")
        } else {
            format!("workspace-{slug}-{epoch_millis}-{collision_index}")
        };

        if app.get_webview_window(&candidate).is_none() {
            return Ok(candidate);
        }

        collision_index += 1;
    }
}

fn build_health_response(request: HealthCheckRequest) -> Result<HealthCheckResponse, String> {
    request.validate().map_err(str::to_string)?;

    Ok(HealthCheckResponse {
        request_id: request.request_id,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        runtime: "tauri-v2".to_string(),
        active_workspace: request.active_workspace,
        healthy: true,
        message: "Desktop shell baseline is healthy.".to_string(),
        subsystems: vec![
            photo_workroom_db::bootstrap_snapshot(),
            photo_workroom_image::bootstrap_snapshot(),
            photo_workroom_ingest::bootstrap_snapshot(),
            photo_workroom_metadata::bootstrap_snapshot(),
            photo_workroom_task_runtime::bootstrap_snapshot(),
        ],
        queues: photo_workroom_task_runtime::bootstrap_queues(),
    })
}

#[tauri::command]
fn health_check(request: HealthCheckRequest) -> Result<HealthCheckResponse, String> {
    build_health_response(request)
}

#[tauri::command]
async fn open_folder_in_separate_window(
    app: tauri::AppHandle,
    pending_requests: tauri::State<'_, PendingWindowFolderOpenRequests>,
    request: OpenFolderInSeparateWindowRequest,
) -> Result<OpenFolderInSeparateWindowResponse, String> {
    request.validate().map_err(str::to_string)?;

    let window_label = next_folder_window_label(&app, &request.folder_path)?;
    pending_requests.queue(window_label.clone(), request.folder_path.clone())?;

    let window_title = format!("Photo Workroom - {}", folder_basename(&request.folder_path));
    if let Err(error) = tauri::WebviewWindowBuilder::new(
        &app,
        &window_label,
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title(&window_title)
    .build()
    {
        let _ = pending_requests.consume(&window_label);
        return Err(format!("failed to open separate window: {error}"));
    }

    Ok(OpenFolderInSeparateWindowResponse {
        request_id: request.request_id,
        window_label,
        folder_path: request.folder_path,
        active_workspace: request.active_workspace,
        opened: true,
        message: "Opened folder in a separate window.".to_string(),
    })
}

#[tauri::command]
fn consume_window_folder_open_request(
    window: tauri::Window,
    pending_requests: tauri::State<'_, PendingWindowFolderOpenRequests>,
) -> Result<ConsumeWindowFolderOpenRequestResponse, String> {
    let window_label = window.label().to_string();
    let folder_path = pending_requests.consume(&window_label)?;

    Ok(ConsumeWindowFolderOpenRequestResponse {
        window_label,
        folder_path,
    })
}

pub fn run() {
    let _ = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .try_init();
    tracing::info!("starting Photo Workroom desktop shell");

    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().map_err(|error| {
                std::io::Error::other(format!(
                    "failed to resolve app data directory for catalog database: {error}"
                ))
            })?;
            let database =
                photo_workroom_db::open_catalog_database(&app_data_dir).map_err(|error| {
                    std::io::Error::other(format!(
                        "failed to initialize local catalog database: {error}"
                    ))
                })?;
            let schema_version = database.schema_version().map_err(|error| {
                std::io::Error::other(format!(
                    "failed to read catalog schema version after initialization: {error}"
                ))
            })?;

            tracing::info!(
                db_path = %database.database_path().display(),
                schema_version,
                "initialized local catalog database"
            );
            Ok(())
        })
        .manage(PendingWindowFolderOpenRequests::default())
        .invoke_handler(tauri::generate_handler![
            health_check,
            open_folder_in_separate_window,
            consume_window_folder_open_request
        ])
        .run(tauri::generate_context!())
        .expect("error while running Photo Workroom");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_health_response_for_the_active_workspace() {
        let response = build_health_response(HealthCheckRequest {
            request_id: "health-001".to_string(),
            active_workspace: "Assignments".to_string(),
        })
        .expect("health response should build");

        assert!(response.healthy);
        assert_eq!(response.active_workspace, "Assignments");
        assert_eq!(response.runtime, "tauri-v2");
        assert_eq!(response.queues[0].name, "preview");
    }

    #[test]
    fn slugifies_folder_paths_for_window_labels() {
        assert_eq!(
            slugify_window_label("/Volumes/Card-01/Match Day"),
            "match-day"
        );
        assert_eq!(slugify_window_label("///"), "folder");
    }

    #[test]
    fn queues_and_consumes_pending_window_folder_requests() {
        let pending_requests = PendingWindowFolderOpenRequests::default();

        pending_requests
            .queue(
                "workspace-match-day-1".to_string(),
                "/Assignments/Match Day".to_string(),
            )
            .expect("queue should be writable");

        let consumed = pending_requests
            .consume("workspace-match-day-1")
            .expect("consume should work");
        let consumed_again = pending_requests
            .consume("workspace-match-day-1")
            .expect("consume should work when empty");

        assert_eq!(consumed.as_deref(), Some("/Assignments/Match Day"));
        assert_eq!(consumed_again, None);
    }
}
