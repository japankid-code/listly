#!/bin/bash

if [ -z "$1" ]; then
  echo "Choose an option:"
  echo "1. run"
  echo "2. redo"
  echo "3. revert"
  read -p "Type 1, 2 or 3, then press Enter: " choice
else
  choice=$1
fi

mydate=$(date +"%Y-%m-%d")
mytime=$(date +"%H%M")

if [ "$choice" -eq 1 ]; then
  echo "applying migration..."
  diesel migration run
elif [ "$choice" -eq 2 ]; then
  echo "redoing migration..."
  diesel migration redo
elif [ "$choice" -eq 3 ]; then
  echo "reverting migration..."
  diesel migration revert
else
  echo "Invalid choice. Exiting."
  exit 1
fi

echo "$mydate"_"$mytime": migration completed.