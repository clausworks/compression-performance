#!/bin/bash
testProgram=~kmammen/357/Project7/lzwCompress
for testInput in *.in; do
   # remove extension
   name=${testInput%.in}

   # generate .expect files
   $testProgram $(<$name.in) 1> /dev/null 2> $name.err.expect
   exitCode=$?
   xxd -b files/$name.lzw > $name.out.expect
   echo "Exited with code $exitCode" >> $name.out.expect
   rm files/$name.lzw
done
