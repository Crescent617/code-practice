val=10
int=10
if (( int >= val )); then
    echo greater equal
fi

if [ -s "./sum.sh" ] && (( int >= val )); then
    echo file not empty
fi
