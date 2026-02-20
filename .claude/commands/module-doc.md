---
description: Create or update module documentation in _doc/modules/
argument-hint: [module name or "all" to check all]
---

# Module Documentation Command

Generate or update technical documentation for application modules in `_doc/modules/`.

## Instructions

### 1. Identify target modules

- If `$ARGUMENTS` specifies a module name (e.g., "TRANSIT", "CONGESTION"), target only that module
- If `$ARGUMENTS` is "all" or empty, scan for all modules and check which docs need creating or updating

### 2. Discover modules

Find all modules by scanning:
- `front/src/views/modulesMapbox/` for main view files
- `front/src/components/modules/` for module components
- Cross-reference with existing docs in `_doc/modules/`

### 3. Gather source code information

For each module to document, explore the source code to extract:
- Module name and permission page name
- Request subtypes (if any)
- Required parameters (origin/destination, zone, periods, vehicle types, etc.)
- Display rules (what's shown on map, tooltip content)
- Warning messages shown to users
- Threshold/color configuration (default values, color codes, ranges)
- Traffic direction handling
- Multi-period support and comparison mode
- Module-specific features (Amplisim, Reference Map Editor, diagrams, etc.)
- Export capabilities (CSV, XLSX, GeoJSON, Image)
- Label options (dev mode)
- Available map layers
- Main source files (views, components, composables, types, utils, backend)

Use parallel Explore agents (Haiku) to gather information from multiple modules simultaneously.

### 4. Follow the standard structure

Every doc MUST follow this exact section order:

```markdown
# Module [Nom Français] / [CODE_MODULE]

* Nom du module : **[moduleRequestName]**
* Nom de la page (permissions) : **[PAGE_NAME]**

[Sous-types de requête if applicable]

## Parametres requis

## Regles de gestion

### Affichage
### Avertissements
### Seuils / coloration
### Direction du trafic (if applicable)

## Multi periode (if applicable)

## [Module-specific sections] (if applicable)

## Exports

## Labels (mode dev) (if applicable)

## Calques disponibles

## Fichiers principaux
```

### 5. Formatting rules

- Write documentation in **French** with proper accents (e, e, e, c, etc.)
- Use Markdown tables for structured data (thresholds, colors, file paths)
- Include exact hex color codes with backticks
- Include exact default values and ranges from source code
- Keep descriptions concise and factual
- Do NOT add any AI attribution comments

### 6. Update vs Create

- **New module**: Create the full doc following the standard structure
- **Existing module**: Read the current doc, compare with source code, fix any inconsistencies or missing information. Preserve existing correct content.
- **Structure check**: Ensure section order matches the standard. Reorder if needed.

### 7. Final verification

After writing/updating, verify:
- All sections are in the correct order
- No broken internal references ("voir tableau ci-dessus" pointing to nothing)
- Color codes match source code constants
- Default threshold values match source code
- File paths in "Fichiers principaux" table are valid
