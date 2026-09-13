import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
  Settings,
  ProviderAccount,
  ModelProfile,
  Conversation,
  Message,
  Memory,
  Permission,
  AuditEntry,
  ToolDefinition,
  ApiCatalogEntry,
  CatalogSyncResult,
  CatalogStats,
  ApiToolDefinition,
  ApiCredential,
  ApiCredentialTest,
  ActivityEvent,
  AssistantState,
  ToolExecution,
} from "../types";

import { create } from "zustand";

// ── Settings Store ──

interface SettingsState {
  settings: Settings | null;
  loading: boolean;
  load: () => Promise<void>;
  update: (settings: Settings) => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: null,
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const settings = await invoke<Settings>("get_settings");
      set({ settings, loading: false });
    } catch (e) {
      console.error("Failed to load settings:", e);
      set({ loading: false });
    }
  },
  update: async (settings) => {
    try {
      const updated = await invoke<Settings>("update_settings", { settings });
      set({ settings: updated });
    } catch (e) {
      console.error("Failed to update settings:", e);
    }
  },
}));

// ── Provider Store ──

interface ProviderState {
  accounts: ProviderAccount[];
  loading: boolean;
  load: () => Promise<void>;
  add: (name: string, apiKey: string, baseUrl?: string) => Promise<void>;
  remove: (id: string) => Promise<void>;
  setActive: (id: string) => Promise<void>;
}

