export type CategoryType =
  | "Xcode"
  | "Node"
  | "Cargo"
  | "Cache"
  | "Logs"
  | "Docker"
  | "Go"
  | "Python"
  | "Analyzer";

export interface ScanItem {
  path: string;
  size: number;
  name: string;
  selected: boolean;
  description: string;
}

export interface ScanResult {
  category: CategoryType;
  items: ScanItem[];
  total_size: number;
}

export function formatBytes(bytes: number, decimals = 2) {
  if (!+bytes) return "0 Bytes";
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = [
    "Bytes",
    "KiB",
    "MiB",
    "GiB",
    "TiB",
    "PiB",
    "EiB",
    "ZiB",
    "YiB",
  ];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / k ** i).toFixed(dm))} ${sizes[i]}`;
}
