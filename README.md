# quickclip
A directory book marker written in rust 🦀!  
## Installation
with cargo run : `cargo install quickclip`
## Usage
- add new mark with `quickclip`  
- list all marks with `quickclip list`  
- remove a clip with ` quickclip remove {NAME}`
- print a clip path with `quickclip goto {NAME}`
### Tips
- Saving quickclip as an alias for your shell such as `alias qc = quickclip`
- create a shell goto function to cd to the output of the quickclip goto command
ie (in fish) :
```
# set to cd using bookmarks
function goto
  set -l path (command quickclip goto $argv)
  cd -- "$path"
end

```
## Roadmap
Eventually I want to make a TGUI for selecting a mark to goto. but this porject is finished for the most part as it
was mostly intended to learn about the process of creating a rust package.  
