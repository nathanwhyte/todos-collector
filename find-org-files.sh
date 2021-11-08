#!/usr/bin/env bash

# find all of the Org files within a given root directory
# redirect output into temp file

function read_lines () {
  if [ -f todo-files.txt ]; then
    rm -f todo-files.txt
  fi

  while read -r line; do
    if grep -q "* TODO" "$line"; then
      echo $line >> todo-files.txt
    fi
  done < org-files.txt
}

function find_no_root_dir () {
  find ~/Dropbox/Org -iname "*.org" > org-files.txt
  read_lines
}

function find_root_dir () {
  find $FIND_ROOT_DIR -iname "*.org" > org-files.txt
  read_lines
  rm -f org-files.txt
  rm -f ~/Dropbox/Org/planner/todo-list.org
  exit 0
}

while getopts d: flag; do
  case "${flag}" in
    d) FIND_ROOT_DIR=${OPTARG}
       find_root_dir
       ;;
  esac
done

find_no_root_dir
rm -f ~/Dropbox/Org/planner/todo-list.org
rm -f org-files.txt
