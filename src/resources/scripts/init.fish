function goto
  set -l path (command quickclip goto $argv)
  cd -- "$path"
end
