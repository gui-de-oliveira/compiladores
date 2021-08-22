#!/bin/bash

passedTests=0
failedTests=0

SUCCESS=0
FAIL=1

buildCompiler () {
    flex scanner.l
    gcc -c lex.yy.c
    gcc -c main.c
    gcc lex.yy.o main.o -o etapa1 -lfl
}

testInput () {
    givenInput=$1
    expectedOutput=$2

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

buildCompiler

# Inputs 00 - 25
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
for keyword in "12" "-12" "+12"
do
    testInput "$keyword" "1 TK_LIT_INT [${keyword}]"
done

# Inputs 64 - 66
for keyword in "12.34" "-12.34" "+12.34"
do
    testInput "$keyword" "1 TK_LIT_FLOAT [${keyword}]"
done

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

# Input 74 - todo
# 12
#  //34  56
# 78

# Input 75 - todo
# 12 /*
#    34  56
# */78

# Input 76 - todo
# id12
# 34
# 56.78

# Input 77 - todo
# %>%

# Input 78 - todo
# %|%

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