use std::path::{Path, PathBuf};

/// Validates that a path is safe for filesystem operations.
/// Rejects paths containing traversal sequences and verifies
/// the resolved path stays within an allowed root.
pub fn validate_path(path: &str, allowed_roots: &[&Path]) -> Result<PathBuf, String> {
    if path.is_empty() {
        return Err("Empty path".into());
    }

    // Reject obvious traversal patterns before touching the filesystem
    if contains_traversal(path) {
        return Err("Path contains traversal sequences".into());
    }

    let candidate = Path::new(path);

    // Canonicalize if the path exists, otherwise use lexical cleanup
    let resolved = if candidate.exists() {
        candidate
            .canonicalize()
            .map_err(|e| format!("Cannot resolve path: {e}"))?
    } else {
        lexical_clean(candidate)
    };

    // Check that resolved path falls under at least one allowed root
    if allowed_roots.is_empty() {
        return Ok(resolved);
    }

    for root in allowed_roots {
        let root_resolved = if root.exists() {
            root.canonicalize().unwrap_or_else(|_| root.to_path_buf())
        } else {
            lexical_clean(root)
        };
        if resolved.starts_with(&root_resolved) {
            return Ok(resolved);
        }
    }

    Err(format!(
        "Path '{}' is outside allowed directories",
        resolved.display()
    ))
}

/// Validates that a path is safe without restricting to specific roots.
/// Blocks traversal attacks but allows any absolute path.
pub fn sanitize_path(path: &str) -> Result<PathBuf, String> {
    validate_path(path, &[])
}

fn contains_traversal(path: &str) -> bool {
    path.contains("..") || path.contains('\0')
}

/// Lexical path cleanup without filesystem access (for paths that don't exist yet).
fn lexical_clean(path: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                result.pop();
            }
            std::path::Component::CurDir => {}
            other => result.push(other),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_traversal() {
        let roots = [Path::new("/home/user/music")];
        assert!(validate_path("/home/user/music/../../../etc/passwd", &roots).is_err());
    }

    #[test]
    fn rejects_null_bytes() {
        assert!(sanitize_path("/home/user/\0evil").is_err());
    }

    #[test]
    fn rejects_empty_path() {
        assert!(sanitize_path("").is_err());
    }

    #[test]
    fn accepts_clean_path() {
        assert!(sanitize_path("/home/user/music/sample.wav").is_ok());
    }
}
