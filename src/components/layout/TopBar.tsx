import { useState, useRef, useEffect } from "react";
import { useModelStore, useUpdateStore } from "../../lib/stores";

export function TopBar() {
  const { profiles, activeProfile } = useModelStore();
  const { currentVersion } = useUpdateStore();
  const [modelOpen, setModelOpen] = useState(false);
  const { setActive } = useModelStore();
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!modelOpen) return;
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setModelOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [modelOpen]);

  return (
    <div className="topbar">
      <div className="topbar-left">
        <div className="topbar-logo">
          <div className="topbar-logo-icon">L</div>
          <div className="topbar-logo-text">LUNA</div>
        </div>
      </div>

      <div className="topbar-center" ref={ref}>
        <div className="model-popover" style={{ position: "relative" }}>
          <button
            className="model-popover-trigger"
            onClick={() => setModelOpen(!modelOpen)}
            type="button"
          >
            <span style={{ color: "var(--text-secondary)", fontSize: 12 }}>
              Active model:
            </span>
            <strong style={{ fontSize: 12 }}>
              {activeProfile?.name || "Not configured"}
            </strong>
            <span style={{ fontSize: 10, color: "var(--text-muted)" }}>▾</span>
          </button>
          {modelOpen && (
            <div className="model-popover-dropdown">
              {profiles
                .filter((p) => p.enabled)
                .map((p) => (
                  <button
                    key={p.id}
                    className={`model-popover-item ${p.is_active ? "selected" : ""}`}
                    onClick={() => {
                      setActive(p.id);
                      setModelOpen(false);
                    }}
                    type="button"
                  >
                    <div>
                      <div className="model-popover-item-name">{p.name}</div>
                      <div className="model-popover-item-id">{p.model_id}</div>
                    </div>
                    {p.is_active && (
                      <span style={{ color: "var(--accent)" }}>✓</span>
                    )}
                  </button>
                ))}
              {profiles.filter((p) => p.enabled).length === 0 && (
                <div style={{ padding: 12, fontSize: 12, color: "var(--text-muted)", textAlign: "center" }}>
                  No models configured
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      <div className="topbar-right">
        {currentVersion && (
          <span style={{ fontSize: 11, color: "var(--text-muted)" }}>
            v{currentVersion}
          </span>
        )}
      </div>
    </div>
  );
}
