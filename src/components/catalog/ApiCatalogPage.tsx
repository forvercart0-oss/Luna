import { useState, useEffect } from "react";
import { useCatalogStore } from "../../lib/stores";

const PUBLIC_APIS_README_URL =
  "https://raw.githubusercontent.com/public-apis/public-apis/master/README.md";

const AUTH_FILTERS = [
  { value: "", label: "All" },
  { value: "none", label: "No Auth" },
  { value: "apiKey", label: "API Key" },
  { value: "oauth", label: "OAuth" },
  { value: "bearer", label: "Bearer" },
];

export function ApiCatalogPage() {
  const {
    entries,
    stats,
    categories,
    loading,
    syncing,
    searchQuery,
    filterCategory,
    filterAuth,
    setSearchQuery,
    setFilterCategory,
    setFilterAuth,
    sync,
    search,
    loadStats,
    loadCategories,
    toggleEntry,
  } = useCatalogStore();

  const [showSyncResult, setShowSyncResult] = useState<string | null>(null);
  const [showCategories, setShowCategories] = useState(false);

  useEffect(() => {
    if (entries.length === 0 && !stats) {
      loadStats();
      loadCategories();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleSync = async () => {
    try {
      const response = await fetch(PUBLIC_APIS_README_URL);
      if (!response.ok) throw new Error(`Failed to fetch: ${response.status}`);
      const content = await response.text();
      const result = await sync(content);
      setShowSyncResult(
        `Synced: ${result.added} added, ${result.updated} updated, ${result.removed} removed. Total: ${result.total}`
      );
      setTimeout(() => setShowSyncResult(null), 5000);
    } catch (e) {
      setShowSyncResult(`Sync failed: ${String(e)}`);
      setTimeout(() => setShowSyncResult(null), 5000);
    }
  };

  const handleSearch = async () => {
    await search(searchQuery, filterCategory || undefined, filterAuth || undefined);
  };

  useEffect(() => {
    const timeout = setTimeout(() => {
      if (searchQuery || filterCategory || filterAuth) {
        search(searchQuery, filterCategory || undefined, filterAuth || undefined);
      }
    }, 300);
    return () => clearTimeout(timeout);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchQuery, filterCategory, filterAuth]);

  return (
    <>
      <div className="page-header">
        <h1 className="page-title">API Catalog</h1>
        <div style={{ display: "flex", gap: 8 }}>
          <button className="btn" onClick={handleSearch}>
            Search
          </button>
          <button
            className="btn btn-primary"
            onClick={handleSync}
            disabled={syncing}
          >
            {syncing ? "Syncing..." : "Sync from public-apis"}
          </button>
        </div>
      </div>
      <div className="page-body">
        {showSyncResult && (
          <div
            className={`toast ${showSyncResult.includes("failed") ? "error" : "success"}`}
            style={{ position: "sticky", top: 0, marginBottom: 16 }}
          >
            {showSyncResult}
          </div>
        )}

        {stats && (
          <div className="stats-grid">
            <div className="stat-card">
              <div className="stat-value">{stats.total_apis}</div>
              <div className="stat-label">APIs in Catalog</div>
            </div>
            <div className="stat-card">
              <div className="stat-value">{stats.categories.length}</div>
              <div className="stat-label">Categories</div>
            </div>
            <div className="stat-card">
              <div className="stat-value">
                {stats.last_synced
                  ? new Date(stats.last_synced).toLocaleDateString()
                  : "Never"}
              </div>
              <div className="stat-label">Last Synced</div>
            </div>
          </div>
        )}

        <div className="search-bar">
          <input
            className="search-input"
            placeholder="Search APIs..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
            style={{ flex: 1 }}
          />
        </div>

        {/* Category filter chips */}
        <div style={{ marginBottom: 12 }}>
          <div style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 8 }}>
            <button
              className="filter-chip"
              onClick={() => setShowCategories(!showCategories)}
              style={{ fontWeight: 500 }}
            >
              Category: {filterCategory || "All"} {showCategories ? "▴" : "▾"}
            </button>
            {filterCategory && (
              <button
                className="filter-chip"
                onClick={() => setFilterCategory("")}
                style={{ color: "var(--danger)" }}
              >
                ✕ Clear
              </button>
            )}
          </div>
          {showCategories && (
            <div className="filter-chips">
              <button
                className={`filter-chip ${!filterCategory ? "active" : ""}`}
                onClick={() => { setFilterCategory(""); setShowCategories(false); }}
              >
                All
              </button>
              {categories.map((cat) => (
                <button
                  key={cat}
                  className={`filter-chip ${filterCategory === cat ? "active" : ""}`}
                  onClick={() => { setFilterCategory(cat); setShowCategories(false); }}
                >
                  {cat}
                </button>
              ))}
            </div>
          )}
        </div>

        {/* Auth type filter chips */}
        <div style={{ marginBottom: 16 }}>
          <div className="filter-chips">
            {AUTH_FILTERS.map((af) => (
              <button
                key={af.value}
                className={`filter-chip ${filterAuth === af.value ? "active" : ""}`}
                onClick={() => setFilterAuth(af.value)}
              >
                {af.label}
              </button>
            ))}
          </div>
        </div>

        {loading && (
          <div className="empty-state">
            <div className="spinner" />
            <div className="empty-text">Searching...</div>
          </div>
        )}

        {!loading && entries.length === 0 && (
          <div className="empty-state">
            <div className="empty-icon">📡</div>
            <div className="empty-text">
              {stats && stats.total_apis > 0
                ? "Use the search bar to find APIs, or click Sync to update the catalog."
                : "No APIs in catalog yet. Click 'Sync from public-apis' to import the catalog."}
            </div>
          </div>
        )}

        <div className="catalog-grid">
          {entries.map((entry) => (
            <div key={entry.id} className="catalog-card">
              <div className="catalog-card-header">
                <div className="catalog-card-name">{entry.name}</div>
                <button
                  className={`toggle ${entry.enabled ? "active" : ""}`}
                  onClick={() => toggleEntry(entry.id, !entry.enabled)}
                  title={entry.enabled ? "Disable" : "Enable"}
                />
              </div>
              <div className="catalog-card-desc">{entry.description}</div>
              <div className="catalog-card-meta">
                <span className="badge badge-muted">{entry.category}</span>
                {entry.auth_type !== "none" && (
                  <span className="badge badge-warning">{entry.auth_type}</span>
                )}
                {entry.auth_type === "none" && (
                  <span className="badge badge-success">No Auth</span>
                )}
                {entry.https && <span className="badge badge-info">HTTPS</span>}
                <span className="badge badge-muted">{entry.cors}</span>
              </div>
              {entry.homepage && (
                <a
                  href={entry.homepage}
                  target="_blank"
                  rel="noopener noreferrer"
                  style={{
                    display: "block",
                    marginTop: 8,
                    fontSize: 12,
                    color: "var(--accent)",
                    textDecoration: "none",
                  }}
                >
                  Visit Homepage →
                </a>
              )}
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
