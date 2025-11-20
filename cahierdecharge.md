

---

# 📘 Cahier des Charges Technique : "RustGuard Vision" (Qt Edition)

**Version :** 2.0
**Stack :** Rust, Qt6 (QML), CXX-Qt, ONNX, PostgreSQL
**Cible :** Desktop Application (Windows/Linux/macOS) haute performance.

---

## 1. Stack Technologique & Architecture Hybride

L'architecture repose sur une séparation stricte : le **Backend Rust** gère la logique lourde (IA, I/O), et le **Frontend QML** gère le rendu graphique.

*   **Langage Core :** Rust (Edition 2021).
*   **Framework GUI :** **Qt 6** via le moteur **QML**.
*   **Binding Rust-Qt :** **`cxx-qt`** (Le standard actuel pour lier Rust et Qt de manière "Safe").
*   **Moteur IA :** `ort` (ONNX Runtime) ou `tch-rs` (LibTorch).
*   **Base de Données :** PostgreSQL + `pgvector`.
*   **Build System :** Cargo combiné potentiellement avec CMake (souvent nécessaire pour Qt).

---

## 2. Architecture Logicielle (Pattern "Worker-Signal")

Contrairement au web, ici nous ne faisons pas de requêtes HTTP.

1.  **The UI Thread (Qt/Main) :** Il ne doit **JAMAIS** être bloqué. Il affiche l'interface QML.
2.  **The Bridge (CXX-Qt) :** Des objets Rust exposés comme des `QObject` au QML. Ils contiennent des `Properties` (ex: `cameraStatus`) et des `Signals` (ex: `newFaceDetected`).
3.  **The Worker Threads (Rust Pure) :**
    *   Un thread par caméra pour le décodage.
    *   Un thread global pour l'inférence IA (pour batcher les requêtes GPU).
    *   Ces threads communiquent avec le Bridge via des `channels` (Tokio mpsc ou Crossbeam).

---

## 3. Pipeline Vidéo Optimisé (Le "Core")

Le pipeline reste strict pour économiser le CPU/GPU.

1.  **Frame Capture :** `OpenCV` capture l'image brute.
2.  **Zero-Copy Rendering :** L'image brute est convertie en format affichable et envoyée à l'UI via un **QQuickImageProvider** (pour éviter de copier la mémoire inutilement vers QML).
3.  **Logic Gate (Optimisation) :**
    *   *Mouvement ?* Non -> Skip.
    *   *Tracking actif ?* Oui -> Skip reconnaissance.
    *   *Nouveau visage stable ?* -> **Snap & Analyze**.
4.  **Asynchronous Saving :** L'écriture en DB et sur disque se fait dans un thread détaché pour ne pas ralentir la vidéo.

---

## 4. Spécifications UI/UX (Qt Quick / QML)

L'interface sera développée en QML déclaratif pour un look "Cyberpunk Corporate" ou "Modern Clean".

### A. Fenêtre Principale (Dashboard)
*   **Layout :** `GridLayout` responsive.
*   **Video Element :** Composant personnalisé QML recevant le flux RGB du Rust.
*   **Overlays :** Les rectangles de détection ne sont pas "peints" sur l'image (lent), mais sont des objets `Rectangle {}` QML transparents posés par-dessus la vidéo, dont les coordonnées (x,y,w,h) sont bindées aux propriétés Rust. C'est ultra-fluide.

### B. Onglet "Analyse de Fréquentation"
*   **TableView moderne :** Liste des "Clients Quotidiens".
*   **Filtres Rapides :** Boutons QML stylisés ("Aujourd'hui", "7 derniers jours", "Clients VIP").
*   **Détail Profil :** Cliquer sur une ligne ouvre un `Drawer` ou une `Dialog` modale avec l'historique des photos de la personne.

### C. Onglet Administration
*   **Configuration Caméras :** Liste éditable des URL RTSP.
*   **System Monitor :** Jauges circulaires (QML Canvas) montrant l'usage CPU et RAM de l'application en temps réel.

---

## 5. Modèle de Données (DB)

