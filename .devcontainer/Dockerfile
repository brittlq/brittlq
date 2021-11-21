# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.191.0/containers/rust/.devcontainer/base.Dockerfile
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

ARG USERNAME=vscode

# Precreate a few directories that hopefully prevent reinstalls/redownloads
RUN mkdir -p /home/$USERNAME/.vscode-server/extensions \
        /home/$USERNAME/.vscode-server-insiders/extensions \
        /home/$USERNAME/.vscode-server/data/User/globalStorage \
        /home/$USERNAME/.vscode-server-insiders/data/User/globalStorage \
    && chown -R $USERNAME \
        /home/$USERNAME/.vscode-server \
        /home/$USERNAME/.vscode-server-insiders \
        /home/$USERNAME/.vscode-server/data/User/globalStorage \
        /home/$USERNAME/.vscode-server-insiders/data/User/globalStorage
    

RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && apt-get update && apt-get install -y nodejs
# add the vue cli service global binary
RUN npm install -g @vue/cli @vue/cli-service-global

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
