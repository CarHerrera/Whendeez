#!/bin/bash

S1="Excalidraw"
S2="Images"
S3="Maps"
S4="Nonmd"
S5="Templates"
for d in */; do
    # Will print */ if no directories are available
    s="${d%/}"
    if [[ "$s" != "$S1" && "$s" != "$S2" && "$s" != "$S3" && "$s" != "$S4" && "$s" != "$S5" ]]; then
         mkdir "$s/Tips & Tricks"
         mkdir "$s/Execs"
         mkdir "$s/images"
    fi
done
