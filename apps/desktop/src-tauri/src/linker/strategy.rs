use crate::error::CraiteError;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub enum LinkStrategy {
    Hardlink,
    #[allow(dead_code)]
    Copy,
}

/// Determine the best linking strategy for the given paths.
/// Hardlinks when source and output are on the same filesystem (zero extra disk space,
/// fully transparent to any app including DAWs). Falls back to copy across filesystems.
/// Symlinks are avoided: many apps (FL Studio, Ableton) don't resolve them for drag & drop.
pub fn determine_strategy(source: &Path, target: &Path) -> LinkStrategy {
    if same_filesystem(source, target) {
        LinkStrategy::Hardlink
    } else {
        LinkStrategy::Copy
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

    // Remove existing file to avoid "File exists" errors on re-link
    if target.exists() {
        std::fs::remove_file(target)?;
    }

    match strategy {
        LinkStrategy::Hardlink => {
            std::fs::hard_link(source, target)?;
        }
        LinkStrategy::Copy => {
            std::fs::copy(source, target)?;
        }
    }

    Ok(())
}

/// Walk up ancestors to find the first existing directory
fn first_existing_ancestor(p: &Path) -> Option<std::path::PathBuf> {
    let mut current = if p.is_file() { p.parent()? } else { p };
    loop {
        if current.exists() {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}

/// Check if two paths are on the same filesystem
fn same_filesystem(a: &Path, b: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let meta_a = std::fs::metadata(a).ok();
        let target_check = first_existing_ancestor(b);
        let meta_b = target_check.and_then(|p| std::fs::metadata(p).ok());
        match (meta_a, meta_b) {
            (Some(a), Some(b)) => a.dev() == b.dev(),
            _ => false,
        }
    }
    #[cfg(windows)]
    {
        fn volume_root(p: &Path) -> Option<std::path::PathBuf> {
            let mut components = p.components();
            let prefix = components.next()?;
            let root = components.next();
            match root {
                Some(r) => Some([prefix.as_os_str(), r.as_os_str()].iter().collect()),
                None => Some(std::path::PathBuf::from(prefix.as_os_str())),
            }
        }
        let target_check = first_existing_ancestor(b).unwrap_or_else(|| b.to_path_buf());
        match (volume_root(a), volume_root(&target_check)) {
            (Some(ra), Some(rb)) => {
                ra.to_string_lossy().to_lowercase() == rb.to_string_lossy().to_lowercase()
            }
            _ => false,
        }
    }
}
