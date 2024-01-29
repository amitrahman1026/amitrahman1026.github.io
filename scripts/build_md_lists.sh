#!/bin/bash

directory="./assets/blogposts"
markdown_file="./assets/bloglist.md"
title="Blog"

echo "# $title" > "$markdown_file"

urldecode() {
    # URL decode function
    local url_encoded="${1//+/ }"
    printf '%b' "${url_encoded//%/\\x}"
}

for file in "$directory"/*; do
    filename=$(basename "$file")
    encoded_title=$(echo "$filename" | sed -e 's/^[^(]*[(][^)]*[)]_//' -e 's/.md$//')

    # URL-decode the filename for display purposes
    display_name=$(urldecode "$encoded_title")

    # Replace underscores with spaces
    display_name=$(echo "$display_name" | tr '_' ' ')

    # Create the new file path for the link
    new_file_path="#/blogposts/$filename"

    # Create a clickable link for the file
    link="[${display_name}]($new_file_path)"
    
    echo "- $link" >> "$markdown_file"
done

echo "File list saved in $markdown_file"
