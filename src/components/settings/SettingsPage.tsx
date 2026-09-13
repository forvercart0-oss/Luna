import { useState } from "react";
import { useSettingsStore, useProviderStore } from "../../lib/stores";
import type { Settings } from "../../lib/types";

type SettingsTab = "general" | "providers" | "voice";

export function SettingsPage() {
  const { settings, update } = useSettingsStore();
  const { accounts, add, remove, setActive } = useProviderStore();
  const [tab, setTab] = useState<SettingsTab>("general");
  const [showAddKey, setShowAddKey] = useState(false);
  const [newKeyName, setNewKeyName] = useState("");
  const [newKeyValue, setNewKeyValue] = useState("");
  const [newKeyUrl, setNewKeyUrl] = useState("");

  if (!settings) {
    return (
      <div className="page-header">
        <h1 className="page-title">Settings</h1>
      </div>
    );
  }

  const handleSaveGeneral = async (field: keyof Settings["general"], value: string) => {
    await update({
      ...settings,
      general: { ...settings.general, [field]: value },
    });
  };

  const handleSaveAi = async (field: keyof Settings["ai"], value: boolean | number | string) => {
    await update({
      ...settings,
      ai: { ...settings.ai, [field]: value },
    });
  };

  const handleSaveVoice = async (field: keyof Settings["voice"], value: boolean | number | string) => {
    await update({
      ...settings,
      voice: { ...settings.voice, [field]: value },
    });
  };

  const handleAddKey = async () => {
    if (!newKeyName.trim() || !newKeyValue.trim()) return;
    await add(newKeyName.trim(), newKeyValue.trim(), newKeyUrl.trim() || undefined);
    setNewKeyName("");
    setNewKeyValue("");
    setNewKeyUrl("");
    setShowAddKey(false);
  };

  return (
    <div className="settings-layout">
      <nav className="settings-nav">
        <button
          className={`settings-nav-item ${tab === "general" ? "active" : ""}`}
          onClick={() => setTab("general")}
        >
          General
        </button>
        <button
          className={`settings-nav-item ${tab === "providers" ? "active" : ""}`}
          onClick={() => setTab("providers")}
        >
          Providers
        </button>
        <button
          className={`settings-nav-item ${tab === "voice" ? "active" : ""}`}
          onClick={() => setTab("voice")}
        >
          Voice
        </button>
      </nav>

      <div className="settings-content">
        {tab === "general" && (
          <>
            <h2 style={{ fontSize: 18, fontWeight: 600, marginBottom: 20 }}>General</h2>
            <div className="card">
              <div className="form-group">
                <label className="form-label">Theme</label>
                <select
                  className="form-select"
                  value={settings.general.theme}
                  onChange={(e) => handleSaveGeneral("theme", e.target.value)}
                >
                  <option value="dark">Dark</option>
                  <option value="light">Light</option>
                  <option value="system">System</option>
                </select>
              </div>
              <div className="form-group">
                <label className="form-label">Language</label>
                <select
                  className="form-select"
                  value={settings.general.language}
                  onChange={(e) => handleSaveGeneral("language", e.target.value)}
                >
                  <option value="en">English</option>
                  <option value="ur">Urdu</option>
                </select>
              </div>
            </div>

            <div className="card">
              <div className="card-title" style={{ marginBottom: 12 }}>AI Behavior</div>
              <div className="form-group">
                <label className="form-label">Default Provider</label>
                <select
                  className="form-select"
                  value={settings.ai.default_provider}
                  onChange={(e) => handleSaveAi("default_provider", e.target.value)}
                >
                  <option value="openrouter">OpenRouter</option>
                </select>
              </div>
              <div className="form-group">
                <label className="form-label">Auto-save conversations</label>
                <button
                  className={`toggle ${settings.ai.auto_save_conversations ? "active" : ""}`}
                  onClick={() => handleSaveAi("auto_save_conversations", !settings.ai.auto_save_conversations)}
                />
              </div>
              <div className="form-group">
                <label className="form-label">Memory enabled</label>
                <button
                  className={`toggle ${settings.ai.memory_enabled ? "active" : ""}`}
                  onClick={() => handleSaveAi("memory_enabled", !settings.ai.memory_enabled)}
                />
              </div>
              <div className="form-group">
                <label className="form-label">Max context messages</label>
                <input
                  type="number"
                  className="form-input"
                  value={settings.ai.max_context_messages}
                  onChange={(e) => handleSaveAi("max_context_messages", parseInt(e.target.value) || 20)}
                  min={1}
                  max={100}
                />
              </div>
            </div>
          </>
        )}

        {tab === "providers" && (
          <>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 20 }}>
              <h2 style={{ fontSize: 18, fontWeight: 600 }}>Providers</h2>
              <button className="btn btn-primary btn-sm" onClick={() => setShowAddKey(true)}>
                + Add Key
              </button>
            </div>
            <div className="card" style={{ marginBottom: 16 }}>
              <div className="card-title">OpenRouter API Keys</div>
              <div className="card-subtitle" style={{ marginTop: 4 }}>
                Manage your OpenRouter API keys. Keys are stored locally.
              </div>
            </div>
            {accounts.length === 0 && (
              <div className="empty-state">
                <div className="empty-icon">🔑</div>
                <div className="empty-text">
                  No API keys configured. Add your OpenRouter API key to get started.
                </div>
              </div>
            )}
            {accounts.map((acc) => (
              <div key={acc.id} className="card">
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
                  <div>
                    <div style={{ fontWeight: 500 }}>{acc.name}</div>
                    <div style={{ fontSize: 12, color: "var(--text-muted)" }}>
                      {acc.api_key_masked}
                      {acc.base_url && <span> · {acc.base_url}</span>}
                    </div>
                  </div>
                  <div className="card-actions">
                    {!acc.is_active && (
                      <button className="btn btn-sm" onClick={() => setActive(acc.id)}>
                        Set Active
                      </button>
                    )}
                    {acc.is_active && <span className="badge badge-success">Active</span>}
                    <button className="btn btn-danger btn-sm" onClick={() => remove(acc.id)}>
                      Delete
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </>
        )}

        {tab === "voice" && (
          <>
            <h2 style={{ fontSize: 18, fontWeight: 600, marginBottom: 20 }}>Voice</h2>
            <div className="card">
              <div className="card-title" style={{ marginBottom: 12 }}>Text-to-Speech</div>
              <div className="form-group">
                <label className="form-label">TTS Enabled</label>
                <button
                  className={`toggle ${settings.voice.tts_enabled ? "active" : ""}`}
                  onClick={() => handleSaveVoice("tts_enabled", !settings.voice.tts_enabled)}
                />
              </div>
              <div className="form-group">
                <label className="form-label">Voice</label>
                <input
                  className="form-input"
                  value={settings.voice.voice}
                  onChange={(e) => handleSaveVoice("voice", e.target.value)}
                />
              </div>
              <div className="form-group">
                <label className="form-label">Speed: {settings.voice.speed.toFixed(1)}</label>
                <input
                  type="range"
                  min="0.5"
                  max="2.0"
                  step="0.1"
                  value={settings.voice.speed}
                  onChange={(e) => handleSaveVoice("speed", parseFloat(e.target.value))}
                  style={{ width: "100%" }}
                />
              </div>
              <div className="form-group">
                <label className="form-label">Volume: {settings.voice.volume.toFixed(1)}</label>
                <input
                  type="range"
                  min="0.0"
                  max="1.0"
                  step="0.1"
                  value={settings.voice.volume}
                  onChange={(e) => handleSaveVoice("volume", parseFloat(e.target.value))}
                  style={{ width: "100%" }}
                />
              </div>
            </div>

            <div className="card">
              <div className="card-title" style={{ marginBottom: 12 }}>Speech-to-Text</div>
              <div className="form-group">
                <label className="form-label">STT Enabled</label>
                <button
                  className={`toggle ${settings.voice.stt_enabled ? "active" : ""}`}
                  onClick={() => handleSaveVoice("stt_enabled", !settings.voice.stt_enabled)}
                />
              </div>
            </div>
          </>
        )}
      </div>

      {showAddKey && (
        <div className="modal-overlay" onClick={() => setShowAddKey(false)}>
          <div className="modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-title">Add API Key</div>
            <div className="form-group">
              <label className="form-label">Profile Name</label>
              <input
                className="form-input"
                value={newKeyName}
                onChange={(e) => setNewKeyName(e.target.value)}
                placeholder="e.g. Personal, Work"
              />
            </div>
            <div className="form-group">
              <label className="form-label">API Key</label>
              <input
                className="form-input"
                type="password"
                value={newKeyValue}
                onChange={(e) => setNewKeyValue(e.target.value)}
                placeholder="sk-or-..."
              />
            </div>
            <div className="form-group">
              <label className="form-label">Base URL (optional)</label>
              <input
                className="form-input"
                value={newKeyUrl}
                onChange={(e) => setNewKeyUrl(e.target.value)}
                placeholder="https://openrouter.ai/api/v1"
              />
            </div>
            <div className="modal-actions">
              <button className="btn" onClick={() => setShowAddKey(false)}>
                Cancel
              </button>
              <button className="btn btn-primary" onClick={handleAddKey}>
                Add
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
