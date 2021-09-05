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
        sleep 0.1

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

# Int declarations
testValidInput "int v1;"
testValidInput "int v1   ;"
testInvalidInput "int ;"
testInvalidInput "int;"

testValidInput "static int v1;"
testValidInput "static int v1  ;"

testValidInput "int v1, v2, v3;"
testValidInput "int v1, v2, v3 ;"
testValidInput "int v1,v2,v3;"
testInvalidInput "int ,v1 ;"
testInvalidInput "int v1, ;"

testValidInput "int v1[3];"
testValidInput "int v1[+3];"
testValidInput "int v1[3] ;"
testValidInput "int v1 [3] ;"
testValidInput "int v1 [ 3 ] ;"
testInvalidInput "int v1[0];"
testInvalidInput "int v1[-1];"

testValidInput "int v1[1], v2[2], v3[3];"
testValidInput "static int v1[1], v2, v3[3];"
testValidInput "int v1, v2[+5], v3;"

echo "RESULTS:"
echo "Passed tests: $successfulTestsCounter"
echo "Failed tests: $failedTestsCounter"