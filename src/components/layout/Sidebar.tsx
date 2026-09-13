import type { Page } from "../../lib/types";
import { OrbStatus } from "../orb/OrbStatus";

const NAV_ITEMS: { id: Page; label: string; icon: string }[] = [
  { id: "home", label: "Home", icon: "🏠" },
  { id: "chat", label: "Conversations", icon: "💬" },
  { id: "memory", label: "Memory", icon: "🧠" },
  { id: "activity", label: "Activity", icon: "📊" },
  { id: "permissions", label: "Tools", icon: "🔧" },
  { id: "api-catalog", label: "API Catalog", icon: "🌐" },
  { id: "settings", label: "Settings", icon: "⚙️" },
];

interface SidebarProps {
  page: Page;
  onNavigate: (page: Page) => void;
}

export function Sidebar({ page, onNavigate }: SidebarProps) {
  return (
    <aside className="sidebar">
      <nav className="sidebar-nav">
        {NAV_ITEMS.map((item) => (
          <button
            key={item.id}
            className={`nav-item ${page === item.id ? "active" : ""}`}
            onClick={() => onNavigate(item.id)}
            title={item.label}
          >
            <span className="nav-icon">{item.icon}</span>
            <span className="nav-item-label">{item.label}</span>
          </button>
        ))}
      </nav>
      <div className="sidebar-bottom">
        <OrbStatus />
      </div>
    </aside>
  );
}
