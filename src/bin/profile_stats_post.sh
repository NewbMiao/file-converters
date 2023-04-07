#! /bin/bash
set -e # Enable errexit

pname=${1:ProfileStatsSeedBatch}
echo "use phinx name: "$pname

# do migration...
