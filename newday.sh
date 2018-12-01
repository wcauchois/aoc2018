#!/bin/bash
set -e

if [ "$#" -lt 1 ]; then
  echo "usage: $0 DAYNUM"
  exit 1
fi

dest="day$1"
if [ -e "$dest" ]; then
  echo "The directory '$dest' already exists"
  exit 1
fi

mkdir $dest
echo "Created $dest"

cat > $dest/main.py <<EOF
#!/usr/bin/env PYTHONPATH=.. python
from lib import *

EOF
chmod +x $dest/main.py
echo "Created $dest/main.py"

echo "Done."
