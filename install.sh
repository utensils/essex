#!/usr/bin/env bash
# This is a very simple install script for essex

if [ ! -d ~/.essex ]
then
    git clone https://github.com/utensils/essex.git ~/.essex
    echo "Be sure to add ~/.essex to your \$PATH"
else
    echo "Essex apprears to already be installed"
    exit 1
fi