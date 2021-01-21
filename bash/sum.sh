#!/bin/bash

sum=0

while
    read -p 'input num> ' n && echo $n && [[ $n =~ [0-9]+ ]]
do
    ((sum += n))
done

echo "total sum is $sum"
