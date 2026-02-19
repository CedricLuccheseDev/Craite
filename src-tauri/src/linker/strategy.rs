use std::path::Path;
use crate::error::CraiteError;

#[derive(Debug, Clone, Copy)]
pub enum LinkStrategy {
    Hardlink,
    Symlink,
    Copy,
}

/// Determine the best linking strategy for the given paths
pub fn determine_strategy(source: &Path, target: &Path) -> LinkStrategy {
    // Hardlink only works on the same filesystem/partition
    if same_filesystem(source, target) {
        LinkStrategy::Hardlink
    } else {
        LinkStrategy::Symlink
    }
}

/// Create a link from source to target using the given strategy
pub fn create_link(
    source: &Path,
    target: &Path,
    strategy: LinkStrategy,
) -> Result<(), CraiteError> {
    // Ensure parent directory exists
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    match strategy {
        LinkStrategy::Hardlink => {
            std::fs::hard_link(source, target)?;
        }
        LinkStrategy::Symlink => {
            #[cfg(unix)]
            std::os::unix::fs::symlink(source, target)?;
            #[cfg(windows)]
            std::os::windows::fs::symlink_file(source, target)?;
        }
        LinkStrategy::Copy => {
            std::fs::copy(source, target)?;
        }
    }

    Ok(())
}

/// Check if two paths are on the same filesystem (Unix only)
fn same_filesystem(a: &Path, b: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let meta_a = std::fs::metadata(a).ok();
        // Use the parent of the target since it may not exist yet
        let target_check = if b.exists() { b.to_path_buf() } else {
            b.parent().unwrap_or(b).to_path_buf()
        };
        let meta_b = std::fs::metadata(&target_check).ok();

        match (meta_a, meta_b) {
            (Some(a), Some(b)) => a.dev() == b.dev(),
            _ => false,
        }
    }
    #[cfg(not(unix))]
    {
        let _ = (a, b);
        false
    }
}
