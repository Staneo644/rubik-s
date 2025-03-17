#!/bin/bash

# Définition des mouvements possibles
MOVES=(U U' U2 D D' D2 L L' L2 R R' R2 F F' F2 B B' B2)

# Génération d'une séquence de 15 mouvements aléatoires
SEQUENCE=""
for i in {1..15}; do
    SEQUENCE+="${MOVES[RANDOM % ${#MOVES[@]}]} "
done

# Affichage de la séquence générée
echo "Séquence générée : $SEQUENCE"

# Mesure du temps d'exécution
START_TIME=$(date +%s%N)
#./target/release/rubik "$SEQUENCE"
./target/release/rubik.exe "$SEQUENCE"
END_TIME=$(date +%s%N)

# Calcul et affichage du temps écoulé
ELAPSED_TIME=$(( (END_TIME - START_TIME) / 1000000000 ))
echo "Temps d'exécution : ${ELAPSED_TIME} s"