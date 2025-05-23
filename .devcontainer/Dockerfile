FROM mcr.microsoft.com/devcontainers/base:ubuntu

# User

USER vscode
ENV HOME="/home/vscode"

# Env

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="$HOME/.local/bin:$PATH"

# Shell

# Fish
RUN sudo apt-get update \
        && sudo apt-get install -y fish \
        && mkdir -p "$HOME/.config/fish/completions"

# Starship
RUN curl -sS https://starship.rs/install.sh | sh -s -- --yes \
        && echo "starship init fish | source" >> "$HOME/.config/fish/config.fish"

# Stack

# mise-en-place
RUN curl https://mise.run | sh \
        && echo "mise activate fish | source" >> ~/.config/fish/config.fish \
        && PATH="$HOME/.local/bin:$PATH" \
        && mise use -g usage \
        && mise completion fish > ~/.config/fish/completions/mise.fish
