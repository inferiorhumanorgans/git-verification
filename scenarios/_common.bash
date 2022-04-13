# Exporting variables from a function seems gross
function init {
    # Clear the directory stack
    dirs -c

    export GIT_CONFIG="${PWD}/gitconfig"
    export GIT_CONFIG_GLOBAL="${GIT_CONFIG}"

    git config advice.statusHints false
    git config advice.resolveConflict false
    git config advice.commitBeforeMerge false
    git config advice.skippedCherryPicks false

    git config init.defaultBranch master
    git config user.name "User Name"
    git config user.email user@example.com
}

function sanity_check {
    if [ "x${1}" = "x" ]; then
        echo "Workspace not set"
        exit 1
    fi

    if [[ "${1}" =~ \.\. ]]; then
        echo "Workspace unsafe: ${1}"
        exit 1
    fi
}

function create_repo {
    if [ "x${1}" = "x" ]; then
        echo "create_repo() must be called with a non-empty workspace name"
        exit 1
    fi

    git init "${1}"

    # Save the versions of the tools used just in case we need to debug anything
    git --version > "${1}/.git/_git_version"
    gtar --version | head -1 > "${1}/.git/_gnu_tar_version"
    gsed --version | head -1 > "${1}/.git/_gnu_sed_version"
    xz --version > "${1}/.git/_xz_version"
}

function cleanup {
    if [ "x${1}" = "x" ]; then
        echo "cleanup() must be called with a non-empty workspace name"
        return 1
    fi

    rm -rf "${1}"
    rm -f "${GIT_CONFIG}"
}

function finish {
    cd "$(dirs -l -0)" && dirs -c
}

function archive {
    if [ "x${1}" = "x" ]; then
        echo "cleanup() must be called with a non-empty workspace name"
        return 1
    fi

    mkdir archive || true
    gtar -vc --owner=0 --group=0 --no-same-owner --no-same-permissions -f "${WORKSPACE}.tar" "${WORKSPACE}/"
    xz -z3 "${WORKSPACE}.tar"
    mv -i "${WORKSPACE}.tar.xz" archive/
}
