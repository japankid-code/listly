#!/bin/sh

mydate=$(date +"%Y-%m-%d")
mytime=$(date +"%H%M")

echo "Choose an option:"
echo "1. run"
echo "2. redo"

read -p "Type 1 or 2, then press Enter: " choice

if [ "$choice" -eq 1 ]; then
  echo "applying migration..."
  diesel migration run
elif [ "$choice" -eq 2 ]; then
  echo "reverting migration..."
  diesel migration redo
else
  echo "Invalid choice. Exiting."
  exit 1
fi

echo "$mydate"_"$mytime": migration completed."