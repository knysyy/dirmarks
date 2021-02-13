# DirMarks

## Installation
Confirm execution only with zsh on macos.
```
$ make install  
```  
Add the following to .zshrc.
```  
DATABASE_URL="${HOME}/bd.db"  
eval "$(dirmarks init)"  
chpwd() {dirmarks history save}
```
And run the following
```
$ source ~/.zshrc
$ dirmarks migrate
```

### How To Use
```
# help
$ dirmarks help

# Alias
ba -> dirmarks add
bl -> dirmarks list
br -> dirmarks rename
bd -> dirmarks delete

# Add Bookmark
$ cd
$ ba home -d "home directory"
Success : Added home -> your home directory

# Jump To Bookmark
$ bj home
$ pwd
your home directory

# Show List of Bookmark
$ bl
Displaying 1 directories
 id | key  | path                | description
 1  | home | your home directory | home directory

# Rename of Bookmark
$ br -n home2 -o home
Success : Renamed home -> home2

# Delete Bookmark
$ bd home2
Success : Deleted home2 -> your home directory

# Jump From History
Need to instal fzf
$ bh
```

## Develop
### Format
Add rustfmt of nightly version because using the nightly only option.
```  
$ rustup component add rustfmt --toolchain nightly  
$ make fmt  
```
