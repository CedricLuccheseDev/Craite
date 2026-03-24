# CI/CD

## Workflows

| Workflow | Déclencheur | Rôle |
|----------|-------------|------|
| `ci.yml` | PR vers `dev` ou `main` | Lint, typecheck, tests |
| `release.yml` | Push d'un tag `v*` | Extrait la version, déclenche le build |
| `build-desktop.yml` | Appelé par `release.yml` ou manuellement | Build multi-platform + publication |

---

## Développement quotidien

```
feature branch  →  PR vers dev  →  merge
```

À chaque PR, le CI vérifie :
- **Frontend** : lint, format, typecheck, tests (Vitest)
- **Backend** : cargo check, clippy, fmt, tests

Si un check échoue, corriger en local avant de re-push :
```bash
pnpm lint:fix
pnpm format
pnpm typecheck
pnpm test
cargo clippy -- -D warnings   # dans apps/desktop/src-tauri/
cargo fmt                     # dans apps/desktop/src-tauri/
```

---

## Publier une release

**1. Bumper la version**

Critiques (l'auto-updater compare ces valeurs) :

| Fichier | Clé |
|---------|-----|
| `apps/desktop/src-tauri/tauri.conf.json` | `"version"` |
| `apps/desktop/src-tauri/Cargo.toml` | `version = "..."` |

Optionnels (cohérence, aucun impact fonctionnel) :

| Fichier | Clé |
|---------|-----|
| `package.json` | `"version"` |
| `apps/desktop/package.json` | `"version"` |
| `apps/landing/package.json` | `"version"` |

**2. Committer et merger dans `main`**

```bash
git add -p
git commit -m "chore: release v0.5.0"
# PR dev → main, attendre que le CI passe, merger
```

**3. Pousser le tag depuis `main`**

```bash
git checkout main && git pull
git tag v0.5.0
git push origin v0.5.0
```

Le CI build alors pour toutes les plateformes et publie la GitHub Release.

> Ne jamais pousser un tag depuis `dev` ou une branche feature.

---

## Secrets GitHub requis

**Settings → Secrets and variables → Actions**

| Secret | Requis pour |
|--------|-------------|
| `TAURI_SIGNING_PRIVATE_KEY` | Signer les artifacts (auto-update) |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Mot de passe de la clé de signature |

`GITHUB_TOKEN` est auto-généré par GitHub, pas besoin de le configurer.

---

## Build manuel

Pour builder sans passer par une release (debug, test CI) :

```bash
gh workflow run build-desktop.yml -f tag_name=v0.5.0 -f version=0.5.0
```

---

## Troubleshooting

| Problème | Solution |
|----------|----------|
| Lint fails | `pnpm lint:fix` en local |
| Typecheck fails | `pnpm typecheck` en local |
| Clippy fails | `cargo clippy -- -D warnings` dans `apps/desktop/src-tauri/` |
| Signing error | Vérifier `TAURI_SIGNING_PRIVATE_KEY` dans GitHub Settings |
| Tag déjà existant | `git tag -d v0.5.0 && git push origin :refs/tags/v0.5.0` puis recréer |

```bash
# Voir l'état des workflows
gh run list
gh run view <run-id> --log
```
