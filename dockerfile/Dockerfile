FROM ubuntu:18.04

RUN perl -p -i.bak -e 's%https?://(?!security)[^ \t]+%http://ftp.jaist.ac.jp/pub/Linux/ubuntu/%g' /etc/apt/sources.list``
RUN apt-get clean

RUN apt-get clean
RUN rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y software-properties-common 
RUN apt-get update && apt-get install -y tzdata
RUN add-apt-repository ppa:fish-shell/release-3
RUN add-apt-repository ppa:neovim-ppa/stable
RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get update && apt-get install -y \
    locales \
    wget \
    curl \
    make \
    build-essential \
    libssl-dev \
    zlib1g-dev \
    libbz2-dev \
    libreadline-dev \
    libsqlite3-dev \
    llvm \
    libncurses5-dev \
    libncursesw5-dev \
    xz-utils \
    tk-dev \
    libffi-dev \
    liblzma-dev \
    git \
    neovim \
    fish

RUN rm -rf /root/.cache

RUN locale-gen ja_JP.UTF-8

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /opt/program
