import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import Content from "./components/Content";
import Settings from "./components/Settings";
import SideBar from "./components/Sidebar";
import { type CategoryType, formatBytes, type ScanResult } from "./types";
import "./App.css";

function App() {
  const [results, setResults] = useState<ScanResult[]>([]);
  const [selectedCategory, setSelectedCategory] = useState<
    CategoryType | "Dashboard"
  >("Dashboard");
  const [scanning, setScanning] = useState(false);

  async function scan() {
    setScanning(true);
    try {
      const res = await invoke<ScanResult[]>("scan");
      setResults(res);
      // Auto-select dashboard on fresh scan
      // setSelectedCategory('Dashboard');
    } catch (e) {
      console.error("Scan failed:", e);
      alert("Scan failed: " + e);
    } finally {
      setScanning(false);
    }
  }

  // Initial scan
  useEffect(() => {
    scan();
  }, []);

  const totalReclaimable = results.reduce((acc, cat) => {
    // Calculate selected items only? Or total potential?
    // Usually total potential found.
    // But let's verify what the CLI did. CLI sums everything found.
    // But for reclaimable count in sidebar, maybe sum selected?
    // Let adds selected:
    return (
      acc + cat.items.filter((i) => i.selected).reduce((s, i) => s + i.size, 0)
    );
  }, 0);

  const toggleItem = (category: CategoryType, path: string) => {
    setResults((prev) =>
      prev.map((cat) => {
        if (cat.category !== category) return cat;
        return {
          ...cat,
          items: cat.items.map((item) => {
            if (item.path !== path) return item;
            return { ...item, selected: !item.selected };
          }),
        };
      }),
    );
  };

  const cleanItems = async (paths: string[]) => {
    console.log("App.tsx requesting clean for paths:", paths);
    try {
      // Tauri command expects Vec<PathBuf>
      const cleanedBytes = await invoke<number>("clean", { paths });
      console.log("App.tsx clean success, bytes:", cleanedBytes);
      alert(`Cleaned ${formatBytes(cleanedBytes)}`);
      // Re-scan to update UI
      await scan();
    } catch (e) {
      console.error("App.tsx clean failed:", e);
      alert("Clean failed: " + JSON.stringify(e));
    }
  };

  return (
    <div className="flex h-screen bg-background text-foreground font-sans overflow-hidden">
      <SideBar
        results={results}
        selectedCategory={selectedCategory}
        onSelect={setSelectedCategory as any}
        totalReclaimable={totalReclaimable}
        scanning={scanning}
        onScan={scan}
      />
      {selectedCategory === ("Settings" as any) ? (
        <Settings />
      ) : (
        <Content
          results={results}
          selectedCategory={selectedCategory as any}
          onToggleItem={toggleItem}
          onClean={cleanItems}
        />
      )}
    </div>
  );
}

export default App;
