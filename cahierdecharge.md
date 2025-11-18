


# Cahier des Charges : Système d'Analyse Vidéo et Reconnaissance Faciale en Rust

## 1. Contexte et Objectifs
Le but est de développer un logiciel backend performant capable de se connecter à des flux RTSP (caméras de surveillance), de détecter et reconnaître les visages, de stocker les métadonnées et d'offrir une interface pour identifier les clients récurrents.

**Priorité absolue :** Optimisation des ressources (CPU/GPU) pour éviter le traitement redondant (ex: ne pas recalculer l'identité d'une personne 30 fois par seconde).

## 2. Architecture Technique

### 2.1 Stack Technologique (Rust Ecosystem)
*   **Langage :** Rust (Edition 2021+).
*   **Runtime Asynchrone :** `Tokio` (pour gérer plusieurs flux caméras simultanément).
*   **Acquisition Vidéo :** `ffmpeg-next` ou `gstreamer-rs` (pour décoder les flux RTSP).
*   **Inférence IA :** `ort` (binding Rust pour ONNX Runtime) ou `tch-rs` (LibTorch). *Recommandation : ONNX pour la portabilité et la vitesse.*
*   **Vision par ordinateur :** `image` et `imageproc` pour les manipulations basiques, ou `opencv` (binding) si des prétraitements complexes sont nécessaires.
*   **Base de Données :**
    *   **Métadonnées & Vecteurs :** `Qdrant` (écrit en Rust, natif vecteur) ou `PostgreSQL` avec l'extension `pgvector`.
*   **API/Backend Web :** `Axum` ou `Actix-web`.

---

## 3. Fonctionnalités Clés (Backend)

### 3.1 Module d'Ingestion (Video Pipeline)
*   **Connexion Multi-flux :** Le système doit accepter une liste d'URL RTSP (ex: `rtsp://admin:pass@192.168.1.x:554/stream`).
*   **Décodage Intelligent :** Ne pas décoder toutes les frames si inutile. Viser un *sampling* (ex: traiter 5 à 10 frames par seconde au lieu de 30/60).

### 3.2 Pipeline de Traitement (Core Logic)
C'est ici que l'optimisation est critique. Le pipeline suit ces étapes :

1.  **Détection de Mouvement (Pre-filter) :**
    *   Si l'image ne change pas significativement par rapport à la précédente, on saute les calculs lourds.
2.  **Détection de Visage (Face Detection) :**
    *   Utilisation d'un modèle léger (ex: *UltraFace* ou *SCRFD*).
    *   Extraction des "Bounding Boxes" (coordonnées du visage).
3.  **Tracking d'Objet (Object Tracking) - *Point Critique d'Optimisation* :**
    *   Implémentation d'un algorithme type **IOU Tracker** ou **SORT**.
    *   **Logique :** Si un visage détecté à la frame `T` correspond spatialement au visage de la frame `T-1`, on lui assigne le même `Tracker_ID`.
    *   **Règle :** On ne lance la reconnaissance faciale (étape 4) que **si le visage est nouveau** ou si la qualité de l'image du visage s'est considérablement améliorée (visage plus grand/plus face caméra).
4.  **Extraction de Caractéristiques (Embeddings) :**
    *   Utilisation d'un modèle type *ArcFace* ou *MobileFaceNet*.
    *   Conversion de l'image du visage en un vecteur numérique (ex: vecteur de 512 float).
    *   Calcul du "Score de Qualité" (netteté, angle). On ne garde que les visages de bonne qualité.

### 3.3 Gestion des Données et Stockage
*   **Stockage Image :** Sauvegarder le visage (crop) au format JPG/WebP sur le disque (nom de fichier : UUID).
*   **Stockage DB :**
    *   Enregistrement du vecteur.
    *   Timestamp.
    *   ID Caméra.
    *   Lien vers l'image.

