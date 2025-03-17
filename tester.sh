#!/bin/bash

# Function to run a single sequence and track the time
run_sequence() {
    local sequence="$1"
    local start_time=$(date +%s%N)
    
    # Vérification du système d'exploitation et exécution de la commande appropriée
    if [[ "$OS" == "Windows_NT" ]]; then
        ./target/release/rubik.exe "$sequence"
    else
        ./target/release/rubik "$sequence"
    fi
    
    local end_time=$(date +%s%N)
    local elapsed_time=$(( (end_time - start_time) / 1000000000 )) # in seconds

    echo "Time: ${elapsed_time} s"
    echo "Sequence: $sequence"
}

# Main script logic
if [[ "$1" == "single" ]]; then
    # Run a single random sequence from the file
    sequence=$(shuf -n 1 sequences.txt)
    run_sequence "$sequence"
else
    echo "Usage: $0 {single}"
    exit 1
fi