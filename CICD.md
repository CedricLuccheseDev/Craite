# CI/CD - Guide Complet

## 📋 Vue d'ensemble

Le projet CrAIte utilise une CI/CD simplifiée avec **3 workflows GitHub Actions** :

| Workflow | Déclencheur | Objectif |
|----------|-------------|----------|
| **test.yml** | Push/PR sur `dev` ou PR sur `main` | Tests et validation qualité |
| **release.yml** | Push sur `main` | Versioning automatique + build Tauri |
| **build-desktop.yml** | Appelé par `release.yml` ou manuel | Build multi-platform de l'app desktop |

---

## 🔄 Workflow de développement

### 1️⃣ Développement sur `dev`

```bash
# Créer une feature branch
git checkout -b feature/ma-feature

# Faire vos modifications
git add .
git commit -m "feat: ajout nouvelle fonctionnalité"

# Push sur dev (déclenche test.yml)
git push origin dev
```

**Ce qui se passe :**
- GitHub Actions exécute [`.github/workflows/test.yml`](.github/workflows/test.yml)
- Jobs en parallèle :
  - **Frontend** : lint, format, typecheck (desktop + landing), tests, coverage
  - **Backend** : cargo check, clippy, fmt, test, audit
- ✅ Si tous les tests passent → OK pour merger dans `main`
- ❌ Si échec → corrigez et re-push

### 2️⃣ Merge dans `main`

```bash
# Créer une PR de dev → main
gh pr create --base main --head dev --title "Release: nouvelle version"

# Ou via l'interface GitHub
# → test.yml se relance sur la PR
# → Vérifier que les checks passent
# → Merge la PR
```

**Ce qui se passe après le merge :**
1. GitHub Actions exécute [`.github/workflows/release.yml`](.github/workflows/release.yml)
2. **Job `release-please`** :
   - Analyse les commits depuis la dernière release
   - Si commits conventionnels détectés (`feat:`, `fix:`, etc.) :
     - Bump la version (MAJOR.MINOR.PATCH)
     - Génère un CHANGELOG
     - Crée un tag Git (ex: `v1.2.0`)
     - Crée une GitHub Release