### 3.4 Algorithme de Récurrence (Clustering)
*   À chaque nouvelle détection validée, le système interroge la base vectorielle (recherche par similarité cosinus).
*   **Seuil de tolérance :** Si distance < 0.4 (par exemple), c'est la même personne.
    *   $\rightarrow$ On met à jour la date "Dernière vue".
*   Sinon, c'est une nouvelle personne $\rightarrow$ Création nouvel ID.

---

## 4. Interface Utilisateur (Frontend)

L'interface peut être une WebApp (React/Vue) ou Desktop (Tauri + Rust).

### 4.1 Dashboard "Live"
*   Affichage des flux caméras.
*   Incrustation (Overlay) des rectangles verts sur les visages détectés.

### 4.2 Onglet "Analytique Récurrence" (Demande spécifique)
Cette vue doit afficher une table ou une grille filtrée :
*   **Filtre :** "Clients Fidèles" (Ceux vus aujourd'hui ET vus au moins X fois dans le passé).
*   **Affichage :**
    *   Photo de référence (la meilleure qualité capturée).
    *   Photo du jour (capture instantanée).
    *   Fréquence de visite (ex: "Vu 3 fois cette semaine").
    *   Heure d'arrivée aujourd'hui.

---

## 5. Stratégie d'Optimisation (Rust Specifics)

Pour éviter les calculs inutiles, le code devra respecter ces principes :

1.  **Zero-Copy Parsing :** Utiliser des structures de données qui ne copient pas la mémoire vidéo inutilement.
2.  **Concurrence (MPSC Channels) :**
    *   Thread 1 (Decode) $\rightarrow$ Channel $\rightarrow$ Thread 2 (Detect/Track) $\rightarrow$ Channel $\rightarrow$ Thread 3 (Recognize/Store).
    *   Si le buffer du Thread 3 est plein, le Thread 1 doit "dropper" (jeter) les frames pour ne pas saturer la latence (Backpressure).
3.  **Filtrage Géographique (ROI) :** Permettre de définir des zones mortes (ex: plafond) pour ne pas y chercher de visages.
4.  **Batch Processing :** Si GPU disponible, envoyer les images par lots (batch) au modèle ONNX plutôt qu'une par une.

---

## 6. Structure de la Base de Données (Exemple SQL)

```sql
-- Table des identités uniques (Personnes)
CREATE TABLE visitors (
    id UUID PRIMARY KEY,
    first_seen_at TIMESTAMP DEFAULT NOW(),
    last_seen_at TIMESTAMP DEFAULT NOW(),
    visit_count INT DEFAULT 1,
    best_face_image_path TEXT
);

-- Table des événements (Passages)
CREATE TABLE sightings (
    id UUID PRIMARY KEY,
    visitor_id UUID REFERENCES visitors(id),
    camera_id INT,
    captured_at TIMESTAMP DEFAULT NOW(),
    embedding VECTOR(512), -- Nécessite pgvector
    image_path TEXT,
    confidence FLOAT
);
```

## 7. Étapes de Développement

1.  **P:** Lire une vidéo, détecter les visages, dessiner un carré (sans reconnaissance).
2.  **Intégration ONNX :** Ajouter l'extraction de vecteur (embedding) et comparer deux images statiques.
3.  **Pipeline Async :** Implémenter le flux Gstreamer -> Tokio -> Inférence.
4.  **Tracker & Optimisation :** Ajouter la logique "Si ID tracké existe, pas d'inférence".
5.  **Base de données :** Connecter PostgreSQL/Qdrant.
6.  **Frontend :** Créer l'onglet de récurrence.

---

## Résumé des Crates Rust recommandées
*   `tokio` : Runtime async.
*   `ort` : Inférence ONNX (haute performance).
*   `image` : Manipulation d'images.
*   `sqlx` : Driver SQL asynchrone (performant et sûr).
*   `anyhow` & `thiserror` : Gestion d'erreurs.
*   `serde` : Sérialisation JSON.
