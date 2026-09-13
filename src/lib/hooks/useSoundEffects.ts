import { useEffect, useRef } from "react";
import { useAssistantStore, useSettingsStore } from "../stores";

class SoundManager {
  private ctx: AudioContext | null = null;
  private enabled = true;
  private volume = 0.3;

  setEnabled(enabled: boolean) {
    this.enabled = enabled;
  }

  setVolume(volume: number) {
    this.volume = volume;
  }

  private getCtx(): AudioContext {
    if (!this.ctx) {
      this.ctx = new AudioContext();
    }
    return this.ctx;
  }

  private playTone(frequency: number, duration: number, type: OscillatorType = "sine") {
    if (!this.enabled) return;
    try {
      const ctx = this.getCtx();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = type;
      osc.frequency.setValueAtTime(frequency, ctx.currentTime);
      gain.gain.setValueAtTime(this.volume * 0.15, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + duration);
      osc.connect(gain);
      gain.connect(ctx.destination);
      osc.start(ctx.currentTime);
      osc.stop(ctx.currentTime + duration);
    } catch {
      // Audio context may not be available
    }
  }

  playListening() {
    this.playTone(440, 0.15, "sine");
    setTimeout(() => this.playTone(520, 0.1, "sine"), 100);
  }

  playThinking() {
    this.playTone(330, 0.2, "triangle");
  }

  playToolStarted() {
    this.playTone(600, 0.1, "square");
  }

  playToolCompleted() {
    this.playTone(800, 0.1, "sine");
    setTimeout(() => this.playTone(1000, 0.1, "sine"), 80);
  }

  playMessageReceived() {
    this.playTone(660, 0.08, "sine");
  }

  playError() {
    this.playTone(200, 0.3, "sawtooth");
  }
}

export const soundManager = new SoundManager();

export function useSoundEffects() {
  const assistantState = useAssistantStore((s) => s.state);
  const settings = useSettingsStore((s) => s.settings);
  const prevState = useRef(assistantState);

  useEffect(() => {
    soundManager.setEnabled(settings?.voice?.tts_enabled ?? false);
    soundManager.setVolume(settings?.voice?.volume ?? 0.3);
  }, [settings]);

  useEffect(() => {
    if (assistantState === prevState.current) return;

    switch (assistantState) {
      case "listening":
        soundManager.playListening();
        break;
      case "thinking":
        soundManager.playThinking();
        break;
      case "working":
        soundManager.playToolStarted();
        break;
      case "composing":
        soundManager.playToolCompleted();
        break;
      case "error":
        soundManager.playError();
        break;
    }

    prevState.current = assistantState;
  }, [assistantState]);
}
