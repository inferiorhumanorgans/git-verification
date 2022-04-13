#!/usr/bin/env bash

set -e
. _common.bash

WORKSPACE=${WORKSPACE:="cherry-pick"}

init

sanity_check "${WORKSPACE}"

create_repo "${WORKSPACE}"

pushd "${WORKSPACE}"

touch 1
git add 1
git commit -m 1 1
git checkout -b other-branch
echo other-branch > 1
git add 1
git commit -m 1.other 1
git checkout master
echo master > 1
git add 1
git commit -m 1.master 1
# This should fail and leave us in a cherry-pick state
git cherry-pick other-branch || true

finish

archive "${WORKSPACE}"

cleanup "${WORKSPACE}"
