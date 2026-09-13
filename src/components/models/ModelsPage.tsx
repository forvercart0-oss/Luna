import { useState } from "react";
import { useModelStore } from "../../lib/stores";
import { ConfirmDialog } from "../ui/ConfirmDialog";

export function ModelsPage() {
  const { profiles, create, update, remove, setActive, duplicate } = useModelStore();
  const [showCreate, setShowCreate] = useState(false);
  const [editing, setEditing] = useState<string | null>(null);
  const [deleteId, setDeleteId] = useState<string | null>(null);
  const [form, setForm] = useState({
    name: "",
    provider: "openrouter",
    model_id: "",
    system_prompt: "",
    temperature: "0.7",
    max_tokens: "4096",
  });

  const resetForm = () => {
    setForm({
      name: "",
      provider: "openrouter",
      model_id: "",
      system_prompt: "",
      temperature: "0.7",
      max_tokens: "4096",
    });
  };

  const handleCreate = async () => {
    if (!form.name.trim() || !form.model_id.trim()) return;
    await create(
      form.name.trim(),
      form.provider,
      form.model_id.trim(),
      form.system_prompt.trim() || undefined,
      parseFloat(form.temperature) || 0.7,
      parseInt(form.max_tokens) || 4096
    );
    resetForm();
    setShowCreate(false);
  };

  const handleUpdate = async () => {
    if (!editing) return;
    await update(editing, {
      name: form.name.trim() || undefined,
      model_id: form.model_id.trim() || undefined,
      system_prompt: form.system_prompt.trim() || undefined,
      temperature: parseFloat(form.temperature) || undefined,
      max_tokens: parseInt(form.max_tokens) || undefined,
    } as never);
    setEditing(null);
    resetForm();
  };

  const startEdit = (id: string) => {
    const p = profiles.find((m) => m.id === id);
    if (!p) return;
    setForm({
      name: p.name,
      provider: p.provider,
      model_id: p.model_id,
      system_prompt: p.system_prompt || "",
      temperature: p.temperature.toString(),
      max_tokens: p.max_tokens.toString(),
    });
    setEditing(id);
    setShowCreate(true);
  };

  return (
    <>
      <div className="page-header">
        <h1 className="page-title">Models</h1>
        <button
          className="btn btn-primary"
          onClick={() => {
            resetForm();
            setEditing(null);
            setShowCreate(true);
          }}
        >
          + Add Model
        </button>
      </div>
      <div className="page-body">
        {profiles.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">🤖</div>
            <div className="empty-text">
              No model profiles configured. Add a model to start chatting.
            </div>
          </div>
        )}

        {profiles.map((p) => (
          <div key={p.id} className="card">
            <div className="card-header">
              <div>
                <div className="card-title">
                  {p.name}
                  {p.is_active && (
                    <span className="badge badge-success" style={{ marginLeft: 8 }}>
                      Active
                    </span>
                  )}
                  {!p.enabled && (
                    <span className="badge badge-muted" style={{ marginLeft: 8 }}>
                      Disabled
                    </span>
                  )}
                </div>
                <div className="card-subtitle">
                  {p.model_id} · temp={p.temperature} · max_tokens={p.max_tokens}
                </div>
              </div>
              <div className="card-actions">
                {!p.is_active && (
                  <button className="btn btn-sm" onClick={() => setActive(p.id)}>
                    Set Active
                  </button>
                )}
                <button className="btn btn-sm" onClick={() => startEdit(p.id)}>
                  Edit
                </button>
                <button className="btn btn-sm" onClick={() => duplicate(p.id)}>
                  Duplicate
                </button>
                <button className="btn btn-danger btn-sm" onClick={() => setDeleteId(p.id)}>
                  Delete
                </button>
              </div>
            </div>
            {p.system_prompt && (
              <div
                style={{
                  fontSize: 12,
                  color: "var(--text-muted)",
                  whiteSpace: "pre-wrap",
                  marginTop: 4,
                }}
              >
                {p.system_prompt.substring(0, 200)}
                {p.system_prompt.length > 200 ? "..." : ""}
              </div>
            )}
          </div>
        ))}

        {showCreate && (
          <div className="modal-overlay" onClick={() => setShowCreate(false)}>
            <div className="modal" onClick={(e) => e.stopPropagation()}>
              <div className="modal-title">
                {editing ? "Edit Model Profile" : "Add Model Profile"}
              </div>
              <div className="form-group">
                <label className="form-label">Profile Name</label>
                <input
                  className="form-input"
                  value={form.name}
                  onChange={(e) => setForm({ ...form, name: e.target.value })}
                  placeholder="e.g. Luna Main, Luna Coder"
                />
              </div>
              <div className="form-group">
                <label className="form-label">OpenRouter Model ID</label>
                <input
                  className="form-input"
                  value={form.model_id}
                  onChange={(e) => setForm({ ...form, model_id: e.target.value })}
                  placeholder="e.g. openai/gpt-4o, anthropic/claude-3.5-sonnet"
                />
              </div>
              <div className="form-group">
                <label className="form-label">Temperature</label>
                <input
                  className="form-input"
                  type="number"
                  step="0.1"
                  min="0"
                  max="2"
                  value={form.temperature}
                  onChange={(e) => setForm({ ...form, temperature: e.target.value })}
                />
              </div>
              <div className="form-group">
                <label className="form-label">Max Tokens</label>
                <input
                  className="form-input"
                  type="number"
                  min="256"
                  max="128000"
                  value={form.max_tokens}
                  onChange={(e) => setForm({ ...form, max_tokens: e.target.value })}
                />
              </div>
              <div className="form-group">
                <label className="form-label">System Prompt</label>
                <textarea
                  className="form-input"
                  value={form.system_prompt}
                  onChange={(e) => setForm({ ...form, system_prompt: e.target.value })}
                  placeholder="Optional system prompt for this model profile"
                />
              </div>
              <div className="modal-actions">
                <button className="btn" onClick={() => setShowCreate(false)}>
                  Cancel
                </button>
                <button
                  className="btn btn-primary"
                  onClick={editing ? handleUpdate : handleCreate}
                >
                  {editing ? "Save" : "Add"}
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

      <ConfirmDialog
        open={deleteId !== null}
        title="Delete Model Profile"
        message="This will permanently delete this model profile. This action cannot be undone."
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
    </>
  );
}
