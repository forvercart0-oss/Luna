// ── Settings ──

export interface Settings {
  general: GeneralSettings;
  ai: AiSettings;
  voice: VoiceSettings;
}

export interface GeneralSettings {
  theme: string;
  language: string;
}

export interface AiSettings {
  default_provider: string;
  auto_save_conversations: boolean;
  memory_enabled: boolean;
  max_context_messages: number;
}

export interface VoiceSettings {
  tts_enabled: boolean;
  stt_enabled: boolean;
  voice: string;
  speed: number;
  volume: number;
}

// ── Providers ──

export interface ProviderAccount {
  id: string;
  provider: string;
  name: string;
  api_key_masked: string;
  base_url: string | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

// ── Models ──

export interface ModelProfile {
  id: string;
  name: string;
  provider: string;
  model_id: string;
  system_prompt: string | null;
  temperature: number;
  max_tokens: number;
  is_active: boolean;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

// ── Conversations ──

export interface Conversation {
  id: string;
  title: string;
  model_profile_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface Message {
  id: string;
  conversation_id: string;
  role: "system" | "user" | "assistant" | "tool";
  content: string;
  model_profile_id: string | null;
  status: "streaming" | "complete" | "error" | "cancelled";
  metadata: string | null;
  created_at: string;
}

// ── Memory ──

export interface Memory {
  id: string;
  type: string;
  content: string;
  importance: number;
  source: string | null;
  tags: string | null;
  created_at: string;
  updated_at: string;
}

// ── Permissions ──

export interface Permission {
  id: string;
  tool_id: string;
  permission: string;
  scope: string | null;
  created_at: string;
  updated_at: string;
}

// ── Audit ──

export interface AuditEntry {
  id: string;
  timestamp: string;
  action: string;
  tool_id: string | null;
  arguments_summary: string | null;
  result_summary: string | null;
  success: boolean;
  permission_state: string | null;
  metadata: string | null;
}

// ── Tools ──

export interface ToolDefinition {
  id: string;
  name: string;
  description: string;
  parameters: Record<string, unknown>;
  permission_level: string;
  category: string;
}

// ── API Catalog ──

export interface ApiCatalogEntry {
  id: string;
  name: string;
  description: string;
  auth_type: string;
  https: boolean;
  cors: string;
  category: string;
  homepage: string;
  enabled: boolean;
  health_status: string;
  last_health_check: string | null;
  created_at: string;
  updated_at: string;
}

export interface CatalogSyncResult {
  added: number;
  updated: number;
  removed: number;
  total: number;
  sync_time: string;
}

export interface CatalogStats {
  total_apis: number;
  categories: CategoryCount[];
  last_synced: string | null;
  sync_commit: string | null;
}

export interface CategoryCount {
  category: string;
  count: number;
}

export interface ApiToolDefinition {
  id: string;
  catalog_id: string;
  tool_name: string;
  tool_description: string;
  endpoint: string;
  method: string;
  parameters: string | null;
  auth_required: boolean;
  enabled: boolean;
  category: string;
  created_at: string;
  updated_at: string;
}

// ── API Credentials ──

export interface ApiCredential {
  id: string;
  provider_id: string;
  credential_type: string;
  credential_key: string;
  credential_value_encrypted: string;
  label: string | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface ApiCredentialTest {
  success: boolean;
  message: string;
  response_time_ms: number | null;
}

// ── Activity ──

export interface ActivityEvent {
  id: string;
  event_type: string;
  category: string;
  title: string;
  detail: string | null;
  status: string;
  metadata: string | null;
  created_at: string;
}

// ── App State ──

export type Page =
  | "home"
  | "chat"
  | "settings"
  | "models"
  | "memory"
  | "permissions"
  | "activity"
  | "api-catalog"
  | "api-credentials";

export type SettingsPage = "general" | "providers" | "catalog" | "voice";

export type AssistantState =
  | "idle"
  | "listening"
  | "thinking"
  | "working"
  | "searching"
  | "composing"
  | "speaking"
  | "error";

// ── Update ──

export type UpdateStatus =
  | "idle"
  | "checking"
  | "update_available"
  | "up_to_date"
  | "offline"
  | "error";

export interface UpdateCheckResult {
  status: UpdateStatus;
  current_version: string;
  latest_version: string | null;
  download_url: string | null;
  notes: string | null;
}

export interface ToolExecution {
  id: string;
  tool_id: string;
  status: "pending" | "running" | "success" | "error";
  detail: string;
  timestamp: string;
}
