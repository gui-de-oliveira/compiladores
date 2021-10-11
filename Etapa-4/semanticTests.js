const fs = require("fs").promises;
const util = require("util");
const exec = util.promisify(require("child_process").exec);

const ERROR_CODE = {
  ERR_UNDECLARED: 10,
  ERR_DECLARED: 11,
  ERR_VARIABLE: 20,
  ERR_VECTOR: 21,
  ERR_FUNCTION: 22,
  ERR_WRONG_TYPE: 30,
  ERR_STRING_TO_X: 31,
  ERR_CHAR_TO_X: 32,
  ERR_STRING_MAX: 33,
  ERR_STRING_VECTOR: 34,
  ERR_MISSING_ARGS: 40,
  ERR_EXCESS_ARGS: 41,
  ERR_WRONG_TYPE_ARGS: 42,
  ERR_FUNCTION_STRING: 43,
  ERR_WRONG_PAR_INPUT: 50,
  ERR_WRONG_PAR_OUTPUT: 51,
  ERR_WRONG_PAR_RETURN: 52,
  ERR_WRONG_PAR_SHIFT: 53,
};

const Color = {
  Reset: "\x1b[0m",
  Red: "\x1b[31m",
  Green: "\x1b[32m",
};

function logError(messageError) {
  console.trace(Color.Red + messageError + Color.Reset);
  process.exit();
}

let testsCounter = 0;
function acceptTest() {
  console.log(Color.Green + `Test ${++testsCounter} passed!` + Color.Reset);
}

async function testInvalidInput(input, expectedReturnCode, expectedOutput) {
  await fs.writeFile(`.temp`, input);

  const isInputInvalid = await exec(`./etapa4 < .temp`).catch((error) => {
    const { code: receivedReturnCode, stdout: receivedOutput } = error;

    if (expectedReturnCode !== receivedReturnCode) {
      logError(
        `Wrong ReturnCode! expected:"${expectedReturnCode}" received:"${receivedReturnCode}"`
      );
    }

    if (!receivedOutput.startsWith(expectedOutput)) {
      logError(
        `Wrong output! expected:"${expectedOutput}" received:"${receivedOutput}"`
      );
    }

    return true;
  });

  if (!isInputInvalid) {
    logError("input should be invalid!");
  }

  acceptTest();
}

async function testValidInput(input) {
  await fs.writeFile(`.temp`, input);

  await exec(`./etapa4 < .temp`).catch((error) => {
    console.log("Error:", error);
    logError("INPUT SHOULD BE VALID!");
  });

  acceptTest();
}

async function main() {
  // Example
  //   testInvalidInput(
  //     `int f1() { undeclared = 1; }`,
  //     ERROR_CODE.ERR_UNDECLARED,
  //     `Erro semântico na linha 1, coluna 13. Variável "undeclared" não foi declarada.`
  //   );
  testValidInput("int f1() { }");
}

main();
