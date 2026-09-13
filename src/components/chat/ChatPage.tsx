import { useState, useRef, useEffect } from "react";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";
import {
  useConversationStore,
  useModelStore,
  useAssistantStore,
} from "../../lib/stores";
import type { Message } from "../../lib/types";

function ToolExecutionCard({ detail, status }: { detail: string; status: string }) {
  return (
    <div className="tool-execution">
      <div className="tool-execution-header">
        <div className={`tool-status ${status}`} />
        <span>{detail}</span>
      </div>
    </div>
  );
}

function MessageBubble({ msg, profiles }: { msg: Message; profiles: { id: string; name: string }[] }) {
  if (msg.role === "user") {
    return (
      <div className="message message-user">
        <div className="message-bubble">{msg.content}</div>
      </div>
    );
  }

  if (msg.role === "assistant") {
    return (
      <div className="message message-assistant">
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
                      customStyle={{
                        margin: "8px 0",
                        borderRadius: "8px",
                        fontSize: "13px",
                      }}
                    >
                      {codeStr}
                    </SyntaxHighlighter>
                  );
                }
                return (
                  <code
                    className={className}
                    style={{
                      background: "var(--bg-hover)",
                      padding: "2px 6px",
                      borderRadius: "4px",
                      fontSize: "13px",
                    }}
                    {...props}
                  >
                    {children}
                  </code>
                );
              },
            }}
          >
            {msg.content}
          </ReactMarkdown>
        </div>
        <div className="message-meta">
          {msg.model_profile_id && (
            <span className="badge badge-accent">
              {profiles.find((p) => p.id === msg.model_profile_id)?.name || "Model"}
            </span>
          )}
          {new Date(msg.created_at).toLocaleTimeString()}
        </div>
      </div>
    );
  }

  if (msg.role === "system") {
    return (
      <div className="message message-system">
        <div className="message-bubble">{msg.content}</div>
      </div>
    );
  }

  return (
    <div className="message message-error">
      <div className="message-bubble">{msg.content}</div>
    </div>
  );
}

