#!/bin/bash

#add the the command line allias to run the project for zsh terminals
echo 'alias do="$PWD/target/debug/DO"' >> ~/.zshrc
source ~/.zshrc
