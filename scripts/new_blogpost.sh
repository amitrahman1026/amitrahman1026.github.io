#!/bin/bash

working_directory="./assets/blogposts"
date=$(date +%Y-%m-%d-%s)

read -p "Enter the article title: " title

# URL encode the title
encoded_title=$(jq -nr --arg title "$title" '$title|@uri')

# Create a filename with the encoded title
file_name="(${date})_${encoded_title}.md"

cd "$working_directory" || exit
echo "# $title" > "$file_name"
echo "Date: $date" >> "$file_name"
echo "" >> "$file_name"

echo "Template markdown file \"$file_name\" created in \"$working_directory\"."
