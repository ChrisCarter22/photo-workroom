use photo_workroom_core::{HealthCheckRequest, HealthCheckResponse};

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

pub fn run() {
    let _ = tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .try_init();
    tracing::info!("starting Photo Workroom desktop shell");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![health_check])
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
}
