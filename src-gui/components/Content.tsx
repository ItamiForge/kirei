import { Check, Trash2 } from "lucide-react";
import { useState } from "react";
import { type CategoryType, formatBytes, type ScanResult } from "../types";

interface ContentProps {
  results: ScanResult[];
  selectedCategory: CategoryType | "Dashboard";
  onToggleItem: (category: CategoryType, itemPath: string) => void;
  onClean: (paths: string[]) => Promise<void>;
}

export default function Content({
  results,
  selectedCategory,
  onToggleItem,
  onClean,
}: ContentProps) {
  const [cleaning, setCleaning] = useState(false);

  // Dashboard View
  if (selectedCategory === "Dashboard") {
    const totalSize = results.reduce((acc, r) => acc + r.total_size, 0);
    const topCategories = [...results].sort(
      (a, b) => b.total_size - a.total_size,
    );

    return (
      <div className="flex-1 h-screen overflow-y-auto p-8 bg-background">
        <header className="mb-8">
          <h2 className="text-3xl font-bold tracking-tight">System Overview</h2>
          <p className="text-muted-foreground mt-2">
            Summary of reclaimable space found on your system.
          </p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
          {/* Summary Card */}
          <div className="bg-card p-6 rounded-xl border shadow-sm col-span-1 md:col-span-2 lg:col-span-1">
            <h3 className="text-sm font-medium text-muted-foreground">
              Total Space to Clean
            </h3>
            <div className="mt-2 text-4xl font-black text-primary">
              {formatBytes(totalSize)}
            </div>
            <div className="mt-4 text-xs text-muted-foreground flex items-center gap-2">
              <Check className="w-4 h-4 text-green-500" /> Safe to remove
            </div>
          </div>
        </div>

        <h3 className="text-lg font-semibold mb-4">Top Consumers</h3>
        <div className="space-y-4">
          {topCategories.map((cat) => (
            <div
              key={cat.category}
              className="bg-card p-4 rounded-lg border flex items-center justify-between"
            >
              <div className="flex items-center gap-4">
                <div className="p-2 bg-secondary rounded-md">
                  <div className="w-6 h-6 flex items-center justify-center font-bold text-muted-foreground">
                    {cat.category[0]}
                  </div>
                </div>
                <div>
                  <div className="font-medium">{cat.category}</div>
                  <div className="text-xs text-muted-foreground">
                    {cat.items.length} items
                  </div>
                </div>
              </div>
              <div className="font-mono text-sm">
                {formatBytes(cat.total_size)}
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }

  // List View
  const categoryData = results.find((r) => r.category === selectedCategory);
  if (!categoryData) return <div className="p-8">Category not found</div>;

  const selectedItemsCount = categoryData.items.filter(
    (i) => i.selected,
  ).length;
  const selectedBytes = categoryData.items
    .filter((i) => i.selected)
    .reduce((acc, i) => acc + i.size, 0);

  const handleClean = async () => {
    const paths = categoryData.items
      .filter((i) => i.selected)
      .map((i) => i.path);
    if (paths.length === 0) return;

    if (
      !confirm(`Are you sure you want to move ${paths.length} items to Trash?`)
    )
      return;

    setCleaning(true);
    try {
      await onClean(paths);
    } finally {
      setCleaning(false);
    }
  };

  return (
    <div className="flex-1 h-screen flex flex-col bg-background">
      <header className="px-8 py-6 border-b border-border flex items-center justify-between bg-background/80 backdrop-blur-sm sticky top-0 z-10">
        <div>
          <h2 className="text-2xl font-bold">{selectedCategory}</h2>
          <p className="text-sm text-muted-foreground mt-1">
            {selectedItemsCount} selected ({formatBytes(selectedBytes)})
          </p>
        </div>

        <button
          onClick={handleClean}
          disabled={selectedItemsCount === 0 || cleaning}
          className="bg-destructive text-destructive-foreground px-4 py-2 rounded-md text-sm font-medium hover:bg-destructive/90 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2 transition-all"
        >
          {cleaning ? (
            <div className="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin" />
          ) : (
            <Trash2 className="w-4 h-4" />
          )}
          Clean Selected
        </button>
      </header>

      <div className="flex-1 overflow-y-auto p-6">
        <div className="bg-card rounded-lg border shadow-sm divide-y divide-border">
          {categoryData.items.length === 0 && (
            <div className="p-8 text-center text-muted-foreground">
              No items found in this scan.
            </div>
          )}
          {categoryData.items.map((item) => (
            <div
              key={item.path}
              className="p-4 flex items-start gap-4 hover:bg-accent/30 transition-colors group"
            >
              <div className="pt-1">
                <input
                  type="checkbox"
                  checked={item.selected}
                  onChange={() =>
                    onToggleItem(selectedCategory as CategoryType, item.path)
                  }
                  className="w-4 h-4 rounded border-gray-300 text-primary focus:ring-primary cursor-pointer accent-primary"
                />
              </div>
              <div
                className="flex-1 cursor-pointer"
                onClick={() =>
                  onToggleItem(selectedCategory as CategoryType, item.path)
                }
              >
                <div className="flex items-center justify-between mb-1">
                  <span className="font-medium text-sm group-hover:text-primary transition-colors">
                    {item.name}
                  </span>
                  <span className="text-sm font-mono text-muted-foreground">
                    {formatBytes(item.size)}
                  </span>
                </div>
                <div className="text-xs text-muted-foreground mb-1">
                  {item.description}
                </div>
                <div
                  className="text-[10px] text-muted-foreground/60 font-mono truncate max-w-lg"
                  title={item.path}
                >
                  {item.path}
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
