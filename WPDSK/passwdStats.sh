# Z3.1.1
mkdir -p $1

# Z3.1.2
grep --color=none $(whoami) /etc/passwd

# Z3.1.3
cut -d: -f1,6,7 /etc/passwd | sort -r >> $1/F1.csv

# Z3.1.4 
cut -d: -f7 /etc/passwd | sort | uniq | rev | sort | rev >> $1/F2.csv

# Z3.1.5
cut -d: -f1 /etc/passwd | tr '[:lower:]' '[:upper:]' >> $1/F3.txt

# Z3.1.6
grep -C 4 $(whoami) /etc/passwd >> $1/F4.txt

# Z3.1.7
diff /etc/passwd $1/F4.txt >> $1/differences.txt
