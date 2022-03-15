#!/bin/bash

if [[ "$#" -eq 1 ]]
then
  ./expr < "$1" > /dev/null 2>&1
  if [[ "$?" -ne "0" ]]
  then
    echo "Test \"$1\" FAILED"
  else
    echo "Test \"$1\" PASSED"
  fi
else
  FILES=$(ls -A1 ../examples/*.expr)

  for test in ${FILES}
  do
    ./expr < $test > /dev/null 2>&1
    if [[ "$?" -ne "0" ]]
    then
      printf "Test \"${test}\" \033[0;31mFAILED\033[0;0m\n"
    else
      printf "Test \"${test}\" \033[0;32mPASSED\033[0;0m\n"
    fi
  done
fi