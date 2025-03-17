#!/bin/bash

groups=("ZBLL" "PLL" "COLL" "CMLL" "OH-CMLL")

website="http://cubingapp.com/algorithms/"
output_file="zbll.txt"
reverse_file="zbll-reverse.txt"
folder="zbll-list/cubing-app/"

> "$folder""$output_file"
> "$folder""$reverse_file"

i=0
for group in "${groups[@]}"; do
  echo "<<<<< Scraping $group... >>>>>>"
  curl -s "$website""$group/" | grep "<p>&#8226;" | sed -E 's/^.*<p>&#8226;//; s#</p>[[:space:]]*$##g' | sed -E 's/^\s*<p>//; s/<.*$//' | sed 's/[()]//g' |
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
    echo "$line" >> "$folder""$output_file"
    echo "${processed[*]}" | awk '{for(i=NF;i>0;i--) printf "%s%s", $i, (i>1 ? " " : "\n")}' >> "$folder""$reverse_file"
  done
done