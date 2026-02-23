import { invoke } from "@tauri-apps/api/core";
import { Plus, Save, Settings as SettingsIcon, X } from "lucide-react";
import { useEffect, useState } from "react";

interface Config {
  scan_paths: string[];
  ignore_patterns: string[];
}

export default function Settings() {
  const [config, setConfig] = useState<Config | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [permissions, setPermissions] = useState<boolean | null>(null);

  useEffect(() => {
    async function load() {
      try {
        const res = await invoke<Config>("get_config");
        setConfig(res);
        const perm = await invoke<boolean>("check_permissions");
        setPermissions(perm);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const handleSave = async () => {
    if (!config) return;
    setSaving(true);
    try {
      await invoke("save_config", { config });
      alert("Settings saved successfully!");
    } catch (e) {
      console.error("Failed to save config", e);
      alert("Failed to save: " + e);
    } finally {
      setSaving(false);
    }
  };

  if (loading || !config) {
    return <div className="p-8">Loading settings...</div>;
  }

  return (
    <div className="flex-1 h-screen overflow-y-auto p-8 bg-background">
      <header className="mb-8">
        <h2 className="text-3xl font-bold tracking-tight flex items-center gap-3">
          <SettingsIcon className="w-8 h-8 text-primary" />
          Settings
        </h2>
        <p className="text-muted-foreground mt-2">
          Configure search paths and whitelisting patterns.
        </p>
      </header>

      <div className="space-y-8 max-w-2xl">
        <section>
          <h3 className="text-lg font-semibold mb-4 text-foreground/90">
            Scan Locations
          </h3>
          <div className="space-y-2">
            {config.scan_paths.map((path, i) => (
              <div key={i} className="flex items-center gap-2">
                <input
                  value={path}
                  onChange={(e) => {
                    const next = {
                      ...config,
                      scan_paths: [...config.scan_paths],
                    };
                    next.scan_paths[i] = e.target.value;
                    setConfig(next);
                  }}
                  className="flex-1 bg-secondary/50 border border-border px-3 py-1.5 rounded-md text-sm outline-none focus:ring-1 focus:ring-primary text-foreground"
                />
                <button
                  onClick={() => {
                    const next = {
                      ...config,
                      scan_paths: config.scan_paths.filter(
                        (_, idx) => idx !== i,
                      ),
                    };
                    setConfig(next);
                  }}
                  className="p-1.5 hover:bg-destructive/10 text-destructive rounded-md"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>
            ))}
            <button
              onClick={() =>
                setConfig({ ...config, scan_paths: [...config.scan_paths, ""] })
              }
              className="text-xs font-medium text-primary hover:underline flex items-center gap-1 mt-2"
            >
              <Plus className="w-3 h-3" /> Add location
            </button>
          </div>
        </section>

        <section>
          <h3 className="text-lg font-semibold mb-4 text-foreground/90">
            Ignore Patterns (.kireiignore)
          </h3>
          <p className="text-xs text-muted-foreground mb-3">
            Paths or globs matching these will be skipped during scan.
          </p>
          <div className="space-y-2">
            {config.ignore_patterns.map((pattern, i) => (
              <div key={i} className="flex items-center gap-2">
                <input
                  value={pattern}
                  onChange={(e) => {
                    const next = {
                      ...config,
                      ignore_patterns: [...config.ignore_patterns],
                    };
                    next.ignore_patterns[i] = e.target.value;
                    setConfig(next);
                  }}
                  className="flex-1 bg-secondary/50 border border-border px-3 py-1.5 rounded-md text-sm outline-none focus:ring-1 focus:ring-primary font-mono text-foreground"
                />
                <button
                  onClick={() => {
                    const next = {
                      ...config,
                      ignore_patterns: config.ignore_patterns.filter(
                        (_, idx) => idx !== i,
                      ),
                    };
                    setConfig(next);
                  }}
                  className="p-1.5 hover:bg-destructive/10 text-destructive rounded-md"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>
            ))}
            <button
              onClick={() =>
                setConfig({
                  ...config,
                  ignore_patterns: [...config.ignore_patterns, ""],
                })
              }
              className="text-xs font-medium text-primary hover:underline flex items-center gap-1 mt-2"
            >
              <Plus className="w-3 h-3" /> Add pattern
            </button>
          </div>
        </section>

        <section className="mb-8">
          <h3 className="text-sm font-medium text-muted-foreground mb-4 uppercase tracking-wider">
            Permissions
          </h3>
          <div className="bg-card p-4 rounded-lg border shadow-sm flex items-center justify-between">
            <div>
              <div className="font-medium flex items-center gap-2">
                Full Disk Access
                {permissions === true ? (
                  <span className="text-xs bg-green-500/10 text-green-500 px-2 py-0.5 rounded-full border border-green-500/20">
                    Granted
                  </span>
                ) : (
                  <span className="text-xs bg-destructive/10 text-destructive px-2 py-0.5 rounded-full border border-destructive/20">
                    Missing
                  </span>
                )}
              </div>
              <p className="text-xs text-muted-foreground mt-1 max-w-sm">
                Required to scan and clean protected system directories (like
                Safari caches, Mail downloads, etc).
              </p>
            </div>
            {permissions === false && (
              <button
                onClick={async () => await invoke("open_privacy_settings")}
                className="text-xs border border-primary text-primary px-3 py-1.5 rounded hover:bg-primary/5 transition-colors"
              >
                Open Settings
              </button>
            )}
          </div>
        </section>

        <div className="pt-6 border-t border-border mt-8 flex justify-between items-center">
          <button
            onClick={handleSave}
            disabled={saving}
            className="bg-primary text-primary-foreground px-6 py-2 rounded-md font-medium flex items-center gap-2 hover:bg-primary/90 transition-all shadow-sm active:scale-95 disabled:opacity-50"
          >
            {saving ? (
              <div className="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin" />
            ) : (
              <Save className="w-4 h-4" />
            )}
            {saving ? "Saving..." : "Save Configuration"}
          </button>

          <div className="flex flex-col items-end">
            <button
              onClick={async () => {
                if (
                  confirm(
                    "DANGER: This will delete all Kirei configuration and settings, then close the app. You will need to manually move Kirei.app to Trash to finish uninstallation. Are you sure?",
                  )
                ) {
                  await invoke("reset_app");
                }
              }}
              className="text-destructive hover:text-destructive/80 text-xs font-medium border border-destructive/20 hover:bg-destructive/10 px-3 py-1.5 rounded transition-colors"
            >
              Factory Reset & Uninstall Prep
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
