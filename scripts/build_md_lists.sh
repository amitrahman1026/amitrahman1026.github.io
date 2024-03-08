#!/bin/bash

directory="./assets/blogposts"
markdown_file="./assets/bloglist.md"
title="Blog"

# Start the markdown file with the title
echo "# $title" > "$markdown_file"

urldecode() {
    # URL decode function
    local url_encoded="${1//+/ }"
    printf '%b' "${url_encoded//%/\\x}"
}

# Collect an array of filenames and dates
files_and_dates=()
for file in "$directory"/*; do
    filename=$(basename "$file")
    # Extract the date in 'YYYY-MM-DD' format
    date=$(echo "$filename" | sed -n 's/(\([0-9-]*\)).*/\1/p')

    # Append the date and filename to the array
    files_and_dates+=("$date $filename")
done

# Sort the files by date with the latest first
IFS=$'\n' sorted_files=($(sort -r <<<"${files_and_dates[*]}"))
unset IFS

# Generate the markdown links with dates
for entry in "${sorted_files[@]}"; do
    date=$(echo "$entry" | awk '{print $1}')
    filename=$(echo "$entry" | awk '{print $2}')

    encoded_title=$(echo "$filename" | sed -e 's/^[^(]*[(][^)]*[)]_//' -e 's/.md$//')
    display_name=$(urldecode "$encoded_title")
    display_name=$(echo "$display_name" | tr '_' ' ')
    new_file_path="#/blogposts/$filename"

    # Convert the date to 'd M Y' format for display
    # macOS date command version, trim the timestamp if it exists
    date_for_formatting=$(echo "$date" | cut -d'-' -f1-3)
    formatted_date=$(date -j -f "%Y-%m-%d" "$date_for_formatting" "+%d %b %Y")

    # Create a clickable link for the file with the formatted date
    link="[${display_name}]($new_file_path) <span style=\"float: right;\">${formatted_date}</span>"

    # Append to the markdown file
    echo "- $link" >> "$markdown_file"
done

echo "File list saved in $markdown_file"

