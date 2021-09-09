#!/usr/bin/env bash


MAIN_PATH=files
DIRECTORY_NEED=(quotes movies)

echo "Starting..."
echo ""
echo "Checking for Cargo..."
(command -v cargo >/dev/null 2>&1 && echo "Cargo is installed" ) || { echo >&2 "Cargo is not installed.\nAborting."; exit 1;}
echo ""
echo "Setting up..."
echo ""

if [[ -d "${MAIN_PATH}" ]]; then
    echo "${MAIN_PATH}/ exits..."
else
    echo "Creating ${MAIN_PATH}..."
    mkdir "${MAIN_PATH}"
fi

cd "${MAIN_PATH}"
echo "${MAIN_PATH}/..."
for directory_name in "${DIRECTORY_NEED[@]}"; do
    if [[ -d "${directory_name}" ]]; then
        echo "${MAIN_PATH}/${directory_name}/ exits..."
    else
        echo "Creating ${MAIN_PATH}/${directory_name}..."
        mkdir "$directory_name"
    fi
done
cd ..
echo ""
echo "Cargo build..."
cargo build
