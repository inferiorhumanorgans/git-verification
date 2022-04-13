#!/usr/bin/env bash

set -e
. _common.bash

WORKSPACE=${WORKSPACE:="revert"}

init

sanity_check "${WORKSPACE}"

create_repo "${WORKSPACE}"

pushd "${WORKSPACE}"

touch 1 2 3
git add 1
git commit -m 1 1
git add 2
git commit -m 2 2
git add 3
git commit -m 3 3
git revert --no-commit HEAD~1

finish

archive "${WORKSPACE}"

cleanup "${WORKSPACE}"
