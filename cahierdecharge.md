# 📘 Spécifications Techniques : Projet "RustGuard Vision"

**Version :** 1.0
**Stack :** Rust, Tauri, ONNX, PostgreSQL (pgvector)
**Type :** Desktop Application (Cross-platform) avec Backend embarqué.

---

## 1. Architecture Globale & Choix Technologiques

Pour garantir performance (Rust) et une interface "magnifique" et moderne, nous utiliserons l'architecture suivante :

1.  **Core (Backend) :** Rust pur. Gestion des threads, ingestion vidéo, calculs mathématiques.
2.  **AI Engine :** `ort` (Rust bindings pour ONNX Runtime) pour l'inférence matérielle accélérée.
3.  **Frontend (GUI) :** **Tauri** (Rust + Webview). L'interface sera codée en **React (TypeScript)** ou **Svelte** avec **TailwindCSS** pour un design fluide et réactif.
4.  **Base de Données :** **PostgreSQL** avec l'extension `pgvector` (Dockerisé ou local) pour la recherche vectorielle rapide.

---

## 2. Pipeline de Traitement Vidéo (Optimisation Stricte)

Le système **NE DOIT PAS** faire de reconnaissance faciale sur chaque frame. C'est la contrainte critique. Voici le pipeline séquentiel imposé :

1.  **Ingestion :** Décodage du flux RTSP (via `ffmpeg-next` ou `gstreamer`).
2.  **Filtre 1 - Skip Frame :** Ne traiter qu'une image toutes les N millisecondes (ex: 200ms).
3.  **Filtre 2 - Motion Detection :** Calcul d'un delta simple sur l'histogramme des pixels. Si mouvement < seuil, on jette la frame.
4.  **Filtre 3 - Face Detection (Léger) :** Modèle rapide (ex: UltraFace ou YuNet). Retourne des Bounding Boxes.
5.  **Filtre 4 - Object Tracking (Logiciel) :** Algorithme SORT ou ByteTrack.
    *   Si l'ID du visage est déjà suivi ("Tracked"), **ON NE LANCE PAS** la reconnaissance.
    *   On met à jour uniquement les coordonnées.
    *   On score la qualité de l'image (netteté, angle). On garde en cache la "Meilleure Image" de la séquence.
6.  **Extraction (Lourd) :** Une fois que le visage quitte l'écran ou après un délai fixe (ex: 2s de présence), on prend la "Meilleure Image" cacheée -> Modèle ArcFace -> Vecteur 512d -> DB.

---

## 3. Base de Données & Modèle de Données

Le schéma doit être relationnel et vectoriel.

*   **Table `identities`** :
    *   `id` (UUID, PK)
    *   `label` (String - ex: "Client Inconnu 4402", ou Nom assigné manuellement)
    *   `vector` (vector(512) - Empreinte biométrique moyenne)
    *   `created_at` (Timestamp)
    *   `last_seen` (Timestamp)
    *   `visit_count` (Int)

*   **Table `sightings` (Apparitions)** :
    *   `id` (UUID, PK)
    *   `identity_id` (FK -> identities)
    *   `camera_source` (String)
    *   `snapshot_uri` (String - Chemin vers le fichier image JPG stocké localement)
    *   `confidence` (Float)
    *   `timestamp` (Timestamp)

---

## 4. Spécifications de l'Interface Utilisateur (GUI)

L'interface doit être "Easy & Magnificent". Utilisation de **Shadcn/UI** ou **Mantine** recommandée. Thème sombre par défaut, accents néons/modernes.

### A. Dashboard "Live" (Vue Opérateur)
*   Grille dynamique des caméras (1x1, 2x2, etc.).
*   **Overlay AR :** Dessin des rectangles autour des visages en temps réel (Canvas HTML5 overlay sur le flux vidéo).
*   **Sidebar "Derniers Passages" :** Flux défilant vertical à droite montrant les visages capturés dans les 5 dernières minutes avec l'heure.

### B. Onglet "Analyse & Fréquentation" (Le besoin métier)
*   **Section "Habitués" :**
    *   Tableau filtrable : "Visiteurs vus plus de X fois ces Y derniers jours".
    *   Affichage sous forme de cartes "Profil" avec la photo la plus nette.
    *   Badge de statut : "Nouveau", "Régulier", "VIP" (basé sur la fréquence).
*   **Graphiques :** Histogramme des visites par heure de la journée.

