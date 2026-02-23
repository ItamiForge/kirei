import {
  Box,
  CloudLightning,
  Container,
  FileText,
  Globe,
  HardDrive,
  Search,
  Zap,
} from "lucide-react";
import { cn } from "../lib/utils";
import { type CategoryType, formatBytes, type ScanResult } from "../types";

interface SidebarProps {
  results: ScanResult[];
  selectedCategory: CategoryType | "Dashboard" | "Settings";
  onSelect: (category: CategoryType | "Dashboard") => void;
  totalReclaimable: number;
  scanning: boolean;
  onScan: () => void;
}

const icons: Record<string, any> = {
  Xcode: Box,
  Node: CloudLightning,
  Cargo: Box,
  Cache: HardDrive, // or Trash2
  Logs: FileText,
  Docker: Container,
  Go: Zap,
  Python: Globe,
  Analyzer: Search,
};

export default function Sidebar({
  results,
  selectedCategory,
  onSelect,
  totalReclaimable,
  scanning,
  onScan,
}: SidebarProps) {
  return (
    <div className="w-64 bg-secondary/30 h-screen border-r border-border flex flex-col backdrop-blur-xl">
      <div className="p-6">
        <h1 className="text-xl font-bold tracking-tight flex items-center gap-2">
          <span className="text-primary">Kirei</span>
          <span className="text-xs font-normal text-muted-foreground bg-secondary px-2 py-0.5 rounded-full">
            Beta
          </span>
        </h1>
      </div>

      <div className="px-3 flex-1 overflow-y-auto">
        <div className="space-y-1">
          <button
            onClick={() => onSelect("Dashboard")}
            className={cn(
              "w-full text-left px-3 py-2 rounded-md text-sm font-medium transition-colors flex items-center justify-between",
              selectedCategory === "Dashboard"
                ? "bg-primary text-primary-foreground shadow-sm"
                : "hover:bg-accent hover:text-accent-foreground text-muted-foreground",
            )}
          >
            <span>Dashboard</span>
          </button>

          <button
            onClick={() => onSelect("Settings" as any)}
            className={cn(
              "w-full text-left px-3 py-2 rounded-md text-sm font-medium transition-colors flex items-center justify-between",
              selectedCategory === "Settings"
                ? "bg-primary text-primary-foreground shadow-sm"
                : "hover:bg-accent hover:text-accent-foreground text-muted-foreground",
            )}
          >
            <span>Settings</span>
          </button>

          <div className="pt-4 pb-2 px-3 text-xs font-semibold text-muted-foreground uppercase tracking-wider">
            Scanners
          </div>

          {results.map((res) => {
            const Icon = icons[res.category] || Box;
            const isSelected = selectedCategory === res.category;
            return (
              <button
                key={res.category}
                onClick={() => onSelect(res.category)}
                className={cn(
                  "w-full text-left px-3 py-2 rounded-md text-sm font-medium transition-colors flex items-center justify-between group",
                  isSelected
                    ? "bg-accent text-accent-foreground"
                    : "text-muted-foreground hover:bg-accent/50 hover:text-accent-foreground",
                )}
              >
                <div className="flex items-center gap-3">
                  <Icon
                    className={cn(
                      "w-4 h-4",
                      isSelected
                        ? "text-primary"
                        : "text-muted-foreground group-hover:text-primary",
                    )}
                  />
                  <span>{res.category}</span>
                </div>
                <span className="text-xs opacity-70">
                  {formatBytes(res.total_size, 0)}
                </span>
              </button>
            );
          })}
        </div>
      </div>

      <div className="p-4 border-t border-border bg-background/50">
        <div className="mb-4">
          <div className="text-xs text-muted-foreground font-medium mb-1">
            Total Reclaimable
          </div>
          <div className="text-2xl font-bold text-primary">
            {formatBytes(totalReclaimable)}
          </div>
        </div>
        <button
          onClick={onScan}
          disabled={scanning}
          className={cn(
            "w-full bg-primary text-primary-foreground py-2 rounded-md text-sm font-medium shadow hover:bg-primary/90 transition-all active:scale-95 disabled:opacity-50 disabled:pointer-events-none flex items-center justify-center gap-2",
            scanning && "animate-pulse",
          )}
        >
          {scanning ? (
            <>
              <svg
                className="animate-spin -ml-1 mr-2 h-4 w-4 text-white"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  className="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  strokeWidth="4"
                ></circle>
                <path
                  className="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
              </svg>
              Scanning...
            </>
          ) : (
            "Scan System"
          )}
        </button>
      </div>
    </div>
  );
}
