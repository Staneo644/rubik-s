#!/bin/bash

# Définition des mouvements possibles
MOVES=("U" "U'" "U2" "D" "D'" "D2" "L" "L'" "L2" "R" "R'" "R2" "F" "F'" "F2" "B" "B'" "B2")

# Initialize counters
good_count=0
bad_count=0
total_good_count=0
sequence_size=1

# Loop until 500 good sequences are found
while [ $total_good_count -lt 150 ]; do
    # Génération d'une séquence de mouvements aléatoires
    SEQUENCE=""
    for ((i=1; i<=sequence_size; i++)); do
        SEQUENCE+="${MOVES[RANDOM % ${#MOVES[@]}]} "
    done

    # Affichage de la séquence générée
    echo "Séquence générée : $SEQUENCE"

    # Mesure du temps d'exécution
    START_TIME=$(date +%s%N)
    # Vérification du système d'exploitation et exécution de la commande appropriée
    if [[ "$OS" == "Windows_NT" ]]; then
        ./target/release/rubik.exe "$SEQUENCE"
    else
        ./target/release/rubik "$SEQUENCE"
    fi
    END_TIME=$(date +%s%N)

    # Calcul du temps écoulé
    ELAPSED_TIME=$(( (END_TIME - START_TIME) / 1000000000 ))

    # Affichage du temps écoulé
    echo "Temps d'exécution : ${ELAPSED_TIME} s"

    # Write the sequence to the appropriate file based on the elapsed time
    if [ $ELAPSED_TIME -lt 5 ]; then
        echo "$SEQUENCE" >> sequences.txt
        good_count=$((good_count + 1))
        total_good_count=$((total_good_count + 1))
    else
        bad_count=$((bad_count + 1))
    fi

    # Increment sequence size for every 10 sequences found
    if [ $good_count -ge 10 ]; then
        good_count=0
        sequence_size=$((sequence_size + 1))
        if [ $sequence_size -gt 15 ]; then
            sequence_size=1
        fi
    fi
done

echo "Found 300 sequences."