### C. Onglet "Administration"
*   Gestion des sources caméras (Ajout URL RTSP, Nom).
*   Réglage des seuils (Seuil de confiance IA, Seuil de détection de mouvement).
*   Bouton "Purger la base de données" (RGPD).

---

## 5. Roadmap de Développement & Checklist de Suivi

Cochez les cases au fur et à mesure de l'avancement.

### Phase 1 : Fondations & Infrastructure
- [ ] **1.1 Setup Rust :** Initialiser projet Cargo workspace (Core + UI).
- [ ] **1.2 Setup Tauri :** Configurer Tauri avec React/TypeScript/Vite.
- [ ] **1.3 Database :** Monter un Docker PostgreSQL + pgvector et écrire les scripts de migration SQL (`sqlx`).
- [ ] **1.4 Logging :** Mettre en place `tracing` pour les logs (console + fichier).

### Phase 2 : Moteur de Vision (Backend Rust)
- [ ] **2.1 Connexion RTSP :** Réussir à lire un flux vidéo et décoder les frames en mémoire (`opencv` ou `ffmpeg`).
- [ ] **2.2 Motion Detector :** Implémenter la comparaison de pixels (frame diff) pour skipper les frames vides.
- [ ] **2.3 Détection Visage :** Intégrer le modèle ONNX de détection. Dessiner les box dans la console/log.
- [ ] **2.4 Tracking (SORT) :** Implémenter la logique d'ID unique tant que la personne est dans le cadre.

### Phase 3 : Reconnaissance & Stockage
- [ ] **3.1 Extraction Vecteur :** Intégrer le modèle ONNX de reconnaissance (ArcFace/MobileFace).
- [ ] **3.2 Logique de Comparaison :** Coder la fonction Cosine Similarity.
- [ ] **3.3 DB Insert :**
    - [ ] Si distance < 0.4 (exemple) => UPDATE identity (last_seen, visit_count++).
    - [ ] Sinon => INSERT new identity.
- [ ] **3.4 Stockage Image :** Sauvegarder le crop du visage (JPG) sur le disque dur dans un dossier organisé par date.

### Phase 4 : Interface Graphique (Frontend)
- [ ] **4.1 Communication :** Mettre en place les Commandes Tauri (Frontend appelle Backend) et Events (Backend pousse les frames/alertes au Frontend).
- [ ] **4.2 Vue Live :** Afficher le flux vidéo (via Canvas ou WebRTC local) et dessiner les rectangles reçus du backend.
- [ ] **4.3 Vue Analyse :** Créer la page "Clients Quotidiens". Faire la requête SQL `SELECT ... GROUP BY ... HAVING count > X`.
- [ ] **4.4 Design :** Appliquer le CSS (Tailwind), les animations de transition et le "Dark Mode".

### Phase 5 : Packaging & Optimisation Finale
- [ ] **5.1 Profiling :** Utiliser `flamegraph` pour vérifier qu'il n'y a pas de goulot d'étranglement CPU.
- [ ] **5.2 Gestion Erreurs :** S'assurer que si une caméra se déconnecte, le programme ne plante pas (Retry loop).
- [ ] **5.3 Build Release :** Compiler l'installateur (`.msi` ou `.deb`) via Tauri.

---

## 6. Contraintes de Sécurité (Strict)

1.  **Memory Safety :** Utilisation exclusive de Rust Safe, pas de bloc `unsafe` sauf nécessité absolue dans les bindings FFI.
2.  **Concurrency :** Utilisation de `Tokio` channels (`mpsc`) pour passer les images entre le thread de capture, le thread d'IA et le thread de DB. Ne jamais utiliser de Mutex bloquants sur le thread principal.
3.  **Données :** Les vecteurs faciaux sont des données biométriques.
    - [ ] Ajouter une option pour chiffrer la base de données.
    - [ ] Ajouter une "Retention Policy" (suppression auto après 30 jours).

---

## 7. Exemple de Requête SQL "Clients Quotidiens" (Pour Phase 4.3)

```sql
-- Récupérer les gens venus au moins 3 jours différents sur les 7 derniers jours
SELECT 
    i.id, 
    i.label, 
    count(DISTINCT date_trunc('day', s.timestamp)) as jours_de_visite,
    MAX(s.timestamp) as derniere_venue
FROM identities i
JOIN sightings s ON i.id = s.identity_id
WHERE s.timestamp > NOW() - INTERVAL '7 days'
GROUP BY i.id
HAVING count(DISTINCT date_trunc('day', s.timestamp)) >= 3
ORDER BY jours_de_visite DESC;
```
