#!/bin/bash
target=../a.out
for testInput in *.in; do
   # remove file extension
   name=${testInput%.in}

   # run tests (with arguments as input)
   echo "$name"
   $target $(<$name.in) 1> /dev/null 2> $name.err
   exitCode=$?
   if [ $exitCode -eq 0 ]; then
      xxd -b files/$name.lzw > $name.out
   fi
   echo "Exited with code $exitCode" >> $name.out
   if [ $exitCode -eq 0 ]; then
   diff -q $name.out $name.out.expect
   fi
   diff -q $name.err $name.err.expect
   if [ $exitCode -eq 0 ]; then
      rm files/$name.lzw
   fi
done
echo "Done."
