#!/bin/bash
# Z1.1.1
mkdir $2

# Z1.1.2
cp -r $1/* $2

# Z1.1.3
cd $2

# Z1.1.4
ls -l

# Z1.1.5
tree -h

# Z1.1.6
pwd

# Z1.1.7
cd -

# Z1.1.8
rm -r $2
rmdir $2
