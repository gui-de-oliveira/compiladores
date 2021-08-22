#!/usr/bin/expect -f
set timeout 1

set givenInput [lindex $argv 0]
set expectedOutput [lindex $argv 1]

set FAIL 0
set SUCCESS 1

spawn -noecho "./etapa1" 
send_user "Input: "
send -- "$givenInput\n"

expect {
    -ex "$expectedOutput" { exit $SUCCESS }
}

exit $FAIL