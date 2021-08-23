#!/bin/bash

passedTests=0
failedTests=0

SUCCESS=0
FAIL=1

inputCounter=0

buildCompiler () {
    make
}

runTestScript () {
    script=$1
    expectedOutput=$2

    ((inputCounter++))
    echo "Test $inputCounter"

    expect -c "$script"
    result=$?

    if [ $result -eq $FAIL ]
    then
        echo "TEST FAILED!"
        echo "Expected output: $expectedOutput"
        ((failedTests++))
    else
        echo "SUCCESS!"
        ((passedTests++))
    fi
    
    echo ""
}

testInput () {
    givenInput=$1
    expectedOutput=$2

    escapedInput="$givenInput"
    escapedInput="${escapedInput//"["/"\\["}"
    escapedInput="${escapedInput//"]"/"\\]"}"
    escapedInput="${escapedInput//"\""/"\\\""}"

    escapedOutput="$expectedOutput"
    escapedOutput="${escapedOutput//"["/"\\["}"
    escapedOutput="${escapedOutput//"]"/"\\]"}"
    escapedOutput="${escapedOutput//"\""/"\\\""}"

    script='
        set timeout 1

        set givenInput "'${escapedInput}'"
        set expectedOutput "'${escapedOutput}'"

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
    '

    runTestScript "$script" "$expectedOutput"
}

testInput62 () {
    script='
        set timeout 1

        set FAIL 1
        set SUCCESS 0

        spawn -noecho "./etapa1" 
        send_user "Input: "
        send -- "-12\n"

        expect {
            -ex "1 TK_LIT_INT \[-12\]" { exit $SUCCESS }
            default { exit $FAIL }
        }

        exit $FAIL
    '

    runTestScript "$script" "1 TK_LIT_INT [-12]"
}

testInput65 () {
    script='
        set timeout 1

        set FAIL 1
        set SUCCESS 0

        spawn -noecho "./etapa1" 
        send_user "Input: "
        send -- "-12.34\n"

        expect {
            -ex "1 TK_LIT_FLOAT \[-12.34\]" { exit $SUCCESS }
            default { exit $FAIL }
        }

        exit $FAIL
    '

    runTestScript "$script" "1 TK_LIT_INT [-12.34]"
}

testInput74 () {
    script='
        set FAIL 1
        set SUCCESS 0

        spawn -noecho "./etapa1" 
        send_user "Input: "

        send -- "12\n"
        expect {
            -ex "1 TK_LIT_INT \[12\]" { }
            default { exit $FAIL }
        }

        send -- " //34  56\n"
        expect {
            -ex " " { }
            default { exit $FAIL }
        }

        send -- "78\n"
        expect {
            -ex "3 TK_LIT_INT \[78\]" { exit $SUCCESS }
            default { exit $FAIL }
        }

        exit $FAIL
    '

    runTestScript "$script" "[3 messages...]"
}

testInput75 () {
    script='
        set timeout 1

        set FAIL 1
        set SUCCESS 0

        spawn -noecho "./etapa1" 
        send_user "Input: "

        send -- \
        "12 /*
        34  56
        */78\n"

        expect {
            -ex "1 TK_LIT_INT \[12\]" { }
            default { exit $FAIL }
        }

        expect {
            -ex "3 TK_LIT_INT \[78\]" { exit $SUCCESS }
            default { exit $FAIL }
        }

        exit $FAIL
    '

    runTestScript "$script" "3 messages"
}

testInput76 () {
    script='
        set timeout 1

        set FAIL 1
        set SUCCESS 0

        spawn -noecho "./etapa1" 
        send_user "Input: "

        send -- \
        "id12
        34
        56.78\n"

        expect {
            -ex "1 TK_IDENTIFICADOR \[id12\]" { }
            default { exit $FAIL }
        }

        expect {
            -ex "2 TK_LIT_INT \[34\]" { }
            default { exit $FAIL }
        }

        expect {
            -ex "3 TK_LIT_FLOAT \[56.78\]" { exit $SUCCESS }
            default { exit $FAIL }
        }

        exit $FAIL
    '

    runTestScript "$script" "3 messages"
}

