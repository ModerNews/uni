#!/bin/bash
# Z8.0
file_split () {
    if [ ! -r $1 ]; then
        echo "Brak uprawnień do odczytu pliku $1" 1>&2
        exit -1 
    fi

    if [ ! -e $2 ]; then
        touch $2
    fi
    if [ ! -e $3 ]; then
        touch $3
    fi

    if [ ! -w $2 -o ! -w $3 ]; then
        echo "Brak uprawnień do zapisu w plikach $2 lub $3" 1>&2
        exit -1
    fi
    
    echo "" > $2
    echo "" > $3

    while read line; do
        if (( $RANDOM % 2 == 0 )); then
            echo $line >> $2
        else
            echo $line >> $3
        fi
    done < $1
}

# Z8.1
dir_init ()
{   if [ ! -d $1 ]; then
        echo "Podana ścieżka $1 nie itnieje lub nie jest katalogiem" 1>&2
        return
    fi
    if [ ! -w $1 ]; then
        echo "Brak uprawnień do zapisu w katalogu $1" 1>&2
        return
    fi

    if [ ! -r $2 ]; then
        echo "Brak uprawnień do odczytu pliku $2" 1>&2
        return
    fi
    while read line; do
        mkdir $1/$line
    done < $2
}

#########
# Tests #
#########
# file_split "loremipsum.txt" "test1" "test2"
# dir_init "test" "test"
# dir_init "test_dir" "test"
# echo test
