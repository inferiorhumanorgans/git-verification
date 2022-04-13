#!/usr/bin/env bash

set -e
. _common.bash

WORKSPACE=${WORKSPACE:="rebase-interactive"}

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
# This doesn't work with older git versions
EDITOR="gsed -i.bak -z 's/pick/edit/2'" git rebase --rebase-merges --interactive HEAD~2

finish

archive "${WORKSPACE}"

cleanup "${WORKSPACE}"
