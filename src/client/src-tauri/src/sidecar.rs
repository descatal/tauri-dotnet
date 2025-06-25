use std::path::PathBuf;
use std::process::Stdio;
use tauri::{AppHandle, Manager};
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use log::{debug, warn, error};

pub async fn launch_service(app: AppHandle) {
    match find_and_launch_service(&app).await {
        Ok(_) => debug!("Service launched successfully"),
        Err(e) => {
            warn!("Failed to launch service: {}. App will continue normally.", e);
        }
    }
}

async fn find_and_launch_service(app: &AppHandle) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service_path = find_service_executable(app)?;
    debug!("Found service executable at: {:?}", service_path);
    
    execute_service(service_path).await?;
    Ok(())
}

fn find_service_executable(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let resource_dir = app.path().resource_dir()
        .map_err(|e| format!("Failed to get resource directory: {}", e))?;
    
    // Try different executable names based on platform
    let executable_names = get_executable_names();
    
    for name in executable_names {
        let service_path = resource_dir.join(&name);
        debug!("Checking for service at: {:?}", service_path);
        
        if service_path.exists() {
            // Verify it's executable (Unix-like systems)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(metadata) = std::fs::metadata(&service_path) {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 == 0 {
                        warn!("Service file found but not executable: {:?}", service_path);
                        continue;
                    }
                }
            }
            
            return Ok(service_path);
        }
    }
    
    Err(format!("Service executable not found in resource directory: {:?}", resource_dir).into())
}

fn get_executable_names() -> Vec<String> {
    #[cfg(windows)]
    {
        vec!["service.exe".to_string(), "service".to_string()]
    }
    
    #[cfg(not(windows))]
    {
        vec!["service".to_string()]
    }
}

async fn execute_service(service_path: PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    debug!("Executing service: {:?}", service_path);
    
    let mut cmd = Command::new(&service_path);
    cmd.stdout(Stdio::piped())
       .stderr(Stdio::piped())
       .stdin(Stdio::null());
    
    // Set working directory to the service's directory
    if let Some(parent) = service_path.parent() {
        cmd.current_dir(parent);
    }
    
    let mut child = cmd.spawn()
        .map_err(|e| format!("Failed to spawn service process: {}", e))?;
    
    // Handle stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        tauri::async_runtime::spawn(async move {
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                debug!("Service stdout: {}", line.trim());
            }
        });
    }
    
    // Handle stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        tauri::async_runtime::spawn(async move {
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                debug!("Service stderr: {}", line.trim());
            }
        });
    }
    
    // Monitor the process status
    tauri::async_runtime::spawn(async move {
        match child.wait().await {
            Ok(status) => {
                if status.success() {
                    debug!("Service process completed successfully");
                } else {
                    warn!("Service process exited with status: {}", status);
                }
            }
            Err(e) => {
                error!("Error waiting for service process: {}", e);
            }
        }
    });
    
    debug!("Service process started successfully");
    Ok(())
}

// Convenience function to call from your main app
pub fn initialize_service(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        // Use the event-based version that matches your logging pattern
        launch_service(app).await;
    });
}