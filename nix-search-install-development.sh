src_cmd="reload:nix-search {q}"
default_prompt="Search nix.pkgs > "

cargo run $(# fzf --bind "change:$src_cmd" \
                --bind "start:$src_cmd" \
                --disabled \
                --prompt "$default_prompt" \
                --bind "ctrl-t:transform:[[ \$FZF_PROMPT != \"$default_prompt\" ]] &&
                echo 'rebind(change)+change-prompt($default_prompt)+disable-search' ||
                echo 'unbind(change)+change-prompt(Filtering with fzf > )+enable-search'" \
                | awk '{print $1}')

sudo nixos-rebuild switch
# echo "Installing $package"
# cargo run $package 
  # --preview 'brew i  
  # get fifth column of the output

