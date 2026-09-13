import { usePermissionStore, useToolStore } from "../../lib/stores";

const PERMISSION_STATES = ["allowed", "denied", "ask"];

export function PermissionsPage() {
  const { permissions, update } = usePermissionStore();
  const { tools } = useToolStore();

  const handlePermissionChange = async (toolId: string, state: string) => {
    await update(toolId, state);
  };

  return (
    <>
      <div className="page-header">
        <h1 className="page-title">Permissions</h1>
      </div>
      <div className="page-body">
        <div className="card" style={{ marginBottom: 16 }}>
          <div className="card-title">Tool Permissions</div>
          <div className="card-subtitle" style={{ marginTop: 4 }}>
            Control which tools LUNA can use. "Ask" means LUNA will request
            confirmation before using a tool.
          </div>
        </div>

        {tools.map((tool) => {
          const existing = permissions.find((p) => p.tool_id === tool.id);
          const currentPerm = existing?.permission || "ask";

          return (
            <div key={tool.id} className="card">
              <div className="card-header">
                <div>
                  <div className="card-title">{tool.name}</div>
                  <div className="card-subtitle">
                    {tool.id} · {tool.category}
                  </div>
                  <div
                    style={{
                      fontSize: 12,
                      color: "var(--text-secondary)",
                      marginTop: 4,
                    }}
                  >
                    {tool.description}
                  </div>
                </div>
                <div style={{ display: "flex", gap: 6 }}>
                  {PERMISSION_STATES.map((state) => (
                    <button
                      key={state}
                      className={`btn btn-sm ${
                        currentPerm === state ? "btn-primary" : ""
                      }`}
                      onClick={() => handlePermissionChange(tool.id, state)}
                    >
                      {state === "allowed" && "✓ Allow"}
                      {state === "denied" && "✕ Deny"}
                      {state === "ask" && "? Ask"}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          );
        })}

        {tools.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">🔒</div>
            <div className="empty-text">No tools registered.</div>
          </div>
        )}
      </div>
    </>
  );
}
