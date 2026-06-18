# AI Clustering - Adresses

**Stack**: Rust + ndarray + linfa

**Architecture**: Injection dépendances via traits

## Structure
```
clustering-ai/
├── Cargo.toml
├── src/
│   ├── main.rs          # Point entrée
│   ├── clustering.rs    # Trait + algos
│   ├── address.rs       # Struct Address
│   └── encoder.rs       # Features -> vecteurs
```

## Commandes
```bash
# Créer projet
cargo init --name clustering-ai

# Build release (optimisé)
cargo build --release

# Run
cargo run --release
```

## Features
- **Trait ClusteringAlgorithm**: Interface injection dépendances
- **K-Means**: Clustering rapide
- **DBSCAN**: Clustering densité
- **Encodeur**: Texte + coords → vecteurs

## Dépendances Cargo.toml
```toml
ndarray = "0.15"
linfa = "0.7"
linfa-clustering = "0.7"
serde = { version = "1.0", features = ["derive"] }
```

## Optimisations
- `--release`: SIMD + optimisations
- Matrices ndarray: cache-friendly
- Binary statique: ~3MB
- Portable: Linux/Mac/Windows

## Usage code
```rust
let data = vec![Address::new(...)];
let kmeans = KMeans::new(5);
let clusters = kmeans.fit(&data);
```

**Réponses courtes, code direct, zéro verbiage.**