export const useProviderStore = create<ProviderState>((set, get) => ({
  accounts: [],
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const accounts = await invoke<ProviderAccount[]>("get_provider_accounts");
      set({ accounts, loading: false });
    } catch (e) {
      console.error("Failed to load providers:", e);
      set({ loading: false });
    }
  },
  add: async (name, apiKey, baseUrl) => {
    try {
      await invoke("add_provider_account", {
        name,
        apiKey,
        baseUrl: baseUrl || null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to add provider:", e);
      throw e;
    }
  },
  remove: async (id) => {
    try {
      await invoke("delete_provider_account", { id });
      await get().load();
    } catch (e) {
      console.error("Failed to remove provider:", e);
    }
  },
  setActive: async (id) => {
    try {
      await invoke("update_provider_account", {
        id,
        isActive: true,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to set active provider:", e);
    }
  },
}));

// ── Model Store ──

interface ModelState {
  profiles: ModelProfile[];
  activeProfile: ModelProfile | null;
  loading: boolean;
  load: () => Promise<void>;
  create: (
    name: string,
    provider: string,
    modelId: string,
    systemPrompt?: string,
    temperature?: number,
    maxTokens?: number
  ) => Promise<void>;
  update: (id: string, changes: Partial<ModelProfile>) => Promise<void>;
  remove: (id: string) => Promise<void>;
  setActive: (id: string) => Promise<void>;
}

export const useModelStore = create<ModelState>((set, get) => ({
  profiles: [],
  activeProfile: null,
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const profiles = await invoke<ModelProfile[]>("get_model_profiles");
      const active = await invoke<ModelProfile | null>("get_active_model_profile");
      set({ profiles, activeProfile: active, loading: false });
    } catch (e) {
      console.error("Failed to load models:", e);
      set({ loading: false });
    }
  },
  create: async (name, provider, modelId, systemPrompt, temperature, maxTokens) => {
    try {
      await invoke("create_model_profile", {
        name,
        provider,
        modelId,
        systemPrompt: systemPrompt || null,
        temperature: temperature ?? null,
        maxTokens: maxTokens ?? null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to create model:", e);
      throw e;
    }
  },
  update: async (id, changes) => {
    try {
      await invoke("update_model_profile", {
        id,
        name: changes.name ?? null,
        provider: changes.provider ?? null,
        modelId: changes.model_id ?? null,
        systemPrompt: changes.system_prompt ?? null,
        temperature: changes.temperature ?? null,
        maxTokens: changes.max_tokens ?? null,
        enabled: changes.enabled ?? null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to update model:", e);
    }
  },
  remove: async (id) => {
    try {
      await invoke("delete_model_profile", { id });
      await get().load();
    } catch (e) {
      console.error("Failed to delete model:", e);
    }
  },
  setActive: async (id) => {
    try {
      await invoke("set_active_model_profile", { id });
      await get().load();
    } catch (e) {
      console.error("Failed to set active model:", e);
    }
  },
}));

// ── Conversation Store (with streaming support) ──

interface ConversationState {
  conversations: Conversation[];
  activeConversation: string | null;
  messages: Message[];
  loading: boolean;
  sending: boolean;
  streamingContent: string;
  loadConversations: () => Promise<void>;
  loadMessages: (conversationId: string) => Promise<void>;
  create: (title?: string) => Promise<string>;
  remove: (id: string) => Promise<void>;
  rename: (id: string, title: string) => Promise<void>;
  setActive: (id: string | null) => void;
  sendMessage: (content: string, modelProfileId?: string) => Promise<void>;
  cancelGeneration: () => Promise<void>;
  cleanup: () => void;
}

export const useConversationStore = create<ConversationState>((set, get) => {
  let unlistenStream: (() => void) | null = null;
  let unlistenMessage: (() => void) | null = null;

  const setupListeners = async () => {
    if (unlistenStream) return;

    unlistenStream = await listen<{ conversation_id: string; content: string; delta: string }>(
      "chat:stream-chunk",
      (event) => {
        const { conversation_id, content } = event.payload;
        if (conversation_id === get().activeConversation) {
          set({ streamingContent: content });
        }
      }
    );

    unlistenMessage = await listen<Message>("chat:assistant-message", (event) => {
      const msg = event.payload;
      if (msg.conversation_id === get().activeConversation) {
        set((state) => ({
          messages: [...state.messages.filter((m) => m.id !== msg.id), msg],
          streamingContent: "",
          sending: false,
        }));
      }
    });
  };

  return {
    conversations: [],
    activeConversation: null,
    messages: [],
    loading: false,
    sending: false,
    streamingContent: "",
    loadConversations: async () => {
      set({ loading: true });
      try {
        const conversations = await invoke<Conversation[]>("get_conversations");
        set({ conversations, loading: false });
      } catch (e) {
        console.error("Failed to load conversations:", e);
        set({ loading: false });
      }
    },
    loadMessages: async (conversationId) => {
      try {
        const messages = await invoke<Message[]>("get_messages", { conversationId });
        set({ messages });
      } catch (e) {
        console.error("Failed to load messages:", e);
      }
    },
    create: async (title) => {
      try {
        const conv = await invoke<Conversation>("create_conversation", {
          title: title || null,
          modelProfileId: null,
        });
        await get().loadConversations();
        set({ activeConversation: conv.id, messages: [] });
        return conv.id;
      } catch (e) {
        console.error("Failed to create conversation:", e);
        throw e;
      }
    },
    remove: async (id) => {
      try {
        await invoke("delete_conversation", { id });
        if (get().activeConversation === id) {
          set({ activeConversation: null, messages: [] });
        }
        await get().loadConversations();
      } catch (e) {
        console.error("Failed to delete conversation:", e);
      }
    },
    rename: async (id, title) => {
      try {
        await invoke("rename_conversation", { id, title });
        await get().loadConversations();
      } catch (e) {
        console.error("Failed to rename conversation:", e);
      }
    },
    setActive: (id) => {
      set({ activeConversation: id });
      if (id) {
        setupListeners();
        get().loadMessages(id);
      } else {
        set({ messages: [] });
      }
    },
    sendMessage: async (content, modelProfileId) => {
      const { activeConversation } = get();
      if (!activeConversation) return;

      set({ sending: true });

      const userMsg: Message = {
        id: crypto.randomUUID(),
        conversation_id: activeConversation,
        role: "user",
        content,
        model_profile_id: modelProfileId || null,
        status: "complete",
        metadata: null,
        created_at: new Date().toISOString(),
      };

      set((state) => ({ messages: [...state.messages, userMsg] }));

      try {
        await invoke("send_message", {
          conversationId: activeConversation,
          content,
          modelProfileId: modelProfileId || null,
        });
        await get().loadConversations();
      } catch (e) {
        console.error("Failed to send message:", e);
        const errorMsg: Message = {
          id: crypto.randomUUID(),
          conversation_id: activeConversation,
          role: "assistant",
          content: `Error: ${String(e)}`,
          model_profile_id: null,
          status: "error",
          metadata: null,
          created_at: new Date().toISOString(),
        };
        set((state) => ({ messages: [...state.messages, errorMsg], sending: false }));
      }
    },
    cancelGeneration: async () => {
      const { activeConversation } = get();
      if (!activeConversation) return;
      try {
        await invoke("cancel_generation", { conversationId: activeConversation });
        set({ sending: false, streamingContent: "" });
      } catch (e) {
        console.error("Failed to cancel generation:", e);
      }
    },
    cleanup: () => {
      unlistenStream?.();
      unlistenMessage?.();
      unlistenStream = null;
      unlistenMessage = null;
    },
  };
});

// ── Memory Store ──

interface MemoryState {
  memories: Memory[];
  loading: boolean;
  load: () => Promise<void>;
  create: (type: string, content: string, importance?: number, source?: string, tags?: string) => Promise<void>;
  update: (id: string, changes: Partial<Memory>) => Promise<void>;
  remove: (id: string) => Promise<void>;
  search: (query: string) => Promise<Memory[]>;
}

export const useMemoryStore = create<MemoryState>((set, get) => ({
  memories: [],
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const memories = await invoke<Memory[]>("get_memories");
      set({ memories, loading: false });
    } catch (e) {
      console.error("Failed to load memories:", e);
      set({ loading: false });
    }
  },
  create: async (type, content, importance, source, tags) => {
    try {
      await invoke("create_memory", {
        type,
        content,
        importance: importance ?? null,
        source: source || null,
        tags: tags || null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to create memory:", e);
      throw e;
    }
  },
  update: async (id, changes) => {
    try {
      await invoke("update_memory", {
        id,
        type: changes.type ?? null,
        content: changes.content ?? null,
        importance: changes.importance ?? null,
        source: changes.source ?? null,
        tags: changes.tags ?? null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to update memory:", e);
    }
  },
  remove: async (id) => {
    try {
      await invoke("delete_memory", { id });
      await get().load();
    } catch (e) {
      console.error("Failed to delete memory:", e);
    }
  },
  search: async (query) => {
    try {
      return await invoke<Memory[]>("search_memories", { query });
    } catch (e) {
      console.error("Failed to search memories:", e);
      return [];
    }
  },
}));

// ── Permission Store ──

interface PermissionState {
  permissions: Permission[];
  loading: boolean;
  load: () => Promise<void>;
  update: (toolId: string, permission: string, scope?: string) => Promise<void>;
}

export const usePermissionStore = create<PermissionState>((set, get) => ({
  permissions: [],
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const permissions = await invoke<Permission[]>("get_permissions");
      set({ permissions, loading: false });
    } catch (e) {
      console.error("Failed to load permissions:", e);
      set({ loading: false });
    }
  },
  update: async (toolId, permission, scope) => {
    try {
      await invoke("update_permission", {
        toolId,
        permission,
        scope: scope || null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to update permission:", e);
    }
  },
}));

// ── Audit Store ──

interface AuditState {
  logs: AuditEntry[];
  loading: boolean;
  load: (limit?: number) => Promise<void>;
}

export const useAuditStore = create<AuditState>((set) => ({
  logs: [],
  loading: false,
  load: async (limit) => {
    set({ loading: true });
    try {
      const logs = await invoke<AuditEntry[]>("get_audit_logs", { limit: limit || null });
      set({ logs, loading: false });
    } catch (e) {
      console.error("Failed to load audit logs:", e);
      set({ loading: false });
    }
  },
}));

// ── Tool Store ──

interface ToolState {
  tools: ToolDefinition[];
  loading: boolean;
  load: () => Promise<void>;
}

export const useToolStore = create<ToolState>((set) => ({
  tools: [],
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const tools = await invoke<ToolDefinition[]>("get_tool_list");
      set({ tools, loading: false });
    } catch (e) {
      console.error("Failed to load tools:", e);
      set({ loading: false });
    }
  },
}));

// ── API Catalog Store ──

interface CatalogState {
  entries: ApiCatalogEntry[];
  stats: CatalogStats | null;
  categories: string[];
  loading: boolean;
  syncing: boolean;
  searchQuery: string;
  filterCategory: string;
  filterAuth: string;
  setSearchQuery: (q: string) => void;
  setFilterCategory: (c: string) => void;
  setFilterAuth: (a: string) => void;
  sync: (readmeContent: string) => Promise<CatalogSyncResult>;
  search: (query: string, category?: string, authFilter?: string) => Promise<void>;
  loadStats: () => Promise<void>;
  loadCategories: () => Promise<void>;
  toggleEntry: (id: string, enabled: boolean) => Promise<void>;
  createApiTool: (catalogId: string, toolName: string, description: string, endpoint: string, method: string, params?: string, authRequired?: boolean, category?: string) => Promise<void>;
  deleteApiTool: (id: string) => Promise<void>;
  loadApiTools: () => Promise<ApiToolDefinition[]>;
  apiTools: ApiToolDefinition[];
}

export const useCatalogStore = create<CatalogState>((set, get) => ({
  entries: [],
  stats: null,
  categories: [],
  loading: false,
  syncing: false,
  searchQuery: "",
  filterCategory: "",
  filterAuth: "",
  apiTools: [],
  setSearchQuery: (q) => set({ searchQuery: q }),
  setFilterCategory: (c) => set({ filterCategory: c }),
  setFilterAuth: (a) => set({ filterAuth: a }),
  sync: async (readmeContent) => {
    set({ syncing: true });
    try {
      const result = await invoke<CatalogSyncResult>("sync_catalog", { readmeContent });
      set({ syncing: false });
      await get().loadStats();
      await get().loadCategories();
      return result;
    } catch (e) {
      set({ syncing: false });
      throw e;
    }
  },
  search: async (query, category, authFilter) => {
    set({ loading: true });
    try {
      const entries = await invoke<ApiCatalogEntry[]>("search_catalog", {
        query,
        category: category || null,
        authFilter: authFilter || null,
      });
      set({ entries, loading: false });
    } catch (e) {
      console.error("Failed to search catalog:", e);
      set({ loading: false });
    }
  },
  loadStats: async () => {
    try {
      const stats = await invoke<CatalogStats>("get_catalog_stats");
      set({ stats });
    } catch (e) {
      console.error("Failed to load catalog stats:", e);
    }
  },
  loadCategories: async () => {
    try {
      const categories = await invoke<string[]>("get_categories");
      set({ categories });
    } catch (e) {
      console.error("Failed to load categories:", e);
    }
  },
  toggleEntry: async (id, enabled) => {
    try {
      await invoke("toggle_catalog_entry", { id, enabled });
      set((state) => ({
        entries: state.entries.map((e) =>
          e.id === id ? { ...e, enabled } : e
        ),
      }));
    } catch (e) {
      console.error("Failed to toggle entry:", e);
    }
  },
  createApiTool: async (catalogId, toolName, description, endpoint, method, params, authRequired, category) => {
    try {
      await invoke("create_api_tool", {
        catalogId,
        toolName,
        toolDescription: description,
        endpoint,
        method,
        parameters: params || null,
        authRequired: authRequired ?? false,
        category: category || "",
      });
      await get().loadApiTools();
    } catch (e) {
      console.error("Failed to create API tool:", e);
      throw e;
    }
  },
  deleteApiTool: async (id) => {
    try {
      await invoke("delete_api_tool", { id });
      await get().loadApiTools();
    } catch (e) {
      console.error("Failed to delete API tool:", e);
    }
  },
  loadApiTools: async () => {
    try {
      const tools = await invoke<ApiToolDefinition[]>("get_api_tools", { enabledOnly: false });
      set({ apiTools: tools });
      return tools;
    } catch (e) {
      console.error("Failed to load API tools:", e);
      return [];
    }
  },
}));

// ── API Credentials Store ──

interface CredentialState {
  credentials: ApiCredential[];
  loading: boolean;
  load: () => Promise<void>;
  create: (providerId: string, credentialType: string, credentialKey: string, credentialValue: string, label?: string) => Promise<void>;
  remove: (id: string) => Promise<void>;
  setActive: (id: string) => Promise<void>;
  test: (id: string, testUrl?: string) => Promise<ApiCredentialTest>;
  update: (id: string, label?: string, credentialValue?: string) => Promise<void>;
}

export const useCredentialStore = create<CredentialState>((set, get) => ({
  credentials: [],
  loading: false,
  load: async () => {
    set({ loading: true });
    try {
      const credentials = await invoke<ApiCredential[]>("get_credentials");
      set({ credentials, loading: false });
    } catch (e) {
      console.error("Failed to load credentials:", e);
      set({ loading: false });
    }
  },
  create: async (providerId, credentialType, credentialKey, credentialValue, label) => {
    try {
      await invoke("create_credential", {
        providerId,
        credentialType,
        credentialKey,
        credentialValue,
        label: label || null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to create credential:", e);
      throw e;
    }
  },
  remove: async (id) => {
    try {
      await invoke("delete_credential", { id });
      await get().load();
    } catch (e) {
      console.error("Failed to delete credential:", e);
    }
  },
  setActive: async (id) => {
    try {
      await invoke("set_active_credential", { id });
      await get().load();
    } catch (e) {
      console.error("Failed to set active credential:", e);
    }
  },
  test: async (id, testUrl) => {
    try {
      return await invoke<ApiCredentialTest>("test_credential", {
        id,
        testUrl: testUrl || null,
      });
    } catch (e) {
      return { success: false, message: String(e), response_time_ms: null };
    }
  },
  update: async (id, label, credentialValue) => {
    try {
      await invoke("update_credential", {
        id,
        label: label || null,
        credentialValue: credentialValue || null,
      });
      await get().load();
    } catch (e) {
      console.error("Failed to update credential:", e);
    }
  },
}));

// ── Activity Store ──

interface ActivityState {
  events: ActivityEvent[];
  loading: boolean;
  load: (limit?: number, eventType?: string) => Promise<void>;
  create: (eventType: string, category: string, title: string, detail?: string, status?: string) => Promise<ActivityEvent>;
  update: (id: string, status: string, detail?: string) => Promise<void>;
}

export const useActivityStore = create<ActivityState>((set) => ({
  events: [],
  loading: false,
  load: async (limit, eventType) => {
    set({ loading: true });
    try {
      const events = await invoke<ActivityEvent[]>("get_activity_events", {
        limit: limit || null,
        eventType: eventType || null,
      });
      set({ events, loading: false });
    } catch (e) {
      console.error("Failed to load activity events:", e);
      set({ loading: false });
    }
  },
  create: async (eventType, category, title, detail, status) => {
    try {
      const event = await invoke<ActivityEvent>("create_activity_event", {
        eventType,
        category,
        title,
        detail: detail || null,
        status: status || null,
        metadata: null,
      });
      set((state) => ({ events: [event, ...state.events] }));
      return event;
    } catch (e) {
      console.error("Failed to create activity event:", e);
      throw e;
    }
  },
  update: async (id, status, detail) => {
    try {
      await invoke("update_activity_event", {
        id,
        status,
        detail: detail || null,
      });
      set((state) => ({
        events: state.events.map((e) =>
          e.id === id ? { ...e, status, detail: detail || e.detail } : e
        ),
      }));
    } catch (e) {
      console.error("Failed to update activity event:", e);
    }
  },
}));

// ── Assistant State Store ──

interface AssistantStateStore {
  state: AssistantState;
  toolExecutions: ToolExecution[];
  setState: (s: AssistantState) => void;
  addToolExecution: (te: ToolExecution) => void;
  updateToolExecution: (id: string, updates: Partial<ToolExecution>) => void;
  clearToolExecutions: () => void;
  init: () => void;
}

export const useAssistantStore = create<AssistantStateStore>((set) => ({
  state: "idle",
  toolExecutions: [],
  setState: (s) => set({ state: s }),
  addToolExecution: (te) =>
    set((state) => ({ toolExecutions: [...state.toolExecutions, te] })),
  updateToolExecution: (id, updates) =>
    set((state) => ({
      toolExecutions: state.toolExecutions.map((te) =>
        te.id === id ? { ...te, ...updates } : te
      ),
    })),
  clearToolExecutions: () => set({ toolExecutions: [] }),
  init: () => {
    listen<string>("luna:state-change", (event) => {
      set({ state: event.payload as AssistantState });
    });
  },
}));

// ── System Stats Store ──

export interface SystemStats {
  cpu_usage: number | null;
  memory_used_mb: number | null;
  memory_total_mb: number | null;
  disk_used_gb: number | null;
  disk_total_gb: number | null;
  hostname: string | null;
  os: string | null;
}

interface SystemStatsState {
  stats: SystemStats | null;
  load: () => Promise<void>;
}

export const useSystemStatsStore = create<SystemStatsState>((set) => ({
  stats: null,
  load: async () => {
    try {
      const stats = await invoke<SystemStats>("get_system_stats");
      set({ stats });
    } catch (e) {
      console.error("Failed to load system stats:", e);
    }
  },
}));
