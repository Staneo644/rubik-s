#!/bin/bash

#curl "https://www.speedsolving.com/wiki/index.php/Special:MediawikiAlgDB?mode=view&view=default&puzzle=3&group=ZBLL-U&cid=3" | grep "play animation..." | sed -E "s/^.*title='play animation...'>//; s/<\/a>.*$//" > newnew.txt

groups=("ZBLL-U" "ZBLL-T" "ZBLL-L" "ZBLL-H" "ZBLL-Pi" "ZBLL-S" "ZBLL-AS" "PLL" "COLL" "CMLL" "CLL")
webpage="https://www.speedsolving.com/wiki/index.php/Special:MediawikiAlgDB?mode=view&view=default&puzzle=3&group="
cids=(1 1 1 1 1 1 1 1 1 100 1)

output_file="zbll.txt"
reverse_file="zbll-reverse.txt"
folder="zbll-list/speedsolving/"

> "$folder""$output_file"
> "$folder""$reverse_file"

i=0
for group in "${groups[@]}"; do
  echo "<<<<< Scraping $group... >>>>>>"
  cid=${cids[$((i++))]}
  while true; do
    all_lines=$(curl -s "$webpage""$group""&cid=$cid" | \
    grep "play animation..." | sed -E "s/^.*title='play animation...'>//; s/<\/a>.*$//")
    if [[ -z "$all_lines" || "$all_lines" == *"No results found"* ]]; then
      cid=$((cid + 1))
      all_lines=$(curl -s "$webpage""$group""&cid=$cid" | \
      grep "play animation..." | sed -E "s/^.*title='play animation...'>//; s/<\/a>.*$//")
      if [[ -z "$all_lines" || "$all_lines" == *"No results found"* ]]; then
        cid=$((cid - 1))
        echo "Aucune donnée trouvée pour $group, cid $cid"
        break
      fi
    fi
    smallest_line=$(echo "$all_lines" | awk 'length < min_length || NR == 1 {min_length = length; min_line = $0} END {print min_line}')
    processed=()
    for word in $smallest_line; do
        if [ "${#word}" -eq 1 ]; then
            clean_word="$word'"
        else
            clean_word="${word//\'/}"
        fi
        processed+=("$clean_word")
    done
    echo "${processed[*]}" | awk '{for(i=NF;i>0;i--) printf "%s%s", $i, (i>1 ? " " : "\n")}' >> "$folder""$reverse_file"
    echo "$smallest_line" >> "$folder""$output_file"
    cid=$((cid + 1))
  done
done