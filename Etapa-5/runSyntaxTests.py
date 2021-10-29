from os.path import exists
import subprocess
from colorama import Fore, Style

errorDic = {
    "ERR_UNDECLARED": 10,
    "ERR_DECLARED": 11,
    "ERR_VARIABLE": 20,
    "ERR_VECTOR": 21,
    "ERR_FUNCTION": 22,
    "ERR_WRONG_TYPE": 30,
    "ERR_STRING_TO_X": 31,
    "ERR_CHAR_TO_X": 32,
    "ERR_STRING_MAX": 33,
    "ERR_STRING_VECTOR": 34,
    "ERR_MISSING_ARGS": 40,
    "ERR_EXCESS_ARGS": 41,
    "ERR_WRONG_TYPE_ARGS": 42,
    "ERR_FUNCTION_STRING": 43,
    "ERR_WRONG_PAR_INPUT": 50,
    "ERR_WRONG_PAR_OUTPUT": 51,
    "ERR_WRONG_PAR_RETURN": 52,
    "ERR_WRONG_PAR_SHIFT": 53,
}

failed = []


def execute(fileName):
    file = open(fileName, "r")
    first_line = file.readline().strip()
    resultName = None
    expectedCode = None
    inputText = None
    shouldReturnError = first_line.startswith("//")
    if shouldReturnError:
        resultName = first_line[2:]
        expectedCode = errorDic[resultName]
        inputText = file.read()
    else:
        expectedCode = 0
        resultName = "Success"
        inputText = first_line + file.read()
    completedProcess = subprocess.run(
        ['./etapa5'], input=inputText, text=True, capture_output=True)
    exitCode = completedProcess.returncode

    if expectedCode != exitCode:
        failed.append(fileName)
        print(fileName + Fore.RED + " FAILED!" + Style.RESET_ALL)
        print("Expected: " + Fore.GREEN + str(expectedCode) +
              " (" + resultName + ")" + Style.RESET_ALL)
        print("Received: " + Fore.RED + str(exitCode) + Style.RESET_ALL)
        print("Input:\n" + Fore.CYAN + inputText + Style.RESET_ALL)
        print("Output:\n" + Fore.MAGENTA +
              completedProcess.stdout + '\n' + Style.RESET_ALL)
        return 0

    return 1


successes = 0

for i in range(0, 15):
    fileName = "TestsE4/abc"
    if i < 10:
        fileName += "0"
    fileName += str(i)
    if exists(fileName):
        successes += execute(fileName)

for i in range(0, 100):
    fileName = "TestsE4/kal"
    if i < 10:
        fileName += "0"
    fileName += str(i)
    if exists(fileName):
        successes += execute(fileName)

for i in range(1, 9):
    fileName = "TestsE4/mao0"
    fileName += str(i)
    if exists(fileName):
        successes += execute(fileName)

print("SUCCESSES: " + str(successes))
print("TESTS THAT FAILED:", failed)
