import { useState } from "react";
import { useCredentialStore } from "../../lib/stores";

const CREDENTIAL_TYPES = [
  { value: "api_key", label: "API Key" },
  { value: "bearer_token", label: "Bearer Token" },
  { value: "oauth", label: "OAuth Token" },
  { value: "basic", label: "Basic Auth" },
];

const PROVIDERS = [
  { value: "openrouter", label: "OpenRouter" },
  { value: "openai", label: "OpenAI" },
  { value: "anthropic", label: "Anthropic" },
  { value: "google", label: "Google" },
  { value: "github", label: "GitHub" },
  { value: "weather", label: "Weather API" },
  { value: "maps", label: "Maps API" },
  { value: "custom", label: "Custom" },
];

export function ApiCredentialsPage() {
  const { credentials, create, remove, setActive, test } = useCredentialStore();
  const [showCreate, setShowCreate] = useState(false);
  const [editing, setEditing] = useState<string | null>(null);
  const [testResult, setTestResult] = useState<{
    id: string;
    success: boolean;
    message: string;
  } | null>(null);
  const [form, setForm] = useState({
    provider_id: "openrouter",
    credential_type: "api_key",
    credential_key: "",
    credential_value: "",
    label: "",
  });

  const resetForm = () => {
    setForm({
      provider_id: "openrouter",
      credential_type: "api_key",
      credential_key: "",
      credential_value: "",
      label: "",
    });
  };

  const handleCreate = async () => {
    if (!form.credential_key.trim() || !form.credential_value.trim()) return;
    await create(
      form.provider_id,
      form.credential_type,
      form.credential_key.trim(),
      form.credential_value.trim(),
      form.label.trim() || undefined
    );
    resetForm();
    setShowCreate(false);
  };

  const handleTest = async (id: string) => {
    const result = await test(id);
    setTestResult({ id, ...result });
    setTimeout(() => setTestResult(null), 5000);
  };

  return (
    <>
      <div className="page-header">
        <h1 className="page-title">API Credentials</h1>
        <button
          className="btn btn-primary"
          onClick={() => {
            resetForm();
            setEditing(null);
            setShowCreate(true);
          }}
        >
          + Add Credential
        </button>
      </div>
      <div className="page-body">
        <div className="card" style={{ marginBottom: 16 }}>
          <div className="card-title">Credential Manager</div>
          <div className="card-subtitle" style={{ marginTop: 4 }}>
            Store API keys and tokens for different providers. Credentials are
            obfuscated locally and never sent to third parties.
          </div>
        </div>

        {credentials.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">🔑</div>
            <div className="empty-text">
              No credentials stored yet. Add API keys for services you want to
              use.
            </div>
          </div>
        )}

        {credentials.map((cred) => (
          <div key={cred.id} className="card">
            <div className="card-header">
              <div>
                <div className="card-title">
                  {cred.label || cred.credential_key}
                  {cred.is_active && (
                    <span
                      className="badge badge-success"
                      style={{ marginLeft: 8 }}
                    >
                      Active
                    </span>
                  )}
                </div>
                <div className="card-subtitle">
                  {cred.provider_id} · {cred.credential_type} · Created{" "}
                  {new Date(cred.created_at).toLocaleDateString()}
                </div>
              </div>
              <div className="card-actions">
                {testResult?.id === cred.id && (
                  <span
                    className={`badge ${testResult.success ? "badge-success" : "badge-danger"}`}
                  >
                    {testResult.message}
                  </span>
                )}
                {!cred.is_active && (
                  <button
                    className="btn btn-sm"
                    onClick={() => setActive(cred.id)}
                  >
                    Set Active
                  </button>
                )}
                <button
                  className="btn btn-sm"
                  onClick={() => handleTest(cred.id)}
                >
                  Test
                </button>
                <button
                  className="btn btn-danger btn-sm"
                  onClick={() => remove(cred.id)}
                >
                  Delete
                </button>
              </div>
            </div>
            <div style={{ fontSize: 12, color: "var(--text-muted)" }}>
              Key: {cred.credential_key}
            </div>
          </div>
        ))}

        {showCreate && (
          <div className="modal-overlay" onClick={() => setShowCreate(false)}>
            <div className="modal" onClick={(e) => e.stopPropagation()}>
              <div className="modal-title">
                {editing ? "Edit Credential" : "Add Credential"}
              </div>
              <div className="form-group">
                <label className="form-label">Provider</label>
                <select
                  className="form-select"
                  value={form.provider_id}
                  onChange={(e) =>
                    setForm({ ...form, provider_id: e.target.value })
                  }
                >
                  {PROVIDERS.map((p) => (
                    <option key={p.value} value={p.value}>
                      {p.label}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label className="form-label">Credential Type</label>
                <select
                  className="form-select"
                  value={form.credential_type}
                  onChange={(e) =>
                    setForm({ ...form, credential_type: e.target.value })
                  }
                >
                  {CREDENTIAL_TYPES.map((t) => (
                    <option key={t.value} value={t.value}>
                      {t.label}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label className="form-label">Key Name / Identifier</label>
                <input
                  className="form-input"
                  value={form.credential_key}
                  onChange={(e) =>
                    setForm({ ...form, credential_key: e.target.value })
                  }
                  placeholder="e.g. api_key, authorization"
                />
              </div>
              <div className="form-group">
                <label className="form-label">Value</label>
                <input
                  className="form-input"
                  type="password"
                  value={form.credential_value}
                  onChange={(e) =>
                    setForm({ ...form, credential_value: e.target.value })
                  }
                  placeholder="Enter the credential value"
                />
              </div>
              <div className="form-group">
                <label className="form-label">Label (optional)</label>
                <input
                  className="form-input"
                  value={form.label}
                  onChange={(e) =>
                    setForm({ ...form, label: e.target.value })
                  }
                  placeholder="e.g. Personal, Work"
                />
              </div>
              <div className="modal-actions">
                <button
                  className="btn"
                  onClick={() => setShowCreate(false)}
                >
                  Cancel
                </button>
                <button className="btn btn-primary" onClick={handleCreate}>
                  {editing ? "Save" : "Add"}
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </>
  );
}
