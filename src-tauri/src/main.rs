#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod commands;
mod models;
mod providers;
mod memory;
mod tools;
mod permissions;
mod audit;
mod agent;
mod voice;
mod settings;
mod api_catalog;
mod api_connector;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

pub struct AppState {
    pub db: Arc<db::Database>,
    pub settings: Arc<RwLock<settings::SettingsManager>>,
    pub model_manager: Arc<RwLock<models::ModelManager>>,
    pub conversation_manager: Arc<RwLock<models::ConversationManager>>,
    pub memory_manager: Arc<RwLock<memory::MemoryManager>>,
    pub permission_manager: Arc<RwLock<permissions::PermissionManager>>,
    pub audit_log: Arc<RwLock<audit::AuditLog>>,
    pub tool_registry: Arc<RwLock<tools::ToolRegistry>>,
    pub api_catalog: Arc<RwLock<api_catalog::ApiCatalogManager>>,
    pub api_credentials: Arc<RwLock<api_connector::ApiCredentialManager>>,
    pub agent_engine: Arc<RwLock<agent::AgentEngine>>,
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            std::fs::create_dir_all(&app_dir).ok();

            let db_path = app_dir.join("luna.db");
            let db = db::Database::new(&db_path).expect("Failed to initialize database");

            let db_arc = Arc::new(db);
            let settings_manager = settings::SettingsManager::new(Arc::clone(&db_arc));
            let model_manager = models::ModelManager::new(Arc::clone(&db_arc));
            let conversation_manager = models::ConversationManager::new(Arc::clone(&db_arc));
            let memory_manager = memory::MemoryManager::new(Arc::clone(&db_arc));
            let permission_manager = permissions::PermissionManager::new(Arc::clone(&db_arc));
            let audit_log = audit::AuditLog::new(Arc::clone(&db_arc));
            let tool_registry = tools::ToolRegistry::new();
            let api_catalog = api_catalog::ApiCatalogManager::new(Arc::clone(&db_arc));
            let api_credentials = api_connector::ApiCredentialManager::new(Arc::clone(&db_arc));
            let agent_engine = agent::AgentEngine::new(Arc::clone(&db_arc));

            let state = AppState {
                db: db_arc,
                settings: Arc::new(RwLock::new(settings_manager)),
                model_manager: Arc::new(RwLock::new(model_manager)),
                conversation_manager: Arc::new(RwLock::new(conversation_manager)),
                memory_manager: Arc::new(RwLock::new(memory_manager)),
                permission_manager: Arc::new(RwLock::new(permission_manager)),
                audit_log: Arc::new(RwLock::new(audit_log)),
                tool_registry: Arc::new(RwLock::new(tool_registry)),
                api_catalog: Arc::new(RwLock::new(api_catalog)),
                api_credentials: Arc::new(RwLock::new(api_credentials)),
                agent_engine: Arc::new(RwLock::new(agent_engine)),
            };

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::providers::add_provider_account,
            commands::providers::get_provider_accounts,
            commands::providers::delete_provider_account,
            commands::providers::update_provider_account,
            commands::models::create_model_profile,
            commands::models::get_model_profiles,
            commands::models::update_model_profile,
            commands::models::delete_model_profile,
            commands::models::get_active_model_profile,
            commands::models::set_active_model_profile,
            commands::chat::create_conversation,
            commands::chat::get_conversations,
            commands::chat::get_messages,
            commands::chat::send_message,
            commands::chat::delete_conversation,
            commands::chat::rename_conversation,
            commands::chat::cancel_generation,
            commands::memory::create_memory,
            commands::memory::get_memories,
            commands::memory::search_memories,
            commands::memory::update_memory,
            commands::memory::delete_memory,
            commands::permissions::get_permissions,
            commands::permissions::update_permission,
            commands::audit::get_audit_logs,
            commands::tools::get_tool_list,
            commands::catalog::sync_catalog,
            commands::catalog::search_catalog,
            commands::catalog::get_catalog_stats,
            commands::catalog::get_catalog_entry,
            commands::catalog::toggle_catalog_entry,
            commands::catalog::get_categories,
            commands::catalog::create_api_tool,
            commands::catalog::delete_api_tool,
            commands::catalog::get_api_tools,
            commands::credentials::create_credential,
            commands::credentials::get_credentials,
            commands::credentials::get_credentials_for_provider,
            commands::credentials::delete_credential,
            commands::credentials::set_active_credential,
            commands::credentials::test_credential,
            commands::credentials::update_credential,
            commands::activity::create_activity_event,
            commands::activity::get_activity_events,
            commands::activity::update_activity_event,
            commands::system::get_system_stats,
            commands::voice::tts_speak,
            commands::voice::stt_transcribe,
            commands::voice::tts_check_availability,
            commands::voice::stt_check_availability,
            commands::update::check_for_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running luna");
}
