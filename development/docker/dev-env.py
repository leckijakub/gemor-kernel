#!/usr/bin/env python3

import os
import sys
import itertools
import threading
import argparse
import docker
import functools
import asyncio
from io import BytesIO
import aiodocker

SCRIPT_DIR = os.path.dirname(os.path.realpath(__file__))
MNT_DIR = "/mnt"
IMAGE_NAME = "dev-env"
CONTAINER_NAME = "dev-env-v1"


async def loading_animation(text):
    frames="/-\|"
    for frame in itertools.cycle(frames):
        sys.stdout.write('\r'+text+frame)
        sys.stdout.flush()
        await asyncio.sleep(0.2)

async def docker_build(dockerfile_path, tag):
    docker_client = aiodocker.Docker()
    dockerfile = ""
    with open(dockerfile_path,"r") as file:
        f = BytesIO(file.read().encode("utf-8"))
        tar_obj = aiodocker.utils.mktar_from_dockerfile(f)
        await docker_client.images.build(fileobj=tar_obj, encoding="gzip", tag=IMAGE_NAME)
        await docker_client.close()

def get_args():
    parser = argparse.ArgumentParser(description="Script tor run gemor develop environment")
    parser.add_argument("-v",dest="volumes", action='append', help=f"mount host volume into container under {MNT_DIR}'")
    return parser.parse_args()

async def main():
    args = get_args()
    docker_opt = ""
    docker_client = docker.from_env()
    volumes = ""
    # parse volumes
    if args.volumes is not None:
        for vol in args.volumes:
            if vol[-1] == '/':
                vol = vol[0:-1]
            volumes += f"-v {os.path.realpath(vol)}:{MNT_DIR}/{os.path.basename(vol)}"
        print(volumes)
    # get images name
    images = docker_client.images.list()
    imagenames = set(functools.reduce(lambda x, y: x + y, (img.tags for img in images)))
    # look for target image
    if f"{IMAGE_NAME}:latest" not in imagenames:
        # build image
        building = asyncio.create_task(docker_build(f"{SCRIPT_DIR}/image/Dockerfile", IMAGE_NAME))
        loading = asyncio.create_task(loading_animation("Building..."))
        await building
        loading.cancel()
        print()
    os.system(f"docker run {volumes} --name {CONTAINER_NAME} -it --rm {IMAGE_NAME}")

if __name__ == "__main__":
    asyncio.run(main())
