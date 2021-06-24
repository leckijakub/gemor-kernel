FROM debian:buster

RUN apt-get update && apt-get upgrade -y && apt install -y build-essential git \
    bison flex libgmp3-dev libmpc-dev libmpfr-dev texinfo wget grub-common

RUN useradd -d /home/dev -p dev -s /bin/bash -u 1000 -m dev

WORKDIR /home/dev
USER dev
RUN git clone https://gitlab.com/leckijakub/dotfiles.git
RUN /bin/bash ./dotfiles/install
COPY ./build-scripts .
RUN ./build-cross-compiler.sh
RUN echo 'export PATH="$PATH:$HOME/opt/cross/bin"' >> ./.bashrc

ENTRYPOINT [ "/bin/bash" ]
