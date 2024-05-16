#!/bin/bash
# $1 - source path
# $2 - target path

# Z4.0.1
mkdir -p $2

# Z4.0.2
find $1 -type f -name "*.txt"

# Z4.0.3
find $1 -type l

# Z4.0.4
find $1 -type f -perm /a+x -name "*.sh"

# Z4.0.5
find $1 -type d -name "[AaDd]*"

# Z4.0.6
find $1 -type f -empty -user user

# Z4.0.7
find $1 -type f -group user -size +1M

# Z4.0.8
find $1 -type f -mmin -120

# Z4.0.9
find $1 -type d -perm /o+t

# Z4.0.10
# Must rewrite all the queries with or/and
find $1 -type f -perm /u+s -o -type f -perm /g+s

# Z4.0.11
find /dev/ -type b -o -type c

# Z4.0.12
find $1 -type d -empty -exec rmdir {} +

# Z4.0.13 to test
find $1 -type f -size +100M -exec mv {} $2 \;

# Z4.0.14
find $1 -type d -name "Z*" -exec cp -r {} $2 \;
