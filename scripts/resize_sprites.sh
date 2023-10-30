#!/bin/bash

# Check if a directory was provided as an argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

# Directory path
DIR="$1"

# Create a temporary file name
TMP_FILE=$(mktemp "${TMPDIR:-/tmp}/tempfile.XXXXXXXXXX.png")

# Find and resize all .png files recursively
find "$DIR" -type f -name "*.png" | while IFS= read -r file; do
    if [ ! -f "$file" ]; then
        echo "Warning: Unable to find file '$file'. Skipping."
        continue
    fi

    echo "Resizing $file..."
    ffmpeg -i "$file" -vf "scale=100:100:flags=neighbor" -f image2 "$TMP_FILE"
    
    # Check if ffmpeg was successful
    if [ ! -f "$TMP_FILE" ]; then
        echo "Warning: Resizing failed for '$file'. Skipping."
        continue
    fi
    
    # Overwrite the original file
    mv "$TMP_FILE" "$file"
done

echo "Process completed!"

