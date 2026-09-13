import { useState } from "react";
import { useMemoryStore } from "../../lib/stores";

const MEMORY_TYPES = [
  "preference",
  "fact",
  "project",
  "instruction",
  "relationship",
  "task",
  "technical",
  "other",
];

export function MemoryPage() {
  const { memories, create, update, remove, search } = useMemoryStore();
  const [showCreate, setShowCreate] = useState(false);
  const [editing, setEditing] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<typeof memories | null>(null);
  const [form, setForm] = useState({
    type: "fact",
    content: "",
    importance: "5",
    source: "",
    tags: "",
  });

  const handleSearch = async () => {
    if (!searchQuery.trim()) {
      setSearchResults(null);
      return;
    }
    const results = await search(searchQuery.trim());
    setSearchResults(results);
  };

  const handleCreate = async () => {
    if (!form.content.trim()) return;
    await create(
      form.type,
      form.content.trim(),
      parseInt(form.importance) || 5,
      form.source.trim() || undefined,
      form.tags.trim() || undefined
    );
    setForm({ type: "fact", content: "", importance: "5", source: "", tags: "" });
    setShowCreate(false);
  };

  const handleUpdate = async () => {
    if (!editing) return;
    await update(editing, {
      type: form.type,
      content: form.content.trim(),
      importance: parseInt(form.importance) || 5,
      source: form.source.trim() || undefined,
      tags: form.tags.trim() || undefined,
    });
    setEditing(null);
    setForm({ type: "fact", content: "", importance: "5", source: "", tags: "" });
    setShowCreate(false);
  };

  const startEdit = (id: string) => {
    const m = memories.find((x) => x.id === id);
    if (!m) return;
    setForm({
      type: m.type,
      content: m.content,
      importance: m.importance.toString(),
      source: m.source || "",
      tags: m.tags || "",
    });
    setEditing(id);
    setShowCreate(true);
  };

  const displayMemories = searchResults || memories;

  return (
    <>
      <div className="page-header">
        <h1 className="page-title">Memory</h1>
        <button
          className="btn btn-primary"
          onClick={() => {
            setForm({ type: "fact", content: "", importance: "5", source: "", tags: "" });
            setEditing(null);
            setShowCreate(true);
          }}
        >
          + Add Memory
        </button>
      </div>
      <div className="page-body">
        {/* Search */}
        <div className="card">
          <div style={{ display: "flex", gap: 8 }}>
            <input
              className="form-input"
              style={{ flex: 1 }}
              placeholder="Search memories..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyDown={(e) => e.key === "Enter" && handleSearch()}
            />
            <button className="btn" onClick={handleSearch}>
              Search
            </button>
            {searchResults && (
              <button
                className="btn"
                onClick={() => {
                  setSearchResults(null);
                  setSearchQuery("");
                }}
              >
                Clear
              </button>
            )}
          </div>
        </div>

        {/* Memory list */}
        {displayMemories.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">🧠</div>
            <div className="empty-text">
              {searchResults
                ? "No memories match your search."
                : "No memories stored yet. Add important information to remember."}
            </div>
          </div>
        )}

        {displayMemories.map((m) => (
          <div key={m.id} className="card">
            <div className="card-header">
              <div>
                <div className="card-title">
                  <span className="badge badge-accent">{m.type}</span>
                  <span style={{ marginLeft: 8, fontSize: 12, color: "var(--text-muted)" }}>
                    Importance: {m.importance}/10
                  </span>
                </div>
              </div>
              <div className="card-actions">
                <button className="btn btn-sm" onClick={() => startEdit(m.id)}>
                  Edit
                </button>
                <button className="btn btn-danger btn-sm" onClick={() => remove(m.id)}>
                  Delete
                </button>
              </div>
            </div>
            <div style={{ fontSize: 14, lineHeight: 1.6, whiteSpace: "pre-wrap" }}>
              {m.content}
            </div>
            {(m.source || m.tags) && (
              <div style={{ marginTop: 8, fontSize: 12, color: "var(--text-muted)" }}>
                {m.source && <span>Source: {m.source} · </span>}
                {m.tags && <span>Tags: {m.tags}</span>}
              </div>
            )}
            <div style={{ marginTop: 4, fontSize: 11, color: "var(--text-muted)" }}>
              Created: {new Date(m.created_at).toLocaleDateString()}
            </div>
          </div>
        ))}

        {/* Create/Edit modal */}
        {showCreate && (
          <div className="modal-overlay" onClick={() => setShowCreate(false)}>
            <div className="modal" onClick={(e) => e.stopPropagation()}>
              <div className="modal-title">{editing ? "Edit Memory" : "Add Memory"}</div>
              <div className="form-group">
                <label className="form-label">Type</label>
                <select
                  className="form-select"
                  value={form.type}
                  onChange={(e) => setForm({ ...form, type: e.target.value })}
                >
                  {MEMORY_TYPES.map((t) => (
                    <option key={t} value={t}>
                      {t}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label className="form-label">Content</label>
                <textarea
                  className="form-input"
                  value={form.content}
                  onChange={(e) => setForm({ ...form, content: e.target.value })}
                  placeholder="What should LUNA remember?"
                />
              </div>
              <div className="form-group">
                <label className="form-label">Importance (1-10)</label>
                <input
                  className="form-input"
                  type="number"
                  min="1"
                  max="10"
                  value={form.importance}
                  onChange={(e) => setForm({ ...form, importance: e.target.value })}
                />
              </div>
              <div className="form-group">
                <label className="form-label">Source</label>
                <input
                  className="form-input"
                  value={form.source}
                  onChange={(e) => setForm({ ...form, source: e.target.value })}
                  placeholder="Where did this come from?"
                />
              </div>
              <div className="form-group">
                <label className="form-label">Tags</label>
                <input
                  className="form-input"
                  value={form.tags}
                  onChange={(e) => setForm({ ...form, tags: e.target.value })}
                  placeholder="comma, separated, tags"
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
    </>
  );
}
