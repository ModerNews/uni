#!/bin/bash
# Z7.0
bubble_sort ()
{
    local to_sort=( "$@" )
    local len=${#to_sort[@]}
    local i j
    local swapped
    for ((i = 0; i < len - 1; i++)); do
        swapped=false
        for ((j = 0; j < len - i - 1; j++)); do
            if ((${to_sort[j]} > ${to_sort[j + 1]})); then
                local tmp=${to_sort[j]}
                to_sort[j]=${to_sort[j + 1]}
                to_sort[j + 1]=$tmp
            fi
        done
        if [[ ! $swapped ]]; then
            break
        fi
    done
    echo ${to_sort[@]} 
}

# Z7.1
initiate_2d_arr ()
{
    local rows=$1
    local cols=$2
    if [[ -z $rows || -z $cols ]]; then
        echo "Usage: initiate_2d_arr <rows> <cols>"
        return 1
    fi
    if [[ $rows -lt 1 || $cols -lt 1 ]]; then
        echo "Rows and cols must be greater than 0"
        return 1
    fi
    
    declare -A arr
    for ((i = 0; i < rows; i++)); do
        for ((j = 0; j < cols; j++)); do
            arr[$i,$j]=$((i * j))
        done
    done
    for ((i = 0; i < rows; i++)); do
        for ((j = 0; j < cols; j++)); do
            echo -n "${arr[$i,$j]} "
        done
        echo
    done
}

# Z7.2
trapezoidal_integration () {
    local args=( "$@" )  # measured values
    local n=${#args[@]}
    local x=( "$(seq 0 $n)" )  # points of measurement
    local measured_sum=0
    for ((i = 0; i < n - 1; i++)); do
        val_1=${args[i]}
        val_2=${args[i + 1]}
        measured_sum=$(echo "scale=10; $measured_sum + $(echo "scale=10; ($val_1 + $val_2) / 2" | bc)" | bc)
        # echo $val_1 $val_2 $measured_sum
    done
    echo $measured_sum
}

#########
# Tests #
#########
# bubble_sort 10 3 1 2
# initiate_2d_arr 10 3
# initiate_2d_arr 3
# initiate_2d_arr 3 -3
# trapezoidal_integration 3 4 5 
