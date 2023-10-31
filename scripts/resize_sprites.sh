#!/bin/bash

# Check if a directory was provided as an argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

# Directory path
DIR="$1"

# Find and resize all .png files recursively
find "$DIR" -type f -name "*.png" | while IFS= read -r file; do
    if [[ ! "$file" == .* ]]; then
        file=".$file"
    fi

    if [ ! -f "$file" ]; then
        echo "Warning: Unable to find file '$file'. Skipping."
        continue
    fi

    # Extract filename from the path
    FILENAME=$(basename "$file")

    # Create a temporary file name in the /tmp directory with the same filename as the original
    TMP_FILE="${TMPDIR:-/tmp}/$FILENAME"

    echo "Resizing $file..."
    ffmpeg -y -i "$file" -vf "scale=100:100:flags=neighbor" "$TMP_FILE"
    
    # Check if ffmpeg was successful
    if [ $? -ne 0 ]; then
        echo "Warning: Resizing failed for '$file'. Skipping."
        rm -f "$TMP_FILE"  # Remove the potentially corrupted temp file
        continue
    fi
    
    # Overwrite the original file
    mv "$TMP_FILE" "$file"
done

echo "Process completed!"