testInput77 () {
    script='
        set timeout 1

        set FAIL 1
        set SUCCESS 0

        spawn -noecho "./etapa1" 
        send_user "Input: "
        send -- "%>%\n"

        expect {
            -ex "1 TK_ESPECIAL \[%\]" { }
            default { exit $FAIL }
        }

        expect {
            -ex "1 TK_ESPECIAL \[>\]" { }
            default { exit $FAIL }
        }

        expect {
            -ex "1 TK_ESPECIAL \[%\]" { exit $SUCCESS }
            default { exit $FAIL }
        }

        exit $FAIL
    '

    runTestScript "$script" "3 messages"
}

testInput78 () {
    script='
        set timeout 1

        set FAIL 1
        set SUCCESS 0

        spawn -noecho "./etapa1" 
        send_user "Input: "
        send -- "%|%\n"

        expect {
            -ex "1 TK_ESPECIAL \[%\]" { }
            default { $FAIL }
        }

        expect {
            -ex "1 TK_ESPECIAL \[|\]" { }
            default { $FAIL }
        }

        expect {
            -ex "1 TK_ESPECIAL \[%\]" { exit $SUCCESS }
            default { $FAIL }
        }

        exit $FAIL
    '

    runTestScript "$script" "2 messages"
}

buildCompiler

# Inputs 01 - 25
for keyword in \
    "int" "float" "bool" "char" "string" "if" "then" "else" "while" "do" \
    "input" "output" "return" "const" "static" "foreach" "for" "switch" \
    "case" "break" "continue" "class" "private" "public" "protected"
do
    testInput "$keyword" "1 TK_PR_${keyword^^} [${keyword}]"
done

# Inputs 26 - 47
for keyword in "," ";" ":" "(" ")" "[" "]" "{" "}" "+" "-" "|" "*" \
    "/" "<" ">" "=" "!" "&" "%" "#" "^" "." "$"
do
    testInput "$keyword" "1 TK_ESPECIAL [${keyword}]"
done

# Inputs 48 - 55
testInput "<=" "1 TK_OC_LE [<=]"
testInput ">=" "1 TK_OC_GE [>=]"
testInput "==" "1 TK_OC_EQ [==]"
testInput "!=" "1 TK_OC_NE [!=]"
testInput "&&" "1 TK_OC_AND [&&]"
testInput "||" "1 TK_OC_OR [||]"
testInput ">>" "1 TK_OC_SL [>>]"
testInput "<<" "1 TK_OC_SR [<<]"

# Inputs 56 - 60
for keyword in "id" "ID" "_id" "_ID" "_01"
do
    testInput "$keyword" "1 TK_IDENTIFICADOR [${keyword}]"
done

# Inputs 61 - 63
testInput "12" "1 TK_LIT_INT [12]"
testInput62 # the minus symbol is being misinterpreted as a flag on the generic test function
testInput "+12" "1 TK_LIT_INT [+12]"

# Inputs 64 - 66
testInput "12.34" "1 TK_LIT_FLOAT [12.34]"
testInput65 # the minus symbol is being misinterpreted as a flag on the generic test function
testInput "+12.34" "1 TK_LIT_FLOAT [+12.34]"

# Inputs 67 - 68
testInput "true" "1 TK_LIT_TRUE [true]"
testInput "false" "1 TK_LIT_FALSE [false]"

# Inputs 69 - 71
for keyword in "'a'" "'='" "'+'"
do
    testInput "$keyword" "1 TK_LIT_CHAR [${keyword}]"
done

# Inputs 72 - 73
for keyword in "\"meu nome\"" "\"x = 3\""
do
    testInput "$keyword" "1 TK_LIT_STRING [${keyword}]"
done

# Input 74 - 76
# These tests sends and expects multiple lines
testInput74 
testInput75 
testInput76

# Input 77
# These tests expects more than one reply
testInput77
testInput78

# Inputs 79 - 80
for keyword in "|" "$"
do
    testInput "$keyword" "1 TK_ESPECIAL [${keyword}]"
done

# Inputs 81 - 84
for keyword in "?" "~" "@" "\`"
do
    testInput "$keyword" "1 TOKEN_ERRO [${keyword}]"
done

# Input 85 - 86
testInput "end" "1 TK_PR_END [end]"
testInput "default" "1 TK_PR_DEFAULT [default]"

echo "RESULTS:"
echo "Passed tests: $passedTests"
echo "Failed tests: $failedTests"