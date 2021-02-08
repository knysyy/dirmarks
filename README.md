
# Bookmark-Directory

## Installation
Confirm execution only with zsh.
```
$ make install  
```  
Add the following to .zshrc.
```  
DATABASE_URL="${HOME}/bd.db"  
eval "$(bookmark-directory init)"  
```
And run the following
```
$ source ~/.zshrc
$ bookmark-directory migrate
```

### How To Use
```
# help
$ bookmark-directory help

# Alias
ba -> bookmark-directory add
bj -> bookmark-directory jump
bl -> bookmark-directory list
br -> bookmark-directory rename
bd -> bookmark-directory delete

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
```

## Develop
### Format
Add rustfmt of nightly version because using the nightly only option.
```  
$ rustup component add rustfmt --toolchain nightly  
$ make fmt  
```
