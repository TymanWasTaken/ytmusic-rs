FROM gitpod/workspace-full-vnc
RUN sudo apt-get update && \
    sudo apt-get install -y libgtk-4-dev build-essential && \
    sudo rm -rf /var/lib/apt/lists/*
