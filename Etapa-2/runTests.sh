#!/bin/bash

testCounter=0
successfulTestsCounter=0
failedTestsCounter=0

SUCCESS=0
FAIL=1

buildCompiler () {
    make
    echo ""
}

runTestScript () {
    script=$1

    ((testCounter++))
    echo "Test $testCounter"

    expect -c "$script"
    result=$?

    if [ $result -eq 0 ]
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

    runTestScript "$script"
}

buildCompiler

testValidInput "int v1;"
testValidInput "static int v1;"

echo "RESULTS:"
echo "Passed tests: $successfulTestsCounter"
echo "Failed tests: $failedTestsCounter"