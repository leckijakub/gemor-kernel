#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
VERBOSE=false

DOCKER_IMAGE_NAME="dev-env"
DOCKER_IMAGE_TAG="v1.0.0"
VOLUMES=
CONTAINER_NAME=gemor-dev-env
MNT_DIR="/mnt"

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

checkForImage() {
    docker inspect ${DOCKER_IMAGE_NAME}:${DOCKER_IMAGE_TAG} &> /dev/null
    return $?
}

enterContainer(){

    checkForImage
    errorCheck "Image not Found."
    verbose "volumes \n ${VOLUMES}"
    docker run ${VOLUMES} --name ${CONTAINER_NAME} -it --rm ${DOCKER_IMAGE_NAME}
}

while [[ $# -gt 0 ]]
do
arg="$1"

case $arg in
    -h|--help)
        print_help
        exit 0
        ;;
    -v|--volume)
        ABS_PATH=$(readlink -f $2)
        VOLUMES="-v ${ABS_PATH}:${MNT_DIR}/$(basename ${ABS_PATH}) ${VOLUMES}"
        shift
        shift
        ;;
    --verbose)
        VERBOSE=true
        shift # past argument
        ;;
    *)
        print_help
        exit 1
        ;;

esac
done

enterContainer
