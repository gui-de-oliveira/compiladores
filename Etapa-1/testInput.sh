#!/usr/bin/expect -f
set timeout 1

set givenInput [lindex $argv 0]
set expectedOutput [lindex $argv 1]

set FAIL 1
set SUCCESS 0

spawn -noecho "./etapa1" 
send_user "Input: "
send -- "$givenInput\n"

expect {
    -ex "$expectedOutput" { exit $SUCCESS }
    default { exit $FAIL }
}

exit $FAIL