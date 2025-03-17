#!/bin/bash

input_file="zbll.txt"
output_file="zbll-inverse.txt"

# Vérifier si le fichier d'entrée existe
if [ ! -f "$input_file" ]; then
    echo "$input_file not found"
    exit 1
fi

> "$output_file"

while IFS= read -r line; do
    processed=()
    for word in $line; do
        if [ "${#word}" -eq 1 ]; then
            clean_word="$word'"
        else
            clean_word="${word//\'/}"
        fi
        processed+=("$clean_word")
    done
    echo "${processed[*]}" | awk '{for(i=NF;i>0;i--) printf "%s%s", $i, (i>1 ? " " : "\n")}' >> "$output_file"
done < "$input_file"

echo "Traitement terminé. Résultats enregistrés dans $output_file"
