# Video Analytics Backend

Ce projet est un backend d'analyse vidéo écrit en Rust. Il est conçu pour ingérer des flux vidéo, détecter des visages via un pipeline asynchrone, et exposer les événements via une API REST.

## État du Projet
Actuellement, le projet est une **implémentation squelette fonctionnelle (MVP)**.
*   **Architecture :** Pipeline asynchrone complet (Ingestion -> Détection -> Stockage -> API) implémenté avec `tokio`.
*   **Ingestion :** Utilise `MockVideoSource` (lit une image en boucle) car `ffmpeg` n'est pas disponible dans cet environnement de démo.
*   **Détection :** Utilise `MockFaceDetector` (retourne des détections simulées). Le code pour `OrtFaceDetector` (ONNX Runtime) est présent mais désactivé par défaut sans modèle.
*   **API :** Serveur `Axum` fonctionnel exposant le statut et les événements.

## Prérequis
*   Rust (dernière version stable)
*   `cargo`

## Installation et Lancement

1.  **Générer les données de test :**
    Le pipeline peut simuler un flux vidéo à partir d'une séquence d'images.
    ```bash
    cd video-analytics
    cargo run --bin generate_assets
    ```
    Cela crée `assets/sample_frame.jpg` (mode image simple) et `assets/video_seq/` (mode séquence vidéo).

2.  **Configuration de la source :**
    *   Par défaut, le programme cherche le dossier `assets/video_seq`. S'il existe, il joue la séquence en boucle.
    *   Sinon, il utilise `assets/sample_frame.jpg`.
    *   Pour utiliser vos propres vidéos, décomposez-les en images (ex: frame_001.jpg, frame_002.jpg...) et placez-les dans `assets/video_seq`.

3.  **Lancer le serveur :**
    ```bash
    cargo run --bin video-analytics
    ```
    Vous verrez des logs indiquant que le pipeline démarre et détecte des visages.

## Utilisation de l'API

Le serveur écoute sur `http://0.0.0.0:3000`.

*   **Vérifier le statut :**
    ```bash
    curl http://localhost:3000/status
    ```
    Réponse : `{"status":"running","uptime":"unknown"}`

*   **Lire un événement de détection :**
    ```bash
    curl http://localhost:3000/events
    ```
    Réponse (exemple) :
    ```json
    {
      "camera_id": "camera_01",
      "timestamp": "2025-11-20T00:25:00.123Z",
      "detections": [
        { "x1": 100.0, "y1": 100.0, "x2": 200.0, "y2": 200.0, "score": 0.95 }
      ]
    }
    ```

## Structure du Code
*   `src/ingestion.rs` : Gestion des sources vidéo (Trait `VideoSource`).
*   `src/detection.rs` : Logique de détection IA (Trait `FaceDetector`).
*   `src/pipeline.rs` : Orchestration asynchrone.
*   `src/api.rs` : Serveur Web.
