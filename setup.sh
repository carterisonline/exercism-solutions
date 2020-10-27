#!/bin/bash

# Run `source setup.sh` in order to update environment variables

eval $(gp env -e)

# Temporary Directory for downloaded tarballs & programs to reside
mkdir temp
cd temp

# Download the Exercism Client
wget https://github.com/exercism/cli/releases/download/v3.0.13/exercism-3.0.13-linux-x86_64.tar.gz

# Extract tarball and move program out of temp directory
tar -xf exercism-3.0.13-linux-x86_64.tar.gz
mv exercism ../exercism

cd ..

# Clean it up
rm temp -R
mkdir programs
mv exercism programs/exercism

# Export to the `$PATH` variable: !ONLY WORKS WHEN RUNNING `source exercism.sh`!
export PATH=$PATH:$GITPOD_REPO_ROOT/programs

cd programs

# Allow Execution of the Exercism CLI
chmod +x exercism

# Provide Exercism with the token; run `gp env EXERCISM_KEY=(your key here)` to sync with your user profile
./exercism configure --token=$EXERCISM_KEY
./exercism configure --workspace=..

cd ..