3. **Job `build-tauri`** (s'exécute **toujours**, pas seulement sur releases) :
   - Appelle `build-desktop.yml`
   - Build multi-platform : Windows, macOS (x2), Linux
   - Upload les artifacts signés sur GitHub Releases
4. **Dokploy détecte le tag** (si créé) :
   - Déploie automatiquement la landing page

---

## 📦 Les 3 Workflows en détail

### 1. `test.yml` - Tests automatiques

**Fichier :** [`.github/workflows/test.yml`](.github/workflows/test.yml)

**Déclencheurs :**
```yaml
on:
  push:
    branches: [dev]
  pull_request:
    branches: [dev, main]
```

**Jobs :**

#### Frontend Tests
- **Lint** : ESLint sur tout le code TypeScript/Vue
- **Format** : Vérification Prettier
- **Typecheck** :
  - Desktop : `vue-tsc --noEmit`
  - Landing : `nuxi typecheck`
- **Tests** : Vitest avec coverage
- **Audit** : Vulnérabilités npm (non-bloquant)

#### Backend Tests (Rust)
- **Check** : `cargo check` (compilation sans build)
- **Clippy** : Linter Rust avec warnings stricts
- **Format** : `cargo fmt --check`
- **Tests** : `cargo test`
- **Audit** : Vulnérabilités Rust (non-bloquant)

**Durée moyenne :** 3-5 minutes

---

### 2. `release.yml` - Release + Build

**Fichier :** [`.github/workflows/release.yml`](.github/workflows/release.yml)

**Déclencheur :**
```yaml
on:
  push:
    branches: [main]
```

**Job 1 : `release-please`**
- Utilise [googleapis/release-please-action@v4](https://github.com/googleapis/release-please-action)
- Détecte les commits conventionnels :
  - `feat:` → bump MINOR (1.0.0 → 1.1.0)
  - `fix:` → bump PATCH (1.0.0 → 1.0.1)
  - `BREAKING CHANGE:` → bump MAJOR (1.0.0 → 2.0.0)
  - `chore:`, `docs:`, etc. → pas de release
- Outputs :
  - `release_created` : `true` si tag créé
  - `tag_name` : ex: `v1.2.0`
  - `version` : ex: `1.2.0`

**Job 2 : `build-tauri`**
- **S'exécute toujours** (pas de condition)
- Appelle `build-desktop.yml` avec :
  - Tag de release si disponible
  - Sinon tag de dev : `dev-{sha}`
- Voir section suivante pour détails du build

**Durée moyenne :** 15-25 minutes (builds parallèles)

---

### 3. `build-desktop.yml` - Build multi-platform

**Fichier :** [`.github/workflows/build-desktop.yml`](.github/workflows/build-desktop.yml)

**Déclencheurs :**
- `workflow_call` : Appelé par `release.yml`
- `workflow_dispatch` : Déclenchement manuel depuis GitHub UI

**Matrix de build :**

| Platform | Rust Target | Artifact |
|----------|-------------|----------|
| Windows | - | `.exe`, `.msi` |
| macOS (Intel) | x86_64-apple-darwin | `.dmg`, `.app` |
| macOS (Apple Silicon) | aarch64-apple-darwin | `.dmg`, `.app` |
| Linux | - | `.deb`, `.AppImage` |

**Étapes :**
1. Setup : Node.js 22, pnpm, Rust
2. Install frontend dependencies (`pnpm install`)
3. Build Tauri app avec [tauri-apps/tauri-action@v0](https://github.com/tauri-apps/tauri-action)
4. Sign les artifacts (avec `TAURI_SIGNING_PRIVATE_KEY`)
5. Upload sur GitHub Releases
6. Génère `latest.json` pour l'updater

**Durée moyenne :** 10-20 minutes par plateforme (parallèle)

---

## 🔐 Configuration manuelle requise

### 1. Secrets GitHub

Allez dans **Settings** → **Secrets and variables** → **Actions** → **New repository secret**

| Secret | Description | Requis pour |
|--------|-------------|-------------|
| `GITHUB_TOKEN` | Token auto-généré | Releases, builds ✅ (automatique) |
| `TAURI_SIGNING_PRIVATE_KEY` | Clé privée pour signer les apps | Build Tauri ⚠️ |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Mot de passe de la clé | Build Tauri ⚠️ |

**Comment générer les clés de signature :**
```bash
# Générer une paire de clés
cd apps/desktop/src-tauri
pnpm tauri signer generate -- -w ~/.tauri/myapp.key

# Récupérer la clé privée
cat ~/.tauri/myapp.key
# → Copier dans TAURI_SIGNING_PRIVATE_KEY

# Le password vous sera demandé lors de la génération
# → Copier dans TAURI_SIGNING_PRIVATE_KEY_PASSWORD
```

**Note :** Sans ces secrets, le build échouera à l'étape de signature.

---

### 2. Branch Protection Rules

Allez dans **Settings** → **Branches** → **Add branch protection rule**

**Pour la branche `main` :**

✅ **Require a pull request before merging**
- Require approvals : 1 (optionnel si vous travaillez seul)
- Dismiss stale approvals : ✅

✅ **Require status checks to pass before merging**
- Status checks required :
  - `Frontend Tests`
  - `Backend Tests (Rust)`

✅ **Do not allow bypassing the above settings** (recommandé)

**Pourquoi c'est important :**
- Force les tests à passer avant tout merge dans `main`
- Évite les pushs accidentels qui casseraient la prod
- Garantit que `main` est toujours stable

---

### 3. Configuration Dokploy (Landing)

Dokploy surveille les **tags Git** pour déployer la landing.

**Configuration dans Dokploy :**
1. Repository : `https://github.com/votre-user/CrAIte`
2. Branch à surveiller : **Tags** (pas `main`)
3. Build command : `pnpm build:landing`
4. Output directory : `apps/landing/.output`
5. Auto-deploy : ✅ Activé

**Comment ça marche :**
- Quand `release-please` crée un tag (ex: `v1.2.0`)
- Dokploy le détecte automatiquement
- Lance le build et déploie la landing

**Tester manuellement :**
```bash
# Créer un tag manuellement
git tag v1.0.0-test
git push origin v1.0.0-test

# Dokploy devrait détecter et déployer
```

---

## 📝 Commits Conventionnels

Pour que `release-please` fonctionne, respectez le format [Conventional Commits](https://www.conventionalcommits.org/) :

### Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types principaux

| Type | Description | Bump version |
|------|-------------|--------------|
| `feat` | Nouvelle fonctionnalité | MINOR (1.0.0 → 1.1.0) |
| `fix` | Correction de bug | PATCH (1.0.0 → 1.0.1) |
| `BREAKING CHANGE` | Changement incompatible | MAJOR (1.0.0 → 2.0.0) |
| `chore` | Maintenance, config | Aucun |
| `docs` | Documentation | Aucun |
| `style` | Formatage, style | Aucun |
| `refactor` | Refactoring | Aucun |
| `test` | Ajout/modification tests | Aucun |
| `perf` | Optimisation performance | PATCH |

### Exemples

```bash
# Feature (MINOR bump)
git commit -m "feat: add dark mode toggle"
git commit -m "feat(landing): add pricing section"

# Bug fix (PATCH bump)
git commit -m "fix: resolve login timeout issue"
git commit -m "fix(desktop): prevent crash on file import"

# Breaking change (MAJOR bump)
git commit -m "feat!: redesign API endpoints

BREAKING CHANGE: All API endpoints now use /v2/ prefix"

# Chore (pas de release)
git commit -m "chore: update dependencies"
git commit -m "docs: update README installation steps"
```

**⚠️ Important :**
- Si aucun commit conventionnel depuis la dernière release → **pas de nouveau tag**
- Mais le build Tauri se fait quand même (avec tag `dev-{sha}`)

---

## 🧪 Tester la CI/CD

### Test 1 : Vérifier que les tests se lancent sur `dev`

```bash
git checkout dev
git commit --allow-empty -m "test: trigger CI"
git push origin dev
```

**Vérifier :**
- Aller sur [Actions](../../actions)
- Workflow **Tests** doit apparaître
- Vérifier que `Frontend Tests` et `Backend Tests (Rust)` passent

---

### Test 2 : Vérifier qu'une PR déclenche les tests

```bash
git checkout -b test-pr
git commit --allow-empty -m "test: PR check"
git push origin test-pr
gh pr create --base dev --title "Test PR"
```

**Vérifier :**
- La PR affiche les checks en cours
- Ne peut pas merger tant que les checks ne passent pas

---

### Test 3 : Créer une release avec tag

```bash
git checkout main
git commit --allow-empty -m "feat: test release-please"
git push origin main
```

**Vérifier :**
1. Aller sur [Actions](../../actions) → **Release & Deploy** doit être en cours
2. Après ~5 min, un nouveau tag doit apparaître (ex: `v1.1.0`)
3. Aller sur [Releases](../../releases) → nouvelle release avec CHANGELOG
4. Attendre ~20 min → vérifier que les artifacts Tauri sont uploadés (Windows, macOS, Linux)
5. Vérifier dans Dokploy que la landing a été déployée

---

### Test 4 : Build sans release (commit non-conventional)

```bash
git checkout main
git commit --allow-empty -m "docs: update README"
git push origin main
```

**Vérifier :**
1. [Actions](../../actions) → **Release & Deploy** se lance
2. **Pas de nouveau tag créé** (commit `docs:` ne bump pas la version)
3. **Mais le build Tauri se fait quand même** avec tag `dev-{sha}`
4. Les artifacts sont quand même buildés et uploadés

---

### Test 5 : Build manuel

```bash
# Via GitHub CLI
gh workflow run build-desktop.yml -f tag_name=v1.0.0-manual -f version=1.0.0

# Ou via l'interface GitHub
# Actions → Build Desktop → Run workflow → remplir tag_name et version
```

**Vérifier :**
- Le build se lance manuellement
- Utile pour rebuilder une version spécifique

---

## 🐛 Troubleshooting

### ❌ Les tests échouent sur `dev`

**Problème :** `Frontend Tests` ou `Backend Tests (Rust)` sont en échec

**Solutions :**
```bash
# Lancer les tests en local
pnpm lint
pnpm format:check
pnpm test --run
cd apps/desktop/src-tauri && cargo test

# Corriger les erreurs puis re-push
git add .
git commit -m "fix: resolve test failures"
git push origin dev
```

---

### ❌ `release-please` ne crée pas de tag

**Problème :** Push sur `main` mais aucun tag créé

**Cause probable :** Commits non-conventionnels (ex: `update code`, `fix bug` au lieu de `fix: bug`)

**Solution :**
```bash
# Vérifier l'historique des commits
git log --oneline

# Si besoin, forcer un release
git commit --allow-empty -m "feat: trigger release"
git push origin main
```

---

### ❌ Build Tauri échoue avec "Signing key not found"

**Problème :** `build-desktop.yml` échoue à l'étape de signature

**Cause :** Secrets `TAURI_SIGNING_PRIVATE_KEY` ou `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` manquants

**Solution :**
1. Générer les clés (voir section "Secrets GitHub")
2. Ajouter les secrets dans Settings → Secrets and variables → Actions
3. Re-lancer le workflow :
   ```bash
   gh workflow run build-desktop.yml -f tag_name=v1.0.0 -f version=1.0.0
   ```

---

### ❌ Dokploy ne déploie pas la landing

**Problème :** Tag créé mais landing pas déployée

**Vérifications :**
1. **Dokploy est configuré pour surveiller les tags ?**
   - Aller dans Dokploy → Project → Settings
   - Branch: `Tags` (pas `main`)
   - Auto-deploy: ✅

2. **Le tag a bien été créé ?**
   ```bash
   git fetch --tags
   git tag -l
   # Vérifier que le tag existe (ex: v1.2.0)
   ```

3. **Logs Dokploy**
   - Vérifier les logs de déploiement
   - Vérifier que le build command est correct : `pnpm build:landing`

---

### ❌ "Build failed: Matrix job failed"

**Problème :** Un des jobs de la matrix échoue (ex: macOS)

**Solutions :**
- Vérifier les logs du job spécifique (Windows, macOS, Linux)
- Souvent lié à des dépendances manquantes :
  - **macOS** : Xcode version, signing certificates
  - **Linux** : System dependencies (`libwebkit2gtk`, etc.)
  - **Windows** : Visual Studio Build Tools

**Re-lancer un build spécifique :**
```bash
# Re-lancer tout le build
gh workflow run build-desktop.yml -f tag_name=v1.0.0 -f version=1.0.0
```

---

## 📊 Monitoring & Statistiques

### Voir l'état des workflows

```bash
# Liste des derniers runs
gh run list

# Détails d'un run spécifique
gh run view <run-id>

# Logs d'un run
gh run view <run-id> --log
```

### Dashboard GitHub Actions

Aller sur [Actions](../../actions) pour voir :
- ✅ Succès / ❌ Échecs récents
- ⏱️ Durée moyenne des workflows
- 📊 Statistiques d'utilisation (minutes CI/CD)

---

## 🚀 Workflow complet (exemple)

Scénario : Vous voulez ajouter une nouvelle feature et déployer

```bash
# 1. Créer une feature branch
git checkout dev
git pull origin dev
git checkout -b feature/new-dashboard

# 2. Développer la feature
# ... coder ...
git add .
git commit -m "feat(desktop): add analytics dashboard"

# 3. Push et vérifier que les tests passent
git push origin feature/new-dashboard
# → test.yml se lance et valide

# 4. Merger dans dev
git checkout dev
git merge feature/new-dashboard
git push origin dev
# → test.yml se relance

# 5. Créer une PR vers main
gh pr create --base main --head dev --title "Release: Analytics Dashboard"
# → test.yml se lance sur la PR

# 6. Merger la PR (après review)
gh pr merge --merge
# → release.yml se déclenche automatiquement
# → release-please crée v1.3.0 (feat = MINOR bump)
# → build-tauri build pour toutes les plateformes
# → Dokploy détecte le tag v1.3.0 et déploie la landing

# 7. Vérifier le déploiement
# → GitHub Releases : artifacts disponibles
# → Dokploy : landing déployée
# → Les utilisateurs peuvent update l'app desktop
```

**Durée totale du cycle (dev → prod) :** ~30-40 minutes (dont 20-30 min de build)

---

## 📚 Ressources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Release Please](https://github.com/googleapis/release-please)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [Tauri Action](https://github.com/tauri-apps/tauri-action)
- [Dokploy Documentation](https://dokploy.com/docs)

---

## 🎯 Checklist de mise en prod

Avant de push votre première release :

- [ ] Secrets GitHub configurés (`TAURI_SIGNING_PRIVATE_KEY`, etc.)
- [ ] Branch protection activée sur `main`
- [ ] Dokploy configuré pour surveiller les tags
- [ ] Tests en local passent (`pnpm test --run`, `cargo test`)
- [ ] Premier commit conventionnel prêt (`feat:`, `fix:`, etc.)
- [ ] README et CHANGELOG à jour

Une fois tout configuré, la CI/CD gère automatiquement :
- ✅ Tests à chaque push/PR
- ✅ Versioning sémantique (SemVer)
- ✅ Builds multi-platform signés
- ✅ Déploiement landing via tags
- ✅ GitHub Releases avec CHANGELOG

**Vous n'avez plus qu'à coder et commit ! 🎉**