export function ChatPage() {
  const {
    conversations,
    activeConversation,
    messages,
    sending,
    streamingContent,
    create,
    remove,
    rename,
    setActive,
    sendMessage,
    cancelGeneration,
  } = useConversationStore();

  const { profiles } = useModelStore();
  const { toolExecutions, clearToolExecutions } = useAssistantStore();
  const [input, setInput] = useState("");
  const [selectedModel, setSelectedModel] = useState<string>("");
  const [renamingId, setRenamingId] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, toolExecutions, streamingContent]);

  useEffect(() => {
    const active = profiles.find((p) => p.is_active);
    if (active) setSelectedModel(active.id);
  }, [profiles]);

  const handleSend = async () => {
    const text = input.trim();
    if (!text || sending) return;
    setInput("");
    clearToolExecutions();
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
    }
    await sendMessage(text, selectedModel || undefined);
  };

  const handleCancel = async () => {
    await cancelGeneration();
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
    el.style.height = Math.min(el.scrollHeight, 120) + "px";
  };

  const handleNewConversation = async () => {
    await create();
  };

  const startRename = (id: string, currentTitle: string) => {
    setRenamingId(id);
    setRenameValue(currentTitle);
  };

  const commitRename = async () => {
    if (renamingId && renameValue.trim()) {
      await rename(renamingId, renameValue.trim());
    }
    setRenamingId(null);
    setRenameValue("");
  };

  return (
    <div className="chat-layout">
      <div className="conversation-sidebar">
        <div className="conv-header">
          <button
            className="btn btn-primary btn-sm"
            onClick={handleNewConversation}
          >
            + New
          </button>
        </div>
        <div className="conv-list">
          {conversations.length === 0 && (
            <div className="empty-state" style={{ padding: 24 }}>
              <p className="empty-text">No conversations yet</p>
            </div>
          )}
          {conversations.map((conv) => (
            <div
              key={conv.id}
              className={`conv-item ${activeConversation === conv.id ? "active" : ""}`}
              onClick={() => setActive(conv.id)}
            >
              {renamingId === conv.id ? (
                <input
                  className="form-input"
                  style={{ flex: 1, fontSize: 13, padding: "4px 8px" }}
                  value={renameValue}
                  onChange={(e) => setRenameValue(e.target.value)}
                  onBlur={commitRename}
                  onKeyDown={(e) => {
                    if (e.key === "Enter") commitRename();
                    if (e.key === "Escape") setRenamingId(null);
                  }}
                  autoFocus
                  onClick={(e) => e.stopPropagation()}
                />
              ) : (
                <span className="conv-title">{conv.title}</span>
              )}
              <div className="conv-actions">
                {renamingId !== conv.id && (
                  <button
                    className="conv-action-btn"
                    onClick={(e) => {
                      e.stopPropagation();
                      startRename(conv.id, conv.title);
                    }}
                    title="Rename"
                  >
                    ✏️
                  </button>
                )}
                <button
                  className="conv-action-btn danger"
                  onClick={(e) => {
                    e.stopPropagation();
                    remove(conv.id);
                  }}
                  title="Delete"
                >
                  ✕
                </button>
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="chat-main">
        {!activeConversation ? (
          <div className="welcome">
            <h2>LUNA</h2>
            <p>
              Your desktop AI assistant. Create a new conversation or select an
              existing one to get started.
            </p>
            <button className="btn btn-primary" onClick={handleNewConversation}>
              Start Conversation
            </button>
          </div>
        ) : (
          <>
            <div className="messages-area">
              {messages.length === 0 && !sending && (
                <div className="welcome">
                  <p>Send a message to start the conversation.</p>
                </div>
              )}
              {messages.map((msg) => (
                <MessageBubble key={msg.id} msg={msg} profiles={profiles} />
              ))}
              {toolExecutions.map((te) => (
                <ToolExecutionCard
                  key={te.id}
                  detail={`▸ ${te.detail}`}
                  status={te.status}
                />
              ))}
              {sending && streamingContent && (
                <div className="message message-assistant">
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
                                customStyle={{ margin: "8px 0", borderRadius: "8px", fontSize: "13px" }}
                              >
                                {codeStr}
                              </SyntaxHighlighter>
                            );
                          }
                          return (
                            <code
                              className={className}
                              style={{ background: "var(--bg-hover)", padding: "2px 6px", borderRadius: "4px", fontSize: "13px" }}
                              {...props}
                            >
                              {children}
                            </code>
                          );
                        },
                      }}
                    >
                      {streamingContent}
                    </ReactMarkdown>
                    <span className="cursor-blink">|</span>
                  </div>
                </div>
              )}
              {sending && !streamingContent && (
                <div className="message message-assistant">
                  <div className="message-bubble">
                    <div className="spinner" />
                  </div>
                </div>
              )}
              <div ref={messagesEndRef} />
            </div>

            <div className="input-area">
              <div className="input-wrapper">
                <select
                  className="model-selector"
                  value={selectedModel}
                  onChange={(e) => setSelectedModel(e.target.value)}
                >
                  <option value="">Active model</option>
                  {profiles
                    .filter((p) => p.enabled)
                    .map((p) => (
                      <option key={p.id} value={p.id}>
                        {p.name} — {p.model_id}
                      </option>
                    ))}
                </select>
                <textarea
                  ref={textareaRef}
                  className="chat-input"
                  value={input}
                  onChange={handleTextareaInput}
                  onKeyDown={handleKeyDown}
                  placeholder="Type a message... (Enter to send, Shift+Enter for newline)"
                  rows={1}
                />
              </div>
              {sending ? (
                <button
                  className="btn btn-danger"
                  onClick={handleCancel}
                  title="Stop generation"
                >
                  ■ Stop
                </button>
              ) : (
                <button
                  className="btn btn-primary"
                  onClick={handleSend}
                  disabled={!input.trim()}
                >
                  Send
                </button>
              )}
            </div>
          </>
        )}
      </div>
    </div>
  );
}
