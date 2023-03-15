#!/bin/bash

src_dir="src/entity"
test_dir="tests/entity"

# Créer le répertoire de destination s'il n'existe pas
mkdir -p "$test_dir"

cp -R "$src_dir"* "tests"
