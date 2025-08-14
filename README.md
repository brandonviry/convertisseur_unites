# Convertisseur d'Unités

Cette application en ligne de commande permet de convertir des valeurs entre différentes unités de longueur, de masse et de température.

## Sommaire

- [Convertisseur d'Unités](#convertisseur-dunités)
  - [Sommaire](#sommaire)
  - [Installation](#installation)
  - [Utilisation](#utilisation)
    - [Syntaxe Générale](#syntaxe-générale)
    - [Sous-commandes](#sous-commandes)
    - [Conversion de Longueur](#conversion-de-longueur)
    - [Conversion de Masse](#conversion-de-masse)
    - [Conversion de Température](#conversion-de-température)
  - [Tests](#tests)
    - [Exécution des Tests](#exécution-des-tests)
    - [Types de Tests](#types-de-tests)
    - [Couverture des Tests](#couverture-des-tests)
  - [Exemple de Résultat](#exemple-de-résultat)
  - [Architecture](#architecture)
  - [Licence](#licence)
  - [Auteurs](#auteurs)

## Installation

1. **Clonez le dépôt** (si vous utilisez Git) ou téléchargez les fichiers sources du projet.
   ```bash
   git clone https://github.com/brandonviry/convertisseur_unites.git
   ```

2. **Compilez le projet** en mode release pour obtenir l'exécutable :
   ```bash
   cargo build --release
   ```

3. **L'exécutable** sera situé dans le répertoire `target/release`. Vous pouvez le renommer si nécessaire.

## Utilisation

### Syntaxe Générale

```bash
./convertisseur_unites <sous-commande> --from <unité> --value <valeur>
```

### Sous-commandes

- `longueur` : Convertit les unités de longueur.
- `masse` : Convertit les unités de masse.
- `temperature` : Convertit les unités de température.

### Conversion de Longueur

Pour convertir des unités de longueur :

```bash
./convertisseur_unites longueur --from <unité> --value <valeur>
```

**Unité d'origine** :
- `m` : mètres
- `cm` : centimètres
- `mm` : millimètres
- `km` : kilomètres
- `in` : pouces (inches)
- `ft` : pieds (feet)
- `yd` : yards
- `mi` : miles

**Exemple** :

Convertir 100 mètres en autres unités de longueur :
```bash
./convertisseur_unites longueur --from m --value 100
```

### Conversion de Masse

Pour convertir des unités de masse :

```bash
./convertisseur_unites masse --from <unité> --value <valeur>
```

**Unité d'origine** :
- `g` : grammes
- `kg` : kilogrammes
- `t` : tonnes

**Exemple** :

Convertir 2 kilogrammes en autres unités de masse :
```bash
./convertisseur_unites masse --from kg --value 2
```

### Conversion de Température

Pour convertir des unités de température :

```bash
./convertisseur_unites temperature --from <unité> --value <valeur>
```

**Unité d'origine** :
- `C` : Celsius
- `F` : Fahrenheit
- `K` : Kelvin

**Exemple** :

Convertir 25 degrés Celsius en Fahrenheit et Kelvin :
```bash
./convertisseur_unites temperature --from C --value 25
```

## Tests

Ce projet dispose d'une suite de tests complète pour garantir la fiabilité et la précision des conversions.

### Exécution des Tests

```bash
# Exécuter tous les tests
cargo test

# Exécuter seulement les tests unitaires
cargo test --lib

# Exécuter seulement les tests d'intégration
cargo test --test integration_tests

# Exécuter les tests avec sortie détaillée
cargo test -- --nocapture
```

### Types de Tests

#### Tests Unitaires (49 tests)
- **Tests de conversion de longueur** : Vérification de toutes les conversions entre unités métriques et impériales
- **Tests de conversion de masse** : Validation des conversions grammes/kilogrammes/tonnes
- **Tests de conversion de température** : Contrôle des conversions Celsius/Fahrenheit/Kelvin
- **Tests de gestion d'erreurs** : Validation du comportement avec des unités invalides
- **Tests de précision** : Vérification des conversions aller-retour
- **Tests de cas limites** : Valeurs négatives, zéro, valeurs extrêmes

#### Tests d'Intégration (9 tests)
- **Tests de cohérence** : Vérification que les modules fonctionnent ensemble
- **Tests de cas d'usage réels** : Scénarios pratiques (marathon, construction, météo)
- **Tests de précision globale** : Validation de la précision sur l'ensemble du système
- **Tests de robustesse** : Comportement avec des valeurs extrêmes

#### Tests de Documentation (1 test)
- **Test des exemples** : Validation que les exemples de code dans la documentation fonctionnent

### Couverture des Tests

- ✅ **100% des fonctions publiques testées**
- ✅ **Tous les cas d'erreur couverts**
- ✅ **Tests de précision pour toutes les conversions**
- ✅ **Validation des cas limites et valeurs extrêmes**
- ✅ **Tests de cohérence entre modules**

## Exemple de Résultat

**Conversion de longueur** :
```bash
$ ./convertisseur_unites longueur --from m --value 100
100 m = 10000 cm, 100000 mm, 0.1 km, 3937.01 in, 328.08 ft, 109.36 yd, 0.062 mi
```

**Conversion de masse** :
```bash
$ ./convertisseur_unites masse --from kg --value 2
2 kg = 2000 g, 0.002 t
```

**Conversion de température** (améliorée) :
```bash
$ ./convertisseur_unites temperature --from C --value 25
25 C = 77 F, 298.15 K
```

## Architecture

Le projet est organisé en modules séparés pour une meilleure maintenabilité :

- **`src/main.rs`** : Point d'entrée de l'application CLI
- **`src/lib.rs`** : Bibliothèque exposant les modules publics
- **`src/longueur.rs`** : Module de conversion des unités de longueur
- **`src/masse.rs`** : Module de conversion des unités de masse
- **`src/temperature.rs`** : Module de conversion des unités de température
- **`tests/integration_tests.rs`** : Tests d'intégration

### Fonctionnalités Techniques

- **Fonctions pures** : Séparation entre logique de conversion et affichage
- **Gestion d'erreurs robuste** : Utilisation de `Result<T, String>`
- **Structures de données** : Types dédiés pour les résultats de conversion
- **Tests exhaustifs** : Couverture complète avec tests unitaires et d'intégration
- **Documentation intégrée** : Exemples de code testés automatiquement

## Licence

Ce projet est sous **Aucune Licence**. Vous pouvez utiliser et modifier le code à votre propre discrétion.

## Auteurs

- **VIRY Brandon** - *Développeur principal* - [VIRY Brandon](www.linkedin.com/in/brandon-viry-81187b237)

---

## Développement

### Prérequis
- Rust 1.89.0 ou plus récent
- Cargo (inclus avec Rust)

### Commandes de développement
```bash
# Compilation en mode développement
cargo build

# Compilation optimisée
cargo build --release

# Exécution en mode développement
cargo run -- <arguments>

# Formatage du code
cargo fmt

# Vérification du code (linting)
cargo clippy
