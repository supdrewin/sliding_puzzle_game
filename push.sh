#!/bin/zsh

export LANG=en_US.UTF-8 TZ=UTC

git add .
git commit -m "$(date)"
git push -u origin web
