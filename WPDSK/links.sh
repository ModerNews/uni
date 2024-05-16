#!/bin/bash

# Z2.0.1
mkdir $1

# Z2.0.2
mkdir $1/D1
mkdir $1/D2
mkdir $1/D3
mkdir $1/D4

touch $1/D2/F1.txt
touch $1/D3/F1.txt
touch $1/D4/F1.txt

# Z2.0.3
ln -s /etc/passwd $1/D1/

# Z2.0.4
readlink $1/D1/passwd

# Z2.0.5
ln $1/D3/F1.txt $1/D2/F2.txt

# Z2.0.6
chmod 600 $1/D2/F2.txt

# Z2.0.7
sudo chown :users $1/D2/F2.txt

# Z2.0.8
sudo chgrp users $1/D3

# Z2.0.9
chmod -x $1/D3

# Z2.0.10
chmod -w $1/D2

# Z2.0.11
chmod -r $1/D4

# Z2.0.12
chmod +t $1/D4

# Z2.0.13
touch $1/D1/scr1.sh
chmod ug+x $1/D1/scr1.sh

# Z2.0.14
chmod u+s $1/D1/scr1.sh
chmod g+s $1/D1/scr1.sh
