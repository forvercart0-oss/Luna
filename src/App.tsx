import { useEffect, useState } from "react";
import { Sidebar } from "./components/layout/Sidebar";
import { TopBar } from "./components/layout/TopBar";
import { HomeScreen } from "./components/home/HomeScreen";
import { ChatPage } from "./components/chat/ChatPage";
import { SettingsPage } from "./components/settings/SettingsPage";
import { ModelsPage } from "./components/models/ModelsPage";
import { MemoryPage } from "./components/memory/MemoryPage";
import { PermissionsPage } from "./components/permissions/PermissionsPage";
import { ActivityPage } from "./components/activity/ActivityPage";
import { ApiCatalogPage } from "./components/catalog/ApiCatalogPage";
import { ApiCredentialsPage } from "./components/credentials/ApiCredentialsPage";
import { useSoundEffects } from "./lib/hooks/useSoundEffects";
import {
  useSettingsStore,
  useProviderStore,
  useModelStore,
  useConversationStore,
  useMemoryStore,
  usePermissionStore,
  useAuditStore,
  useToolStore,
  useCatalogStore,
  useCredentialStore,
  useActivityStore,
} from "./lib/stores";
import type { Page } from "./lib/types";

function App() {
  const [page, setPage] = useState<Page>("home");
  useSoundEffects();

  const loadSettings = useSettingsStore((s) => s.load);
  const loadProviders = useProviderStore((s) => s.load);
  const loadModels = useModelStore((s) => s.load);
  const loadConversations = useConversationStore((s) => s.loadConversations);
  const loadMemories = useMemoryStore((s) => s.load);
  const loadPermissions = usePermissionStore((s) => s.load);
  const loadAudit = useAuditStore((s) => s.load);
  const loadTools = useToolStore((s) => s.load);
  const loadStats = useCatalogStore((s) => s.loadStats);
  const loadCategories = useCatalogStore((s) => s.loadCategories);
  const loadCredentials = useCredentialStore((s) => s.load);
  const loadActivity = useActivityStore((s) => s.load);

  useEffect(() => {
    loadSettings();
    loadProviders();
    loadModels();
    loadConversations();
    loadMemories();
    loadPermissions();
    loadAudit();
    loadTools();
    loadStats();
    loadCategories();
    loadCredentials();
    loadActivity();
  }, []);

  const renderPage = () => {
    switch (page) {
      case "home":
        return <HomeScreen onNavigate={setPage} />;
      case "chat":
        return <ChatPage />;
      case "settings":
        return <SettingsPage />;
      case "models":
        return <ModelsPage />;
      case "memory":
        return <MemoryPage />;
      case "permissions":
        return <PermissionsPage />;
      case "activity":
        return <ActivityPage />;
      case "api-catalog":
        return <ApiCatalogPage />;
      case "api-credentials":
        return <ApiCredentialsPage />;
      default:
        return <HomeScreen onNavigate={setPage} />;
    }
  };

  return (
    <div className="app">
      <TopBar />
      <div className="app-body">
        <Sidebar page={page} onNavigate={setPage} />
        <div className="content-area">{renderPage()}</div>
      </div>
    </div>
  );
}

export default App;
