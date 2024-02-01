#!/bin/sh
print_title()
{
  printf "\e[34;47m%s\e[39;49m\n" "$1"
}
print_success()
{
  printf "\e[32;107msuccess\e[39;49m\n"
}
print_failure()
{
  printf "\e[31;107m%s\e[39;49m\n" "$1"
}

print_title "building release" &&
cargo build --release &&

print_success &&
exit 0

print_failure "something went wrong"
exit 1
