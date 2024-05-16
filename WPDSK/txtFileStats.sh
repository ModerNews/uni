#!/bin/bash
# Z3.0.1
file $1

# Z3.0.2
wc -l $1

# Z3.0.3
wc -m $1

# Z3.0.4
du -h $1

# Z3.0.5
grep -c $2 $1

# Z3.0.6
grep -vc $2 $1

# Z3.0.7
cat $1 | head -n 5 | tail -n 1
