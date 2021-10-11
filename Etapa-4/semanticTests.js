const fs = require("fs").promises;
const util = require("util");
const exec = util.promisify(require("child_process").exec);

const ERROR_CODE = {
  ERR_LEX_PAR: 1,
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
  console.log(Color.Red + messageError + Color.Reset);
}

let testsCounter = 0;
function acceptTest() {
  console.log(Color.Green + `Test ${++testsCounter} passed!` + Color.Reset);
}

function rejectTest(reason = "") {
  console.log(Color.Red + `Test ${++testsCounter} failed${reason}!` + Color.Reset);
  process.exit();
}

async function testInvalidInput(input, expectedReturnCode, expectedOutput) {
  await fs.writeFile(`.temp`, input);

  try {
    const value = await exec(`./etapa4 < .temp`);
    const receivedOutput = value.stdout;

    if (!receivedOutput.startsWith(expectedOutput)) {
      logError(`Wrong output!`);
      logError(`Expected:`);
      logError(expectedOutput);
      logError(`Received:`);
      logError(receivedOutput);
    }

    rejectTest(" by succeeding")

  } catch(error) {
    const { code: receivedReturnCode, stdout: receivedOutput } = error;

    let failedTest = false;

    if (expectedReturnCode !== receivedReturnCode) {
      logError(`Wrong ReturnCode!`);
      logError(`Expected:`);
      logError(expectedReturnCode);
      logError(`Received:`);
      logError(receivedReturnCode);
      failedTest = true;
    }

    if (!receivedOutput.startsWith(expectedOutput)) {
      logError(`Wrong output!`);
      logError(`Expected:`);
      logError(expectedOutput);
      logError(`Received:`);
      logError(receivedOutput);
      failedTest = true;
    }

    if(failedTest) {
      rejectTest()
    } else {
      acceptTest();
    }
  }
}

async function testValidInput(input) {
  await fs.writeFile(`.temp`, input);

  await exec(`./etapa4 < .temp`).catch((error) => {
    console.log("Error:", error);
    logError("INPUT SHOULD BE VALID!");
    rejectTest();
  });

  acceptTest();
}

async function main() {
  // Example
  //   await testInvalidInput(
  //     `int f1() { undeclared = 1; }`,
  //     ERROR_CODE.ERR_UNDECLARED,
  //     `Erro semântico na linha 1, coluna 13. Variável "undeclared" não foi declarada.`
  //   );

  // Test 1: Valid input.
  await testValidInput("int f1() { }");

  // Test 2: Generic parsing error.
  await testInvalidInput(
    `
      int a a;
      bool a a;
    `,
    ERROR_CODE.ERR_LEX_PAR,
    `parsing errors: Parsing error at line 2 column 13. No repair sequences found.\n`
  );

  // Test 3: Two same-name global vars, in same scope.
  await testInvalidInput(
    `
      int abc;
      bool abc;
    `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "abc"
First occurrence at line 2, column 11:
      int abc;
          ^^^
And again at line 3, column 12:
      bool abc;
           ^^^
`
  );

  // Test 4: One global vec and one global var, both with same name, in same scope.
  await testInvalidInput(
    `
      int abc[3];
      bool abc;
    `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "abc"
First occurrence at line 2, column 11:
      int abc[3];
          ^^^
And again at line 3, column 12:
      bool abc;
           ^^^
`
  );

  // Test 5: One global vec and one global function, both with same name, in same scope.
  await testInvalidInput(
    `
      int abc[3];
      bool abc() {}
    `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "abc"
First occurrence at line 2, column 11:
      int abc[3];
          ^^^
And again at line 3, column 12:
      bool abc() {}
           ^^^
`
  );

  // Test 6: Two local variables, both with same name, in same scope.
  await testInvalidInput(
    `
      bool abc() {
        float aa <= 1;
        string aa <= "aa";
      }
    `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "aa"
First occurrence at line 3, column 15:
        float aa <= 1;
              ^^
And again at line 4, column 16:
        string aa <= "aa";
               ^^
`
  );

  // Test 7: Uninitialized variable.
  await testInvalidInput(
    `
      int aaa;
      bool bbb() {
        float ccc <= ddd;
      }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "ddd"
Occurrence at line 4, column 22:
        float ccc <= ddd;
                     ^^^
`
  );

  // Test 8: Expected vector, found variable.
  await testInvalidInput(
    `
      int aaa[1];
      bool bbb() {
        float ccc <= aaa;
      }
    `,
    ERROR_CODE.ERR_VECTOR,
    `Vector identifier used as variable: "aaa"
First occurrence at line 2, column 11:
      int aaa[1];
          ^^^
And again at line 4, column 22:
        float ccc <= aaa;
                     ^^^
`
  );

}

main();
