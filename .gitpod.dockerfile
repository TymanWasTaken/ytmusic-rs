FROM gitpod/workspace-full-vnc
RUN sudo apt-get update && \
    sudo apt-get install -y build-essential && \
    sudo rm -rf /var/lib/apt/lists/*
