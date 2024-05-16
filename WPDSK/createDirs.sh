#!/bin/bash
# Z1.0.1
mkdir $1
# Z1.0.2
cd $1
# Z1.0.3
mkdir $1/d1 $1/d2 $1/d3
touch $1/Aa $1/Bb $1/Cc

mkdir $1/d1/d1.1
touch $1/d1/t1.txt

mkdir $1/d2/d2.1
touch $1/d2/d2.1/f1.csv

mkdir $1/d3/d3.1 $1/d3/d3.2
touch $1/d3/d3.2/f2.csv

# Z1.0.4
mv $1/d1/t1.txt $1/d1/d1.1

# Z1.0.5
cp $1/d2/d2.1/f1.csv $1/d1

# Z1.0.6
mv $1/Aa $1/Aa1

# Z1.0.7
ls -d {*1,*2}/

# Z1.0.8
du -h -d 1 $1
