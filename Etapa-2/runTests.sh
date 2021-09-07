#!/bin/bash

testCounter=0
successfulTestsCounter=0
failedTestsCounter=0

SUCCESS=0
FAIL=1

buildCompiler () {
    make
    result=$?

    if [ $result -ne 0 ]
    then
        echo "Build failed!"
        exit $result
    fi

    echo ""
}

runTestScript () {
    givenInput=$1
    expectedValue=$2

    ((testCounter++))
    echo "Test $testCounter"

    escapedInput="$givenInput"
    escapedInput="${escapedInput//"["/"\\["}"
    escapedInput="${escapedInput//"]"/"\\]"}"
    escapedInput="${escapedInput//"\""/"\\\""}"

    script='
        set timeout 1

        set givenInput "'${escapedInput}'"

        spawn -noecho "./etapa2" 
        send_user "Input: "
        send -- "$givenInput\n"

        # Await max time for response (fixes bug of false-positives)
        sleep 0.0001

        expect {
            "syntax error" {
                # return exit code from spawned process
                catch wait result
                exit [lindex $result 3]
            }
            -ex "\r" { }
        }

        close

        # return exit code from spawned process
        catch wait result
        exit [lindex $result 3]
    '

    expect -c "$script"
    result=$?

    if [ $result -eq $expectedValue ]
    then
        echo "SUCCESS!"
        ((successfulTestsCounter++))
    else
        echo "TEST FAILED!"
        ((failedTestsCounter++))
    fi
    
    echo ""
}

testValidInput () { 
    runTestScript "$1" 0
}

testInvalidInput () { 
    runTestScript "$1" 1
}

buildCompiler

# global variable declarations
for type in "int" "char" "float" "bool" "string"
do
    testValidInput "$type v1;"
    testInvalidInput "$type;"

    testValidInput "static $type v1;"
    testInvalidInput "static $type;"

    testValidInput "$type v1, v2;"
    testValidInput "$type v1, v2, v3;"
    testInvalidInput "$type v1,;"
    testInvalidInput "$type ,v1;"

    testValidInput "$type v1[3];"
    testValidInput "$type v1[+3];"
    #TODO: testInvalidInput "$type v1[0];"
    #TODO: testInvalidInput "$type v1[-1];"

    testValidInput "$type v1[1], v2[2], v3[3];"
    testValidInput "static $type v1[1], v2, v3[3];"
    testValidInput "$type v1, v2[+5], v3;"
done

# Function header declaration
testValidInput "int functionName(int a) { }"
testValidInput "int functionName(int a, bool b) { }"
testValidInput "int functionName(int a, bool b, string c) { }"
testValidInput "int functionName(const int a, bool b) { }"
testValidInput "int functionName(int a, const bool b) { }"
testInvalidInput "int functionName(int a[5]) { }"
testInvalidInput "int functionName(int a,) { }"
testInvalidInput "int functionName(,int a) { }"

for static in "static" " "
do
    for basicType in "int" "char" "float" "bool" "string"
    do
        testValidInput "$static $basicType functionName() { }"
        testValidInput "$static $basicType functionName($basicType a) { }"
    done
done

# Command block / commands
for const in "const" " "
do
    for static in "static" " "
    do
        for basicType in "int" "char" "float" "bool" "string"
        do
            testValidInput "int f() { $static $const $basicType id; }"
        done
    done
done

testValidInput "int f() { int id; }"
testValidInput "int f() { int id1; int id2; }"
testValidInput "int f() { int id1; int id2; int id3; }"
testValidInput "int f() { int id1, id2; }"
testValidInput "int f() { int id1, id2, id3; }"

testValidInput "int f() { int id1 <= id2; }"
testValidInput "int f() { int id1 <= id2, id3 <= id4; }"
testValidInput "int f() { int id1 <= id2, id3 <= id4, id5 <= id6; }"
testValidInput "int f() { int id1 <= 0; }"
testValidInput "int f() { float id1 <= 0.0; }"
testValidInput "int f() { char id1 <= 'c'; }"
testValidInput "int f() { string id1 <= \"c\"; }"
testValidInput "int f() { bool id1 <= false; }"
testValidInput "int f() { bool id1 <= true; }"
#TODO: testInvalidInput "int f() { int id1 <= false; }"

testValidInput "int f() { id = true; }"
testValidInput "int f() { id1 = 1; }"
testValidInput "int f() { id1 = 1.0; }"
testValidInput "int f() { id1 = 'c'; }"
testValidInput "int f() { id1 = \"string\"; }"
testValidInput "int f() { id1 = id2; }"
#TODO: "int f() { id1 = <expressão>; }"

testValidInput "int f() { id[1] = true; }"
testValidInput "int f() { id[1] = 1; }"
testValidInput "int f() { id1[1] = 1.0; }"
testValidInput "int f() { id1[1] = 'c'; }"
testValidInput "int f() { id1[1] = \"string\"; }"
testValidInput "int f() { id1[1] = id2; }"
#TODO: "int f() { id1[<expressão>] = true; }"


echo "RESULTS:"
echo "Passed tests: $successfulTestsCounter"
echo "Failed tests: $failedTestsCounter"

make clean