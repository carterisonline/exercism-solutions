#!/bin/bash

# Run `source get-exercism.sh` in order to update environment variables

eval $(gp env -e)

mkdir temp
cd temp

wget https://github.com/exercism/cli/releases/download/v3.0.13/exercism-3.0.13-linux-x86_64.tar.gz
tar -xf exercism-3.0.13-linux-x86_64.tar.gz
mv exercism ../exercism

cd ..
rm temp -R

mkdir programs
mv exercism programs/exercism
export PATH=$PATH:$GITPOD_REPO_ROOT/programs

cd programs
chmod +x exercism
./exercism configure --token=$EXERCISM_KEY
./exercism configure --workspace=..

cd ..