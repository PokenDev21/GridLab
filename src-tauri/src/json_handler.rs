use std::path::PathBuf;
use serde_json::{Map, Value};

pub fn get_workspaces_path() -> Result<PathBuf, String> {
    // Always use current directory (where the exe is, or project root in dev)
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    let json_path = current_dir.join("workspaces.json");
    
    // Create default file if it doesn't exist
    if !json_path.exists() {
        let default_workspaces = serde_json::json!({
            "Math Layout": {
                "main1": { "url": "https://app.haldor.se/" },
                "main2": { "url": "https://nokportalen.se/" },
                "main3": { "url": "https://www.onenote.com/?public=1&wdorigin=ondcauth2&wdorigin=ondc" },
                "main4": { "url": "https://www.microsoft.com/sv-se/microsoft-teams/log-in?market=se" }
            },
            "Programming Layout": {
                "main1": { "url": "https://app.haldor.se/" },
                "main2": { "url": "https://github.com/" },
                "main3": { "url": "https://chatgpt.com/" },
                "main4": { "url": "https://www.w3schools.com/" }
            },
            "Swedish Layout": {
                "main1": { "url": "https://word.cloud.microsoft/en-us/" },
                "main2": { "url": "https://app.haldor.se/" },
                "main3": { "url": "https://teams.microsoft.com/v2/" },
                "main4": { "url": "https://svenska.se/" }
            }
        });
        
        let json_string = serde_json::to_string_pretty(&default_workspaces)
            .map_err(|e| format!("serialize default workspaces: {}", e))?;
        std::fs::write(&json_path, json_string)
            .map_err(|e| format!("write default workspaces: {}", e))?;
        
        println!("Created default workspaces.json at: {:?}", json_path);
    }
    
    Ok(json_path)
}
pub fn read_workspace(layout: &str) -> Result<Value, String> {
    let json_path = get_workspaces_path()?;
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let all: Value = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    all.get(layout)
        .cloned()
        .ok_or_else(|| format!("Workspace '{}' not found", layout))
}
pub fn delete_workspace_in_storage(name: String) -> Result<(), String> 
{
    let json_path = get_workspaces_path()?;
    
    // Read existing workspaces
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let mut all: Map<String, Value> = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    
    // Remove the workspace
    all.remove(&name);
    
    // Write back to file
    let json_string = serde_json::to_string_pretty(&all)
        .map_err(|e| format!("serialize workspaces: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("write {:?}: {}", json_path, e))?;
    
    Ok(())
}
pub fn save_workspace_in_storage(name: String, config: Value) -> Result<(), String>
{
    let json_path = get_workspaces_path()?;
    
    // Read existing workspaces
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let mut all: Map<String, Value> = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    
    // Update or add the workspace
    all.insert(name, config);
    
    // Write back to file
    let json_string = serde_json::to_string_pretty(&all)
        .map_err(|e| format!("serialize workspaces: {}", e))?;
    std::fs::write(&json_path, json_string)
        .map_err(|e| format!("write {:?}: {}", json_path, e))?;
    
    Ok(())
}
pub fn get_all_workspaces_in_storage() -> Result<Value, String> {
    let json_path = get_workspaces_path()?;
    
    let json = std::fs::read_to_string(&json_path)
        .map_err(|e| format!("read {:?}: {}", json_path, e))?;
    let all: Value = serde_json::from_str(&json)
        .map_err(|e| format!("parse workspaces.json: {}", e))?;
    Ok(all)
}

