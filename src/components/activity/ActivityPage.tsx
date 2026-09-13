import { useActivityStore } from "../../lib/stores";

const EVENT_ICONS: Record<string, string> = {
  thinking: "🤔",
  tool_execution: "🔧",
  api_request: "📡",
  memory_retrieval: "🧠",
  memory_update: "💾",
  voice_generation: "🔊",
  error: "❌",
  chat_send: "💬",
  catalog_sync: "🔄",
};

export function ActivityPage() {
  const { events, load } = useActivityStore();

  return (
    <>
      <div className="page-header">
        <h1 className="page-title">Activity</h1>
        <button className="btn" onClick={() => load(200)}>
          Refresh
        </button>
      </div>
      <div className="page-body">
        {events.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">📋</div>
            <div className="empty-text">
              No activity logged yet. Events will appear here as you use LUNA.
            </div>
          </div>
        )}

        {events.length > 0 && (
          <div>
            {events.map((event) => (
              <div key={event.id} className="card" style={{ marginBottom: 8 }}>
                <div style={{ display: "flex", justifyContent: "space-between", alignItems: "flex-start" }}>
                  <div style={{ display: "flex", gap: 12, alignItems: "flex-start" }}>
                    <span style={{ fontSize: 18 }}>
                      {EVENT_ICONS[event.event_type] || "📌"}
                    </span>
                    <div>
                      <div style={{ fontWeight: 500, fontSize: 14 }}>{event.title}</div>
                      <div style={{ fontSize: 12, color: "var(--text-muted)", marginTop: 2 }}>
                        {event.category} · {new Date(event.created_at).toLocaleString()}
                      </div>
                      {event.detail && (
                        <div style={{ fontSize: 12, color: "var(--text-secondary)", marginTop: 4 }}>
                          {event.detail}
                        </div>
                      )}
                    </div>
                  </div>
                  <span
                    className={`badge ${
                      event.status === "completed"
                        ? "badge-success"
                        : event.status === "error"
                        ? "badge-danger"
                        : event.status === "running"
                        ? "badge-accent"
                        : "badge-muted"
                    }`}
                  >
                    {event.status}
                  </span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </>
  );
}
