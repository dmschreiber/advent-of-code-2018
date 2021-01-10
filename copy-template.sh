#!/bin/zsh
if [ $# -ne 1 ]; then
  echo "expected puzzle number argument"
  exit -1
fi
touch inputs/puzzle$1.txt
touch inputs/puzzle$1-test.txt
cat src/puzzle-template.rs | sed 's/puzzle1/puzzle'$1'/g' > src/puzzle$1.rs
cp src/main.rs src/main.rs.bak
sed -e 's/pub mod puzzle1;/pub mod puzzle1;\
pub mod puzzle'$1';/g' -e 's|(puzzle1::solve,".\/inputs\/puzzle1.txt".to_string()),|(puzzle1::solve,".\/inputs\/puzzle1.txt".to_string()),\
        (puzzle'$1'::solve,".\/inputs\/puzzle'$1'.txt".to_string()),|g' src/main.rs.bak  > src/main.rs
status=$?
[ $status -eq 0 ] && rm src/main.rs.bak
