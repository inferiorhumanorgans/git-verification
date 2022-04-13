#!/usr/bin/env bash

set -e
. _common.bash

WORKSPACE=${WORKSPACE:="untracked-file"}

init

sanity_check "${WORKSPACE}"

create_repo "${WORKSPACE}"

pushd "${WORKSPACE}"

touch 1

finish

archive "${WORKSPACE}"

cleanup "${WORKSPACE}"
