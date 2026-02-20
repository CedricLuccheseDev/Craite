Génère un rapport de mon travail du jour en récupérant :
1. Les commits effectués aujourd'hui (via git log)
2. Les branches avec du travail récent
3. Les MRs mergées et en draft

Format de sortie (optimisé pour Teams) :

```
📅 [DATE DU JOUR]

✅ MR mergées :
• [lien complet GitLab MR] - [titre]
  → [description courte]

🚧 MR Draft :
• [lien complet GitLab MR] - [titre]
  → [description courte]
```

Récupère l'URL du remote GitLab (git remote get-url origin) et construis les liens complets des MRs.
Exemple de lien : https://gitlab.com/AutoroutesTrafic/web/compass/compass-app/-/merge_requests/695
