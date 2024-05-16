#Z6.0
calculate() {
    if [[ $(($1 - $2)) == 0 ]]; then
        echo "ValueError: Mianownik nie może równać się zeru"
        exit -1
    fi
    result=$(echo "scale=10; ($1 + $2)/($1 - $2)" | bc)
    echo $result
}

#Z6.1
factorial_rec() {
    result=$1
    if [[ $# > 1 ]]; then
        echo "ArgumentError: Funkcja przyjmuje tylko jeden argument"
        exit -1
    fi
    if [[ $result == 0 ]]; then 
        echo 1
    else
        tmp=$(factorial_rec $(($result - 1)))
        echo $(($result * $tmp))
    fi
}

#Z6.2
factorial_iter() {
    result=$1
    if [[ $# > 1 ]]; then
        echo "ArgumentError: Funkcja przyjmuje tylko jeden argument"
        exit -1
    fi
    if [[ $result == 0 ]]; then 
        echo 1
    else
        tmp=1
        for ((i=1; i<=$result; i++));
        do
            tmp=$(($tmp * $i))
        done
        echo $tmp
    fi
}

#Z6.3
sum() {
    result=0
    for i in $@; do
        if ! [[ $i =~ ^[+-]*[0-9]+$ ]]; then
            echo "ValueError: Argumenty muszą być liczbami całkowitymi"
            exit -1
        fi
        result=$(($result + $i))
    done
    echo $result
}

# ===============
# Test Executions
# ===============
# calculate 3 5
# #calculate 3 3
# factorial_rec 15
# #factorial_rec 15 1
# factorial_iter 15
# #factorial_iter 15 1
# sum 1 2 3 4 5 6 7 8 9 +10 -11
# sum 1 2 3 4 5 6 7 8 9 10
# #sum 1 2 3 4 5 6 7 8 9 10 test