*Identique à la version précédente (Postgres + pgvector), c'est le standard industriel.*

---

## 6. Checklist de Développement & Suivi

À cocher pour valider chaque étape d'ingénierie.

### Phase 1 : Environment & "Hello World"
- [ ] **1.1 Installation Qt6 :** Installer le SDK Qt officiel.
- [ ] **1.2 Setup CXX-Qt :** Configurer le `build.rs` pour compiler du code C++ généré automatiquement par Rust.
- [ ] **1.3 Hello QML :** Réussir à lancer une fenêtre QML depuis le `main.rs` Rust.
- [ ] **1.4 Database :** Docker Compose up pour Postgres.

### Phase 2 : Le Moteur Vidéo & Threading
- [ ] **2.1 Capture Rust :** Thread qui lit une vidéo en boucle avec OpenCV.
- [ ] **2.2 QQuickImageProvider :** Créer une classe C++/Rust capable de nourrir un composant `Image` QML avec des buffers de pixels bruts.
- [ ] **2.3 Optimisation Render :** Vérifier qu'on affiche 30 FPS sans faire monter le CPU à 100%.

### Phase 3 : Intelligence Artificielle (Pipeline)
- [ ] **3.1 Motion Detect :** Implémenter la différence d'histogramme (Rust pur).
- [ ] **3.2 Face Detect (ONNX) :** Intégrer le modèle de détection.
- [ ] **3.3 Tracking (ByteTrack/SORT) :** Assigner des IDs temporaires aux visages.
- [ ] **3.4 Bridge Signals :** Emettre un signal Rust `faceDetected(x, y, w, h)` et voir le rectangle bouger dans QML.

### Phase 4 : Identification & Base de Données
- [ ] **4.1 Embedding :** Calcul du vecteur (ArcFace) sur le thread IA.
- [ ] **4.2 Search Logic :** Requête SQL `ORDER BY vector <-> new_vector LIMIT 1`.
- [ ] **4.3 Insert/Update :** Logique métier (Nouveau visiteur vs Habitué).

### Phase 5 : UI "Magnifique" (QML Polishing)
- [ ] **5.1 Styling :** Créer un fichier `Theme.qml` (couleurs, polices, radius).
- [ ] **5.2 Dashboard :** Grille dynamique (si on ajoute une caméra, la grille se recalcule).
- [ ] **5.3 Page Analyse :** Connecter le `TableView` QML à un `QAbstractListModel` implémenté en Rust (pour afficher les données SQL).
- [ ] **5.4 Animations :** Ajouter des `Behavior on x { NumberAnimation { ... } }` sur les rectangles de visages pour un suivi fluide.

### Phase 6 : Packaging
- [ ] **6.1 Release Build :** Compilation en mode `--release`.
- [ ] **6.2 Deployqt :** Utiliser l'outil `windeployqt` ou `linuxdeployqt` pour inclure les DLLs Qt dans l'exécutable final.

---

## 7. Extrait de Code : Structure du Bridge (Rust/QML)

Voici à quoi ressemble la "colle" entre Rust et l'interface Qt pour ce projet :

```rust
// src/cxxqt_object.rs
#[cxx_qt::bridge]
mod my_object {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "com.rustguard", qml_version = "1.0")]
    pub struct VideoBackend {
        // Propriétés accessibles dans le QML (lecture seule ou lecture/écriture)
        #[qproperty]
        camera_count: i32,
        #[qproperty]
        last_detected_name: QString,
    }

    // Signaux envoyés du Rust vers le QML
    #[cxx_qt::qsignals(VideoBackend)]
    pub enum Signals {
        AlertPersonFound { name: QString, confidence: f32 },
    }

    impl qobject::VideoBackend {
        // Fonction appelable depuis un bouton QML
        #[qinvokable]
        pub fn start_camera_stream(self: Pin<&mut Self>, url: QString) {
            let url_str = url.to_string();
            
            // Lancer le thread async Rust ici
            tokio::spawn(async move {
                // Logique de capture...
            });
        }
    }
}
```
