import { useState, useRef, useEffect } from "react";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";
import { OrbStatus } from "../orb/OrbStatus";
import { useConversationStore, useModelStore, useActivityStore, useMemoryStore, useAssistantStore } from "../../lib/stores";
import type { Page } from "../../lib/types";

interface HomeScreenProps {
  onNavigate: (page: Page) => void;
}

function SystemStats() {
  return (
    <div className="panel-section">
      <div className="panel-header">
        <div className="panel-title">SYSTEM</div>
        <div className="panel-toggle">▾</div>
      </div>
      <div className="system-stat">
        <div className="system-stat-label"><span className="system-stat-icon">⚡</span> CPU</div>
        <div className="system-stat-value">—</div>
      </div>
      <div className="system-stat">
        <div className="system-stat-label"><span className="system-stat-icon">💾</span> Memory</div>
        <div className="system-stat-value">—</div>
      </div>
      <div className="system-stat">
        <div className="system-stat-label"><span className="system-stat-icon">💿</span> Storage</div>
        <div className="system-stat-value">—</div>
      </div>
      <div className="system-stat">
        <div className="system-stat-label"><span className="system-stat-icon">📶</span> Network</div>
        <div className="system-stat-value">—</div>
      </div>
      <div className="system-stat">
        <div className="system-stat-label"><span className="system-stat-icon">🔋</span> Battery</div>
        <div className="system-stat-value">—</div>
      </div>
      <div className="system-stat">
        <div className="system-stat-label"><span className="system-stat-icon">🎤</span> Microphone</div>
        <div className="system-stat-value">—</div>
      </div>
    </div>
  );
}

function MemoryPanel() {
  const { memories } = useMemoryStore();
  const recent = memories.slice(0, 3);

  return (
    <div className="panel-section">
      <div className="panel-header">
        <div className="panel-title">MEMORY</div>
        <div className="panel-toggle">▾</div>
      </div>
      {recent.length === 0 ? (
        <div style={{ fontSize: 12, color: "var(--text-muted)" }}>No memories yet</div>
      ) : (
        recent.map((m) => (
          <div key={m.id} className="memory-item">
            <div className="memory-label">{m.type}</div>
            <div className="memory-value">{m.content.substring(0, 60)}{m.content.length > 60 ? "..." : ""}</div>
          </div>
        ))
      )}
    </div>
  );
}

