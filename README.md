
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
  - [Exemple de Résultat](#exemple-de-résultat)
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

Convertir 32 degrés Fahrenheit en Celsius :
```bash
./convertisseur_unites temperature --from F --value 32
```

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

**Conversion de température** :
```bash
$ ./convertisseur_unites temperature --from C --value 25
25 C = 77 F
```

## Licence

Ce projet est sous **Aucune Licence**. Vous pouvez utiliser et modifier le code à votre propre discrétion.

## Auteurs

- **VIRY Brandon** - *Développeur principal* - [VIRY Brandon](www.linkedin.com/in/brandon-viry-81187b237)

