#!/bin/bash

passedTests=0
failedTests=0

buildCompiler () {
    flex scanner.l
    gcc -c lex.yy.c
    gcc -c main.c
    gcc lex.yy.o main.o -o etapa1 -lfl
}

testInput () {
    givenInput=$1
    expectedOutput=$2

    ./testInput.sh "$givenInput" "$expectedOutput"

    if [ $? -eq 0 ]
    then
        echo "TEST FAILED! Expected output: $expectedOutput"
        ((failedTests++))
    else
        echo "SUCCESS!"
        ((passedTests++))
    fi
    
    echo ""
}

buildCompiler

testInput "5.5" "1 TK_LIT_FLOAT [5.5]"
testInput "51" "1 TK_LIT_FLOAT [5.5]"

echo "RESULTS:"
echo "Passed tests: $passedTests"
echo "Failed tests: $failedTests"