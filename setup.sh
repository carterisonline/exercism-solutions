#!/bin/bash

# Run `source setup.sh` in order to update environment variables

eval $(gp env -e)

# Temporary Directory for downloaded tarballs & programs to reside
mkdir temp
mkdir .programs
cd temp

# Download the Exercism Client & Julia Runtime
wget https://github.com/exercism/cli/releases/download/v3.0.13/exercism-3.0.13-linux-x86_64.tar.gz
wget https://julialang-s3.julialang.org/bin/linux/x64/1.5/julia-1.5.2-linux-x86_64.tar.gz

# Extract tarball and move program out of temp directory
tar -xf exercism-3.0.13-linux-x86_64.tar.gz
tar -xzvf julia-*
rm julia-1.5.2-linux-x86_64.tar.gz

cd ..
mv temp/exercism .programs/exercism
mv temp/julia-* .programs

# Clean it up
rm temp -R

# Export to the `$PATH` variable: !ONLY WORKS WHEN RUNNING `source exercism.sh`!
export PATH=$PATH:$GITPOD_REPO_ROOT/.programs
export PATH=$PATH:$GITPOD_REPO_ROOT/.programs/julia-1.5.2/bin

cd .programs

# Allow Execution of the Exercism CLI
chmod +x exercism
chmod +x julia-*/bin/julia

# Provide Exercism with the token; run `gp env EXERCISM_KEY=(your key here)` to sync with your user profile
./exercism configure --token=$EXERCISM_KEY
./exercism configure --workspace=..

cd ..