#!/bin/bash

function getVersion()
{
    file="./Cargo.toml"
    local line=$(grep "version = " $file)
    version=$(echo $line | cut -d'=' -f 2)
    version=$(echo $version | cut -d'"' -f 2)
    local line=$(grep "name = " $file)
    name=$(echo $line | cut -d'=' -f 2)
    name=$(echo $name | cut -d'"' -f 2)
    local line=$(grep "authors = " $file)
    author=$(echo $line | cut -d'=' -f 2)
    author=$(echo $author | cut -d'"' -f 2)
}

function getVersionParts()
{
    major=$(echo $version | cut -d'.' -f 1)
    minor=$(echo $version | cut -d'.' -f 2)
    patch=$(echo $version | cut -d'.' -f 3)
}

function updateVersion()
{
    local action="$1"
    if [[ $action == major ]]
    then
        major=$(echo $major + 1 | bc)
        minor=0
        patch=0
    elif [[ $action == minor ]]
    then
        minor=$(echo $minor + 1 | bc)
        patch=0
    elif [[ $action == patch ]]
    then
        patch=$(echo $patch + 1 | bc)
    fi
    new_version="$major.$minor.$patch"
}

function addLine()
{
    echo -e "$1\n$(cat $log_output)" > $log_output
}

function gitLog()
{
    log_output="./debian/changelog"

    if [[ $version == "0.0.0" ]]
    then
        gitlog=$(git log --oneline)
    else
        gitlog=$(git log v$version.. --oneline)
    fi

    IFS=$'\n'

    addLine ""
    addLine " -- $author  $(date -R)"
    addLine ""
    for item in $gitlog
    do
        addLine "  * $item"
    done
    addLine ""
    addLine "$name ($new_version) unstable; urgency=low"
}

if [[ $1 == major ]] || [[ $1 == minor ]] || [[ $1 == patch ]]
then
    getVersion
    getVersionParts
    updateVersion $1
    sed -i "0,/version/ s/version = \"$version/version = \"$new_version/" $file
    #sed -i "s/APP_VERSION: \"$version/APP_VERSION: \"$new_version/" "./.github/workflows/build.yml"
    #sed -i "s/$name_$version/$name_$new_version/" "./README.md"
    #sed -i "s/Build\/v$version/Build\/v$new_version/" "./README.md"
    #gitLog
    echo $new_version
elif [[ $1 == get ]]
then
    getVersion
    echo $version
else
    echo "Usage: bump-version.sh major|minor|patch|get"
fi