function ActivityTimeline() {
  const { events } = useActivityStore();
  const recent = events.slice(0, 8);

  const dotClass = (type: string) => {
    if (type.includes("listen")) return "listening";
    if (type.includes("think")) return "thinking";
    if (type.includes("tool")) return "tool";
    if (type.includes("api")) return "api";
    if (type.includes("memory")) return "memory";
    if (type.includes("error")) return "error";
    return "completed";
  };

  return (
    <div className="panel-section">
      <div className="panel-header">
        <div className="panel-title">ACTIVITY</div>
      </div>
      <div className="activity-list">
        {recent.length === 0 && (
          <div style={{ fontSize: 12, color: "var(--text-muted)" }}>No activity yet</div>
        )}
        {recent.map((ev) => (
          <div key={ev.id} className="activity-item">
            <div className={`activity-dot ${dotClass(ev.event_type)}`} />
            <div className="activity-content">
              <div className="activity-time">
                {new Date(ev.created_at).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" })}
              </div>
              <div className={`activity-type ${dotClass(ev.event_type)}`}>
                {ev.event_type.replace(/_/g, " ").toUpperCase()}
              </div>
              <div className="activity-desc">{ev.title}</div>
            </div>
            <div className={`activity-badge ${ev.status === "completed" ? "active" : "idle"}`} />
          </div>
        ))}
      </div>
    </div>
  );
}

export function HomeScreen({ onNavigate: _onNavigate }: HomeScreenProps) {
  const {
    activeConversation,
    messages,
    sending,
    create,
    sendMessage,
  } = useConversationStore();
  const { profiles } = useModelStore();
  const { state: assistantState } = useAssistantStore();
  const [input, setInput] = useState("");
  const [selectedModel, setSelectedModel] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  useEffect(() => {
    const active = profiles.find((p) => p.is_active);
    if (active) setSelectedModel(active.id);
  }, [profiles]);

  const handleSend = async () => {
    const text = input.trim();
    if (!text || sending) return;
    setInput("");
    if (textareaRef.current) textareaRef.current.style.height = "auto";

    if (!activeConversation) {
      await create();
    }
    await sendMessage(text, selectedModel || undefined);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const handleTextareaInput = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setInput(e.target.value);
    const el = e.target;
    el.style.height = "auto";
    el.style.height = Math.min(el.scrollHeight, 80) + "px";
  };

  const stateLabel = assistantState.toUpperCase();
  const activeProfile = profiles.find((p) => p.is_active);

  return (
    <div className="home-screen">
      {/* Left Panel */}
      <div className="left-panel">
        <SystemStats />
        <MemoryPanel />
      </div>

      {/* Center: Orb + Chat */}
      <div className="center-area">
        <div className="orb-section">
          <div className="orb-wrapper">
            <OrbStatus />
            <div className="orb-label">LUNA</div>
            <div className="orb-status-text">ONLINE</div>
            <div className="orb-state-text">{stateLabel}</div>
          </div>
        </div>

        {/* Chat Messages */}
        {activeConversation && messages.length > 0 && (
          <div className="chat-area">
            {messages.map((msg) => (
              <div key={msg.id} className={`message-row ${msg.role === "user" ? "user" : "assistant"}`}>
                {msg.role === "user" && (
                  <>
                    <div className="message-tag user-tag">
                      {msg.content.length > 30 ? msg.content.substring(0, 30) + "..." : msg.content}
                    </div>
                    <div className="message-bubble">{msg.content}</div>
                  </>
                )}
                {msg.role === "assistant" && (
                  <>
                    <div className="message-bubble">
                      <ReactMarkdown
                        components={{
                          code({ className, children, ...props }) {
                            const match = /language-(\w+)/.exec(className || "");
                            const codeStr = String(children).replace(/\n$/, "");
                            if (match) {
                              return (
                                <SyntaxHighlighter
                                  style={oneDark}
                                  language={match[1]}
                                  PreTag="div"
                                  customStyle={{ margin: "6px 0", borderRadius: "6px", fontSize: "12px" }}
                                >
                                  {codeStr}
                                </SyntaxHighlighter>
                              );
                            }
                            return (
                              <code className={className} style={{ background: "var(--bg-hover)", padding: "2px 5px", borderRadius: "3px", fontSize: "12px" }} {...props}>
                                {children}
                              </code>
                            );
                          },
                        }}
                      >
                        {msg.content}
                      </ReactMarkdown>
                    </div>
                    <div className="message-tag assistant-tag">
                      {profiles.find((p) => p.id === msg.model_profile_id)?.name || "LUNA"}
                    </div>
                  </>
                )}
              </div>
            ))}
            {sending && (
              <div className="message-row assistant">
                <div className="message-bubble"><div className="spinner" /></div>
              </div>
            )}
            <div ref={messagesEndRef} />
          </div>
        )}

        {/* Composer */}
        <div className="composer">
          <div className="composer-input-row">
            <textarea
              ref={textareaRef}
              className="composer-input"
              value={input}
              onChange={handleTextareaInput}
              onKeyDown={handleKeyDown}
              placeholder="Ask LUNA anything..."
              rows={1}
            />
            <button className="composer-btn" title="Microphone">🎤</button>
            <button className="composer-btn" title="Attach">📎</button>
            <button
              className="composer-send"
              onClick={handleSend}
              disabled={!input.trim() || sending}
              title="Send"
            >
              ▶
            </button>
          </div>
          <div className="composer-status">
            <div className="composer-status-item">
              {activeProfile?.name || "No model"} | Voice: Kokoro | Ready
            </div>
          </div>
        </div>
      </div>

      {/* Right Panel */}
      <div className="right-panel">
        <ActivityTimeline />
      </div>
    </div>
  );
}
