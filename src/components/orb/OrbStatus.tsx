import { ThinkingOrb } from "thinking-orbs";
import { useAssistantStore } from "../../lib/stores";

const STATE_MAP: Record<string, "working" | "searching" | "solving" | "listening" | "connecting" | "weaving" | "composing" | "breathing" | "shaping"> = {
  idle: "breathing",
  listening: "listening",
  thinking: "solving",
  working: "working",
  searching: "searching",
  composing: "composing",
  speaking: "weaving",
  error: "shaping",
};

export function OrbStatus() {
  const state = useAssistantStore((s) => s.state);
  const orbState = STATE_MAP[state] || "breathing";

  return (
    <div className="orb-container">
      <ThinkingOrb state={orbState} size={64} theme="dark" />
    </div>
  );
}
