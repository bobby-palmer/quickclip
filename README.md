# Quickclip 📎
![](https://github.com/bobby-palmer/quickclip/main/.github/workflows/rust.yml/badge.svg)  
A directory book marker written in rust 🦀!  
## Demo  
![](https://github.com/bobby-palmer/quickclip/blob/main/demo.gif)
## Installation
with cargo run : `cargo install quickclip`  
then add your respective shells startup script:
<details closed>
  <summary>fish</summary>
  <br>
  add `quickclip init fish | source` to your config file.  
  this is usually in fish.config  
</details>
<details closed>
  <summary>zsh</summary>
  <br>  
  add `eval "$(quickclip init zsh)"` to your config file.  
  this is usually in .zshrc  
</details>
<details closed>
  <summary>bash</summary>
  <br>
  add `eval "$(quickclip init bash)"` to your config file.  
  this is usually in .bashrc  
</details>  

## Usage
- add new mark with `quickclip {NAME}`
  - if NAME is not specified, it will default to the folder name.  
- list all marks with `quickclip list`  
- remove a clip with `quickclip remove {NAME}`
- go to a saved directory with `goto {NAME}`
  - if NAME is un specified it will open a TUI with all your saved directories  
### Tips
- Saving quickclip as an alias for your shell such as `alias qc = quickclip`
## Roadmap
Eventually I would like to add a fuzzy finder to the tui for selecting the book mark. 
Currently, though I am more focused on polishing up the current features such as:
- Better error handling
- confirming before overwriting a book mark
  
