#!/usr/bin/env zsh

# Abort on error.
set -e

# Build project.
yarn build

# Set dist/ to current directory.
cd dist

# Push the web app to the host!
git init
git add -A
git commit -m "Deploy"
git push -f git@github.com:felixnaredi/deja-vu.git main:gh-pages

cd -
