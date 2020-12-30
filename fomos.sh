#!/bin/bash

# This file is for installing the builtin commands for FOMOSv2

cd builtin_commands/
cd print/
make print
mv print ../../initramfs/bin/ && cd ../
