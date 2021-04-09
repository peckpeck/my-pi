#!/bin/sh

# extract chronogram
perl -ne 'BEGIN{$t=0; $ref=""}{/^(\d+) (.*)/; $a=$1; $b=$2; if($ref ne $b) { $x=($a-$t); printf("%03d %s\n",$x,$ref); $t=$a; $ref=$b;} }END{$x=($a-$t); print "$x $b\n"}'

# exctract bits
perl -ne '/(\d+) (\d) (\d) (\d)/; if($3==0 && $1<5){$out="?"}elsif($4==1){$out="--"}elsif($3==0){$out=$2}else{$out=""}; print "$out\n";' h1 | grep -v "^$" | perl -pe 's/(\d)\n/$1,/'
