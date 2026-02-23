import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

const defaults = {
  topmost: false,
  sounds: false,
  lang: "English",
  theme: "Default",
  autoInject: false,
  saveTabs: true,
  autoUpdate: true,
  saveSettings: true,
  minimap: false,
  fontSize: 15,
};

type Settings = typeof defaults;

const validLangs = ["English", "French"];
const validThemes = ["Dark", "Light", "System"];

function validate(parsed: any): parsed is Settings {
  for (const key of Object.keys(defaults) as (keyof Settings)[]) {
    if (!(key in parsed)) {
      console.error(`importSettings: missing key "${key}"`);
      return false;
    }
    if (typeof parsed[key] !== typeof defaults[key]) {
      console.error(`importSettings: wrong type for "${key}"`);
      return false;
    }
  }
  if (parsed.fontSize < 1 || parsed.fontSize > 30) {
    console.error(`importSettings: fontSize out of range (${parsed.fontSize})`);
    return false;
  }
  if (!validLangs.includes(parsed.lang)) {
    console.error(`importSettings: invalid lang "${parsed.lang}"`);
    return false;
  }
  if (!validThemes.includes(parsed.theme)) {
    console.error(`importSettings: invalid theme "${parsed.theme}"`);
    return false;
  }
  return true;
}

export const s = writable<Settings>({ ...defaults });

export async function loadSettings() {
  try {
    const raw = await invoke<string>("load_settings");
    const parsed = JSON.parse(raw) as Partial<Settings>;
    s.set(parsed.saveSettings === false ? { ...defaults, saveSettings: false } : { ...defaults, ...parsed });
  } catch (e) {
    console.error("loadSettings failed", e);
    s.set({ ...defaults });
  }
}

export async function saveSettings(cur: Settings) {
  try {
    const data = cur.saveSettings ? cur : { saveSettings: false };
    await invoke("save_settings", { data: JSON.stringify(data) });
  } catch (e) {
    console.error("saveSettings failed", e);
  }
}

export async function exportSettings(cur: Settings) {
  try {
    await invoke("export_settings", { data: JSON.stringify(cur, null, 2) });
  } catch (e) {
    console.error("exportSettings failed", e);
  }
}

export async function importSettings(): Promise<boolean> {
  try {
    const raw = await invoke<string>("import_settings");
    if (!raw) return false;
    const parsed = JSON.parse(raw);
    if (!validate(parsed)) return false;
    s.set(parsed);
    s.update(cur => { saveSettings(cur); return cur; });
    return true;
  } catch (e) {
    console.error("importSettings failed", e);
    return false;
  }
}