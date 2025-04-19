#!/bin/bash
set -e

# Create data directory
mkdir -p data

echo "Checking for Kaggle CLI..."
if ! command -v kaggle &> /dev/null; then
    echo "Kaggle CLI not found. Please install with 'pip install kaggle' and configure your API key."
    echo "For more information, see: https://github.com/Kaggle/kaggle-api"
    exit 1
fi

echo "Downloading Dream Dictionary dataset..."
kaggle datasets download -d yuvrajsanghai/dream-dictionary -p data

echo "Downloading Dictionary of Dreams dataset..."
kaggle datasets download -d manswad/dictionary-of-dreams -p data

echo "Extracting datasets..."
unzip -o data/dream-dictionary.zip -d data
unzip -o data/dictionary-of-dreams.zip -d data

echo "Cleaning up zip files..."
rm data/dream-dictionary.zip
rm data/dictionary-of-dreams.zip

echo "Checking the downloaded files..."
ls -la data/

echo "Download and extraction complete. You can now run the seeder with:"
echo "cargo run --bin dream_seeder" 