# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.191.0/containers/rust/.devcontainer/base.Dockerfile
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && apt-get update && apt-get install -y nodejs
# add the vue cli service global binary
RUN npm install -g @vue/cli @vue/cli-service-global

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>
