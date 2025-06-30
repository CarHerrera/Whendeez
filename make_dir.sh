#!/bin/bash

S1="Excalidraw"
S2="Images"
for d in */; do
    # Will print */ if no directories are available
    s="${d%/}"
    if [[ "$s" != "$S1" && "$s" != "$S2" ]]; then
         mkdir "$s/$s Nades"
    fi
done
