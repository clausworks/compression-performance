echo "THIS SCRIPT DOESN'T WORK"
for testFile in $1/*; do
    echo "$testFile" > "$2_$(basename $testFile).in"
done
