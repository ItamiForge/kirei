use anyhow::Result;
use colored::*;
use human_bytes::human_bytes;
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CategoryType {
    Xcode,
    Node,
    Cargo,
    Cache,
    Logs,
    Docker,
    Go,
    Python,
    Analyzer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanItem {
    pub path: PathBuf,
    pub size: u64,
    pub name: String,
    pub selected: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub category: CategoryType,
    pub items: Vec<ScanItem>,
    pub total_size: u64,
}

pub struct Scanner {
    home_dir: PathBuf,
    config: crate::config::Config,
}

impl Scanner {
    pub async fn new() -> Self {
        let home_dir = directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .to_path_buf();
        
        let config = crate::config::load_config().await;

        Self { home_dir, config }
    }

    fn is_ignored(&self, path: &Path) -> bool {
        // First check static ignores
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if self.config.ignore_patterns.iter().any(|p| {
            // Simple glob / substring match for now
            name.contains(p) || p == name
        }) {
            return true;
        }

        // Check against .kireiignore if exists
        // (Existing Gitignore logic could be moved here if we want to combine them)
        false
    }

    pub async fn scan_all(&self) -> Result<Vec<ScanResult>> {
        let mut results = Vec::new();

        results.push(self.scan_xcode().await?);
        results.push(self.scan_node().await?);
        results.push(self.scan_cargo().await?);
        results.push(self.scan_caches().await?);
        results.push(self.scan_logs().await?);
        
        // Phase 2 new scanners
        results.push(self.scan_docker().await?);
        results.push(self.scan_go().await?);
        results.push(self.scan_python().await?);
        results.push(self.scan_analyzer().await?);

        Ok(results.into_iter().filter(|r| !r.items.is_empty()).collect())
    }

    async fn scan_analyzer(&self) -> Result<ScanResult> {
        let mut items = Vec::new();
        let search_roots = vec![
            self.home_dir.join("Downloads"),
            self.home_dir.join("Desktop"),
            self.home_dir.join("Documents"),
        ];

        for root in search_roots {
            if root.exists() {
                let it = WalkDir::new(root)
                    .max_depth(3)
                    .into_iter()
                    .filter_entry(|e| !self.is_ignored(e.path()));

                for entry in it.filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if let Ok(meta) = entry.metadata() {
                             let size = meta.len();
                             // Show files > 100MB
                             if size > 100 * 1024 * 1024 {
                                 items.push(ScanItem {
                                     path: entry.path().to_path_buf(),
                                     size,
                                     name: entry.file_name().to_string_lossy().to_string(),
                                     selected: false,
                                     description: "Large file detected (>100MB)".to_string(),
                                 });
                             }
                        }
                    }
                }
            }
        }

        Ok(ScanResult {
            category: CategoryType::Analyzer,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

    async fn scan_xcode(&self) -> Result<ScanResult> {
        let mut items = Vec::new();
        let paths = vec![
            (self.home_dir.join("Library/Developer/Xcode/DerivedData"), "DerivedData", "Intermediate build info & indexes", true),
            (self.home_dir.join("Library/Developer/Xcode/Archives"), "Archives", "App Archives", false),
            (self.home_dir.join("Library/Developer/Xcode/iOS DeviceSupport"), "iOS Device Support", "Symbols for connected devices", false),
        ];
        
        for (path, name, desc, selected) in paths {
            if path.exists() && !self.is_ignored(&path) {
                let size = get_size(&path);
                if size > 0 {
                    items.push(ScanItem {
                        path,
                        size,
                        name: name.to_string(),
                        selected,
                        description: desc.to_string(),
                    });
                }
            }
        }

        Ok(ScanResult {
            category: CategoryType::Xcode,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }
    
    async fn scan_node(&self) -> Result<ScanResult> {
         let mut items = Vec::new();
         
         // 1. Global npm cache
         let npm_cache = self.home_dir.join(".npm");
         if npm_cache.exists() && !self.is_ignored(&npm_cache) {
             let size = get_size(&npm_cache);
             if size > 0 {
                 items.push(ScanItem {
                     path: npm_cache,
                     size,
                     name: "npm Cache".to_string(),
                     selected: true,
                     description: "Local npm cache".to_string(),
                 });
             }
         }

         // 2. Recursive node_modules search (New in Phase 2)
         // We limit this to common dev roots to avoid scanning the whole disk
         let search_roots = vec![
             self.home_dir.join("Documents"),
             self.home_dir.join("Desktop"),
             self.home_dir.join("Developer"), // Common custom folder
             self.home_dir.join("GitHub"),    // Common custom folder
             self.home_dir.join("Projects"),  // Common custom folder
         ];

         for root in search_roots {
             if root.exists() {
                 let mut it = WalkDir::new(root)
                     .max_depth(5) // Don't go too deep for performance
                     .into_iter()
                     .filter_entry(|e| !self.is_ignored(e.path()));

                 while let Some(Ok(entry)) = it.next() {
                     if entry.file_name() == "node_modules" && entry.file_type().is_dir() {
                         let path = entry.path().to_path_buf();
                         let size = get_size(&path);
                         if size > 0 {
                              items.push(ScanItem {
                                 path: path.clone(),
                                 size,
                                 name: format!("node_modules ({})", entry.path().parent().and_then(|p| p.file_name()).map(|f| f.to_string_lossy()).unwrap_or_default()),
                                 selected: false,
                                 description: "Project dependencies".to_string(),
                             });
                         }
                         it.skip_current_dir(); // Don't look inside node_modules
                     }
                 }
             }
         }
         
         Ok(ScanResult {
            category: CategoryType::Node,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

    async fn scan_cargo(&self) -> Result<ScanResult> {
         let mut items = Vec::new();
         let paths = vec![
             (self.home_dir.join(".cargo/registry"), "Cargo Registry", "Downloaded crates source", false),
             (self.home_dir.join(".cargo/git"), "Cargo Git DB", "Git dependencies", false),
         ];

         for (path, name, desc, selected) in paths {
             if path.exists() && !self.is_ignored(&path) {
                let size = get_size(&path);
                if size > 0 {
                    items.push(ScanItem {
                        path,
                        size,
                        name: name.to_string(),
                        selected,
                        description: desc.to_string(),
                    });
                }
            }
         }

         Ok(ScanResult {
            category: CategoryType::Cargo,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

    async fn scan_caches(&self) -> Result<ScanResult> {
        let mut items = Vec::new();
        let user_cache = self.home_dir.join("Library/Caches");
        
        if let Ok(mut entries) = fs::read_dir(&user_cache).await {
             while let Ok(Some(entry)) = entries.next_entry().await {
                 let path = entry.path();
                 if path.is_dir() && !self.is_ignored(&path) {
                     let size = get_size(&path);
                     if size > 50 * 1024 * 1024 { // Only show > 50MB
                         items.push(ScanItem {
                             path: path.clone(),
                             size,
                             name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                             selected: false,
                             description: "User App Cache".to_string(),
                         });
                     }
                 }
             }
        }
        
        items.sort_by(|a, b| b.size.cmp(&a.size));

         Ok(ScanResult {
            category: CategoryType::Cache,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

    async fn scan_logs(&self) -> Result<ScanResult> {
        let mut items = Vec::new();
        let logs = self.home_dir.join("Library/Logs");
        if logs.exists() && !self.is_ignored(&logs) {
             let size = get_size(&logs);
             if size > 0 {
                 items.push(ScanItem {
                     path: logs,
                     size,
                     name: "User Logs".to_string(),
                     selected: true,
                     description: "~/Library/Logs".to_string(),
                 });
             }
        }
         Ok(ScanResult {
            category: CategoryType::Logs,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

    async fn scan_docker(&self) -> Result<ScanResult> {
        let mut items = Vec::new();
        // Docker is hard on macOS as it lives in a VM usually
        // But we can check for common buildx caches or local socket-based info if tools available
        // For now, checking the VM disk image if it's huge
        let docker_vm = self.home_dir.join("Library/Containers/com.docker.docker/Data/vms/0/data/Docker.raw");
        if docker_vm.exists() && !self.is_ignored(&docker_vm) {
            if let Ok(meta) = fs::metadata(&docker_vm).await {
                 items.push(ScanItem {
                     path: docker_vm,
                     size: meta.len(),
                     name: "Docker VM Disk".to_string(),
                     selected: false,
                     description: "Contains all Docker images/containers".to_string(),
                 });
            }
        }
        Ok(ScanResult {
            category: CategoryType::Docker,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

    async fn scan_go(&self) -> Result<ScanResult> {
        let mut items = Vec::new();
        let go_path = std::env::var("GOPATH").map(PathBuf::from).unwrap_or_else(|_| self.home_dir.join("go"));
        let pkg_mod = go_path.join("pkg/mod");
        
        if pkg_mod.exists() && !self.is_ignored(&pkg_mod) {
             let size = get_size(&pkg_mod);
              if size > 0 {
                 items.push(ScanItem {
                     path: pkg_mod,
                     size,
                     name: "Go Module Cache".to_string(),
                     selected: false,
                     description: "Downloaded Go modules".to_string(),
                 });
             }
        }
        Ok(ScanResult {
            category: CategoryType::Go,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

    async fn scan_python(&self) -> Result<ScanResult> {
        let mut items = Vec::new();
        let pip_cache = self.home_dir.join("Library/Caches/pip");
        if pip_cache.exists() && !self.is_ignored(&pip_cache) {
             let size = get_size(&pip_cache);
             if size > 0 {
                 items.push(ScanItem {
                     path: pip_cache,
                     size,
                     name: "Pip Cache".to_string(),
                     selected: false,
                     description: "Downloaded python packages".to_string(),
                 });
             }
        }
        Ok(ScanResult {
            category: CategoryType::Python,
            total_size: items.iter().map(|i| i.size).sum(),
            items,
        })
    }

}

fn get_size(path: &Path) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum()
}

pub fn print_report(results: &[ScanResult]) {
    for res in results {
        if res.total_size == 0 { continue; }
        println!("\n{} ({})", format!("{:?}", res.category).bold(), human_bytes(res.total_size as f64).yellow());
        for item in &res.items {
            println!("  - {:<40} {}", item.name, human_bytes(item.size as f64).cyan());
        }
    }
}
