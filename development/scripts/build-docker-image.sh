#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
VERBOSE=false

DOCKER_IMAGE_NAME="dev-env"
DOCKER_IMAGE_TAG="v1.0.0"

verbose(){
    light_blue=$(tput setaf 14)
    reset=$(tput sgr0)
    if [ "$VERBOSE" = true ]; then
        echo -e "${light_blue}[VERB]: $*${reset}"
    fi
}

warn(){
    yellow=$(tput setaf 3)
    reset=$(tput sgr0)
    echo -e "${yellow}[WARN]: $*${reset}"
}

info(){
    green=$(tput setaf 2)
    reset=$(tput sgr0)
    echo -e "${green}[INFO]: $*${reset}"
}

errorCheck() {
    local error_code="$?"
    local error_msg="$1"
    red=$(tput setaf 1)
    reset=$(tput sgr0)
    if [ "$error_code" -ne 0  ]; then
        echo -e "${red}[ERROR]: ${error_msg} : ${error_code}${reset}"
        cleanup
        exit 1
    fi
}

printHelp() {
    echo "$0 - dockerized develop environment build script"
    echo "-h|--help     print help"
    echo "-v|--verbose  be verbose"
}

cleanup() {
    echo "Cleanup"
}

checkRequirements() {
    command -v docker
    errorCheck "Docker not installed"
}

buildDockerizedEnv() {
    docker build ${SCRIPT_DIR}/../docker/image/ -t ${DOCKER_IMAGE_NAME}:${DOCKER_IMAGE_TAG}
    errorCheck "Error building docker image ${DOCKER_IMAGE_NAME}"
}

checkForImage() {
    docker inspect ${DOCKER_IMAGE_NAME}:${DOCKER_IMAGE_TAG} &> /dev/null
    return $?
}


while [[ $# -gt 0 ]]
do
arg="$1"

case $arg in
    -h|--help)
        print_help
        exit 0
        ;;
    -v|--verbose)
        VERBOSE=true
        shift # past argument
        ;;

esac
done


verbose "Searching for docker image"
checkForImage
result=$?

if [ $result -eq 0 ]; then
    verbose "Docker image found."
    info "Docker image already exists."
else
    verbose "Docker image not found."
    info "Building docker image..."
    buildDockerizedEnv
fi
