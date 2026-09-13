import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore, useProviderStore, useUpdateStore } from "../../lib/stores";
import { OptionSelect } from "../ui/OptionSelect";
import { ConfirmDialog } from "../ui/ConfirmDialog";
import type { Settings } from "../../lib/types";

type SettingsTab = "general" | "providers" | "voice" | "updates";

export function SettingsPage() {
  const { settings, update } = useSettingsStore();
  const { accounts, add, remove, setActive, test } = useProviderStore();
  const { status: updateStatus, currentVersion, latestVersion, check: checkUpdates } = useUpdateStore();
  const [tab, setTab] = useState<SettingsTab>("general");
  const [showAddKey, setShowAddKey] = useState(false);
  const [newKeyName, setNewKeyName] = useState("");
  const [newKeyValue, setNewKeyValue] = useState("");
  const [newKeyUrl, setNewKeyUrl] = useState("");
  const [deleteId, setDeleteId] = useState<string | null>(null);
  const [testResult, setTestResult] = useState<{ id: string; success: boolean; message: string } | null>(null);
  const [testingId, setTestingId] = useState<string | null>(null);
  const [voiceTestText, setVoiceTestText] = useState("Hello, I am LUNA.");
  const [voiceTesting, setVoiceTesting] = useState(false);

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

  const handleTestProvider = async (id: string) => {
    setTestingId(id);
    setTestResult(null);
    const result = await test(id);
    setTestResult({ id, ...result });
    setTestingId(null);
  };

  const handleTestVoice = async () => {
    setVoiceTesting(true);
    try {
      await invoke("tts_speak", {
        text: voiceTestText,
        voice: settings.voice.voice || undefined,
        speed: settings.voice.speed,
      });
    } catch (e) {
      setTestResult({ id: "voice", success: false, message: String(e) });
    }
    setVoiceTesting(false);
  };

  const updateStatusText: Record<string, string> = {
    idle: "Not checked yet",
    checking: "Checking...",
    update_available: `Update available: v${latestVersion}`,
    up_to_date: "Up to date",
    offline: "Offline — unable to check",
    error: "Check failed",
  };

  const languageOptions = [
    { value: "en", label: "English", icon: "🇬🇧" },
    { value: "es", label: "Spanish", icon: "🇪🇸" },
    { value: "fr", label: "French", icon: "🇫🇷" },
    { value: "de", label: "German", icon: "🇩🇪" },
    { value: "ja", label: "Japanese", icon: "🇯🇵" },
    { value: "zh", label: "Chinese", icon: "🇨🇳" },
  ];

  const providerOptions = [
    { value: "openrouter", label: "OpenRouter", description: "Multi-model gateway" },
  ];

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
        <button
          className={`settings-nav-item ${tab === "updates" ? "active" : ""}`}
          onClick={() => setTab("updates")}
        >
          Updates
        </button>
      </nav>

      <div className="settings-content">
        {tab === "general" && (
          <>
            <h2 style={{ fontSize: 18, fontWeight: 600, marginBottom: 20 }}>General</h2>
            <div className="card">
              <OptionSelect
                label="Language"
                options={languageOptions}
                value={settings.general.language}
                onChange={(v) => handleSaveGeneral("language", v)}
              />
            </div>

            <div className="card">
              <div className="card-title" style={{ marginBottom: 12 }}>AI Behavior</div>
              <div className="form-group">
                <OptionSelect
                  label="Default Provider"
                  options={providerOptions}
                  value={settings.ai.default_provider}
                  onChange={(v) => handleSaveAi("default_provider", v)}
                />
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
                Manage your OpenRouter API keys. Keys are stored locally and obfuscated.
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
                    <div style={{ fontWeight: 500 }}>
                      {acc.name}
                      {acc.is_active && (
                        <span className="badge badge-success" style={{ marginLeft: 8 }}>Active</span>
                      )}
                    </div>
                    <div style={{ fontSize: 12, color: "var(--text-muted)" }}>
                      {acc.api_key_masked}
                      {acc.base_url && <span> · {acc.base_url}</span>}
                    </div>
                  </div>
                  <div className="card-actions">
                    {testResult?.id === acc.id && (
                      <span className={`badge ${testResult.success ? "badge-success" : "badge-danger"}`}>
                        {testResult.message}
                      </span>
                    )}
                    {!acc.is_active && (
                      <button className="btn btn-sm" onClick={() => setActive(acc.id)}>
                        Set Active
                      </button>
                    )}
                    <button
                      className="btn btn-sm"
                      onClick={() => handleTestProvider(acc.id)}
                      disabled={testingId === acc.id}
                    >
                      {testingId === acc.id ? "Testing..." : "Test"}
                    </button>
                    <button className="btn btn-danger btn-sm" onClick={() => setDeleteId(acc.id)}>
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
                <label className="form-label">Voice Name</label>
                <input
                  className="form-input"
                  value={settings.voice.voice}
                  onChange={(e) => handleSaveVoice("voice", e.target.value)}
                  placeholder="e.g. default"
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
              <div className="form-group">
                <label className="form-label">Test Voice</label>
                <div style={{ display: "flex", gap: 8 }}>
                  <input
                    className="form-input"
                    style={{ flex: 1 }}
                    value={voiceTestText}
                    onChange={(e) => setVoiceTestText(e.target.value)}
                    placeholder="Text to speak..."
                  />
                  <button
                    className="btn btn-primary"
                    onClick={handleTestVoice}
                    disabled={voiceTesting || !settings.voice.tts_enabled}
                  >
                    {voiceTesting ? "Speaking..." : "🔊 Test"}
                  </button>
                </div>
                {testResult?.id === "voice" && (
                  <div style={{ marginTop: 8 }}>
                    <span className={`badge ${testResult.success ? "badge-success" : "badge-danger"}`}>
                      {testResult.message}
                    </span>
                  </div>
                )}
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

        {tab === "updates" && (
          <>
            <h2 style={{ fontSize: 18, fontWeight: 600, marginBottom: 20 }}>Updates</h2>
            <div className="card">
              <div className="form-group">
                <label className="form-label">Current Version</label>
                <div style={{ fontSize: 14, color: "var(--text-primary)" }}>
                  {currentVersion ? `v${currentVersion}` : "Unknown"}
                </div>
              </div>
              <div className="form-group">
                <label className="form-label">Update Status</label>
                <div style={{ fontSize: 14, color: updateStatus === "update_available" ? "var(--accent)" : "var(--text-primary)" }}>
                  {updateStatusText[updateStatus] || updateStatus}
                </div>
              </div>
              {updateStatus === "update_available" && latestVersion && (
                <div style={{ padding: 12, borderRadius: 8, background: "var(--accent-dim)", border: "1px solid var(--border-glow)", marginTop: 8 }}>
                  <div style={{ fontSize: 13, color: "var(--accent)", marginBottom: 4 }}>New version available: v{latestVersion}</div>
                  <div style={{ fontSize: 12, color: "var(--text-secondary)" }}>Run <code style={{ background: "var(--bg-hover)", padding: "2px 6px", borderRadius: 4 }}>luna update</code> to install</div>
                </div>
              )}
              <button
                className="btn btn-primary"
                onClick={checkUpdates}
                disabled={updateStatus === "checking"}
                style={{ marginTop: 12 }}
              >
                {updateStatus === "checking" ? "Checking..." : "Check for Updates"}
              </button>
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

      <ConfirmDialog
        open={deleteId !== null}
        title="Delete Provider Account"
        message="This will permanently remove this provider account and its API key. This action cannot be undone."
        confirmLabel="Delete"
        danger
        onConfirm={async () => {
          if (deleteId) {
            await remove(deleteId);
            setDeleteId(null);
          }
        }}
        onCancel={() => setDeleteId(null)}
      />
    </div>
  );
}
