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
      </div>

      <div className="topbar-center">
        <div className="topbar-model">
          Active model: <strong>{activeProfile?.name || "Not configured"}</strong>
        </div>
      </div>

      <div className="topbar-right" />
    </div>
  );
}
