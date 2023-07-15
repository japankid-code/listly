#!/bin/sh

if [ -z "$1" ]; then
  echo "Provide migration Name: create-migration My-New-Migration"
  exit 1
fi

mydate=$(date +"%Y-%m-%d")
mytime=$(date +"%H%M")

echo "creating migration..."
diesel migration generate "$1"
echo "$mydate"_"$mytime": migration created."