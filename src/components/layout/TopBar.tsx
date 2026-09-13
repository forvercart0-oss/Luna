import { useModelStore } from "../../lib/stores";

export function TopBar() {
  const { activeProfile } = useModelStore();

  return (
    <div className="topbar">
      <div className="topbar-left">
        <div className="topbar-logo">
          <div className="topbar-logo-icon">L</div>
          <div className="topbar-logo-text">LUNA</div>
        </div>
        <div className="topbar-status">
          <div className="topbar-status-dot" />
          ONLINE
        </div>
      </div>

      <div className="topbar-center">
        <div className="topbar-model">
          Current AI model: <strong>{activeProfile?.name || "Not configured"}</strong>
        </div>
      </div>

      <div className="topbar-right">
        <button className="topbar-btn" title="Microphone">
          🎤
        </button>
        <button className="topbar-voice">
          🎵 Voice
          <span style={{ fontSize: 10 }}>▾</span>
        </button>
        <button className="topbar-btn" title="Settings">
          ⚙️
        </button>
        <div className="topbar-window">
          <button className="topbar-window-btn minimize" />
          <button className="topbar-window-btn maximize" />
          <button className="topbar-window-btn close" />
        </div>
      </div>
    </div>
  );
}
