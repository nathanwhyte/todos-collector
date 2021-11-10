#!/usr/bin/env bash

# find all of the Org files within a given root directory
# redirect output into temp file

TMP_ORG_FILE=$(mktemp /tmp/todo-collector-XXXXX)
TMP_TODO_FILE=$(mktemp /tmp/todo-collector-XXXXX)

FIND_IN_DIR=''

function read_lines () {
  if [ -f todo-files.txt ]; then
    rm -f todo-files.txt
  fi

  while read -r LINE; do
    if grep -q "* TODO" "$LINE"; then
      echo $LINE >> TMP_TODO_FILE
    fi
  done < TMP_ORG_FILE
}

function find_no_given_dir () {
  find ~/Dropbox/Org -iname "*.org" > TMP_ORG_FILE
  read_lines
}

function find_in_given_dir () {
  find $FIND_IN_DIR -iname "*.org" > TMP_ORG_FILE
  read_lines
}

if [ $# -eq 0 ]; then
  find_no_given_dir
else
  while getopts "d" FLAG; do
    case $FLAG in
      d) FIND_IN_DIR=$OPTARG
         find_in_given_dir
         ;;
    esac
  done
fi

echo "list of todo files in /tmp : $TMP_TODO_FILE"
