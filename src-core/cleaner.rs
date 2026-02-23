use anyhow::Result;
use std::path::PathBuf;
use trash;

pub struct Cleaner;

impl Cleaner {
    pub fn move_to_trash(paths: &[PathBuf]) -> Result<u64> {
        if paths.is_empty() {
            return Ok(0);
        }

        let mut reclaimed_size = 0;
        for path in paths {
            if let Ok(metadata) = std::fs::symlink_metadata(path) {
                if metadata.is_dir() {
                    reclaimed_size += fs_extra::dir::get_size(path).unwrap_or(0);
                } else {
                    reclaimed_size += metadata.len();
                }
            }
        }

        trash::delete_all(paths)?;
        Ok(reclaimed_size)
    }

    /// Permanently delete files (rm -rf).
    /// 
    /// # Safety
    /// This method includes hardcoded checks to prevent deletion of:
    /// - Root directory (`/`)
    /// - User home root (`/Users`)
    /// 
    /// Use `move_to_trash` whenever possible instead.
    #[allow(dead_code)]
    pub fn delete_permanently(paths: &[PathBuf]) -> Result<u64> {
         if paths.is_empty() {
            return Ok(0);
        }
        
        // Safety check: Don't delete root or critical system paths
        // This is a basic safeguard.
        for path in paths {
            if path == &PathBuf::from("/") || path == &PathBuf::from("/Users") {
                return Err(anyhow::anyhow!("SAFETY GUARD: Attempting to delete critical path {:?}", path));
            }
        }

        let mut reclaimed_size = 0;
        for path in paths {
             // ... implementation ...
             if let Ok(metadata) = std::fs::symlink_metadata(path) {
                if metadata.is_dir() {
                    reclaimed_size += fs_extra::dir::get_size(path).unwrap_or(0);
                    std::fs::remove_dir_all(path)?;
                } else {
                    reclaimed_size += metadata.len();
                    std::fs::remove_file(path)?;
                }
            }
        }
        Ok(reclaimed_size)
    }
}
