import { useState } from "react";
import { useDesktopStore } from "../../lib/stores";
import type { ScreenshotResult, MonitorInfo } from "../../lib/types";

export function DesktopPage() {
  const {
    clipboardContent,
    clipboardHistory,
    screenInfo,
    loadClipboard,
    writeClipboard,
    loadClipboardHistory,
    sendNotification,
    loadScreenInfo,
    captureScreenshot,
    listDir,
  } = useDesktopStore();

  const [activeTab, setActiveTab] = useState<"clipboard" | "notifications" | "screen">("clipboard");
  const [writeText, setWriteText] = useState("");
  const [notifyTitle, setNotifyTitle] = useState("LUNA Notification");
  const [notifyBody, setNotifyBody] = useState("Hello from LUNA!");
  const [isWriting, setIsWriting] = useState(false);
  const [isNotifying, setIsNotifying] = useState(false);
  const [isCapturing, setIsCapturing] = useState(false);
  const [screenshot, setScreenshot] = useState<ScreenshotResult | null>(null);
  const [dirPath, setDirPath] = useState("");
  const [dirContents, setDirContents] = useState<string[]>([]);

  const handleWriteClipboard = async () => {
    if (!writeText.trim()) return;
    setIsWriting(true);
    await writeClipboard(writeText);
    setIsWriting(false);
    setWriteText("");
    await loadClipboard();
  };

  const handleSendNotification = async () => {
    setIsNotifying(true);
    await sendNotification(notifyTitle, notifyBody);
    setIsNotifying(false);
  };

  const handleCaptureScreenshot = async (monitorIndex?: number) => {
    setIsCapturing(true);
    const result = await captureScreenshot(monitorIndex);
    if (result) {
      setScreenshot(result);
    }
    setIsCapturing(false);
  };

  const handleListDir = async () => {
    if (!dirPath.trim()) return;
    const contents = await listDir(dirPath);
    setDirContents(contents);
  };

  const loadAll = () => {
    loadClipboard();
    loadClipboardHistory();
    loadScreenInfo();
  };

  if (!clipboardContent && !screenInfo && activeTab === "clipboard") {
    loadAll();
  }

  return (
    <>
      <div className="page-header">
        <h1 className="page-title">Desktop Tools</h1>
        <p style={{ color: "var(--text-secondary)", marginTop: 4 }}>
          Clipboard, notifications, screen capture, and filesystem utilities
        </p>
      </div>
      <div className="page-body">
        <div className="card" style={{ marginBottom: 16 }}>
          <div className="card-header">
            <nav style={{ display: "flex", gap: 8 }}>
              <button
                className={`btn btn-sm ${activeTab === "clipboard" ? "btn-primary" : ""}`}
                onClick={() => setActiveTab("clipboard")}
              >
                Clipboard
              </button>
              <button
                className={`btn btn-sm ${activeTab === "notifications" ? "btn-primary" : ""}`}
                onClick={() => setActiveTab("notifications")}
              >
                Notifications
              </button>
              <button
                className={`btn btn-sm ${activeTab === "screen" ? "btn-primary" : ""}`}
                onClick={() => setActiveTab("screen")}
              >
                Screen
              </button>
            </nav>
          </div>
        </div>

        {activeTab === "clipboard" && (
          <div className="card">
            <div className="card-title">Clipboard Reader</div>
            {clipboardContent !== null ? (
              <div style={{ marginTop: 12 }}>
                <div className="form-label">Current Clipboard Content</div>
                <div
                  style={{
                    background: "var(--bg-hover)",
                    borderRadius: 6,
                    padding: 12,
                    fontFamily: "monospace",
                    fontSize: 13,
                    whiteSpace: "pre-wrap",
                    maxHeight: 200,
                    overflow: "auto",
                  }}
                >
                  {clipboardContent || <span style={{ color: "var(--text-muted)" }}>Empty</span>}
                </div>
              </div>
            ) : (
              <button className="btn btn-sm" onClick={loadClipboard}>
                Load Clipboard
              </button>
            )}

            <div style={{ marginTop: 16 }}>
              <div className="card-subtitle">Recent History</div>
              {clipboardHistory.length === 0 ? (
                <div style={{ color: "var(--text-muted)", fontSize: 13, marginTop: 8 }}>
                  No history yet
                </div>
              ) : (
                <div style={{ marginTop: 8, display: "flex", flexDirection: "column", gap: 8 }}>
                  {clipboardHistory.map((item, i) => (
                    <div
                      key={i}
                      style={{
                        background: "var(--bg-hover)",
                        borderRadius: 6,
                        padding: 8,
                        fontSize: 12,
                        fontFamily: "monospace",
                        whiteSpace: "pre-wrap",
                        maxHeight: 80,
                        overflow: "hidden",
                      }}
                      title={item.content}
                    >
                      {item.content.substring(0, 100)}
                      {item.content.length > 100 && "..."}
                    </div>
                  ))}
                </div>
              )}
            </div>

            <div style={{ marginTop: 16 }}>
              <div className="form-group">
                <label className="form-label">Write to Clipboard</label>
                <textarea
                  className="form-input"
                  value={writeText}
                  onChange={(e) => setWriteText(e.target.value)}
                  placeholder="Enter text to copy to clipboard..."
                  rows={3}
                />
              </div>
              <button
                className="btn btn-primary"
                onClick={handleWriteClipboard}
                disabled={isWriting || !writeText.trim()}
              >
                {isWriting ? "Copying..." : "Copy to Clipboard"}
              </button>
            </div>
          </div>
        )}

        {activeTab === "notifications" && (
          <div className="card">
            <div className="card-title">Send Notification</div>
            <div className="form-group">
              <label className="form-label">Title</label>
              <input
                className="form-input"
                value={notifyTitle}
                onChange={(e) => setNotifyTitle(e.target.value)}
              />
            </div>
            <div className="form-group">
              <label className="form-label">Body</label>
              <textarea
                className="form-input"
                value={notifyBody}
                onChange={(e) => setNotifyBody(e.target.value)}
                rows={3}
              />
            </div>
            <button
              className="btn btn-primary"
              onClick={handleSendNotification}
              disabled={isNotifying}
            >
              {isNotifying ? "Sending..." : "Send Notification"}
            </button>
          </div>
        )}

        {activeTab === "screen" && (
          <div className="card">
            <div className="card-title">Screen Capture</div>

            {screenInfo && screenInfo.monitors.length > 0 && (
              <div style={{ marginTop: 12 }}>
                <div className="card-subtitle">Detected Monitors</div>
                <div style={{ display: "flex", gap: 8, flexWrap: "wrap", marginTop: 8 }}>
                  {screenInfo.monitors.map((mon: MonitorInfo, i: number) => (
                    <button
                      key={mon.name}
                      className="btn btn-sm"
                      onClick={() => handleCaptureScreenshot(i)}
                      disabled={isCapturing}
                    >
                      {mon.name} ({mon.width}x{mon.height})
                      {mon.primary && " ★"}
                    </button>
                  ))}
                </div>
                <div style={{ marginTop: 12 }}>
                  <button
                    className="btn btn-sm"
                    onClick={() => handleCaptureScreenshot(undefined)}
                    disabled={isCapturing}
                  >
                    {isCapturing ? "Capturing..." : "Capture All Screens"}
                  </button>
                </div>
              </div>
            )}

            {screenshot && (
              <div style={{ marginTop: 16 }}>
                <div className="card-subtitle">
                  Screenshot ({screenshot.width}x{screenshot.height})
                </div>
                <img
                  src={`data:image/png;base64,${screenshot.base64}`}
                  alt="Screenshot"
                  style={{
                    maxWidth: "100%",
                    borderRadius: 8,
                    border: "1px solid var(--border-color)",
                    marginTop: 8,
                  }}
                />
              </div>
            )}

            <div style={{ marginTop: 16, paddingTop: 16, borderTop: "1px solid var(--border-color)" }}>
              <div className="card-title">Filesystem Explorer</div>
              <div className="form-group">
                <input
                  className="form-input"
                  value={dirPath}
                  onChange={(e) => setDirPath(e.target.value)}
                  placeholder="Enter directory path (e.g. /home/user)"
                />
              </div>
              <button className="btn btn-sm" onClick={handleListDir}>
                List Directory
              </button>
              {dirContents.length > 0 && (
                <div style={{ marginTop: 12 }}>
                  <div className="card-subtitle">{dirContents.length} entries</div>
                  <div
                    style={{
                      background: "var(--bg-hover)",
                      borderRadius: 6,
                      padding: 12,
                      fontFamily: "monospace",
                      fontSize: 12,
                      whiteSpace: "pre-wrap",
                      maxHeight: 200,
                      overflow: "auto",
                      marginTop: 8,
                    }}
                  >
                    {dirContents.map((entry, i) => (
                      <div key={i}>{entry}</div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </>
  );
}
