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
  console.log(
    Color.Red + `Test ${++testsCounter} failed${reason}!` + Color.Reset
  );
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

    rejectTest(" by succeeding");
  } catch (error) {
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

    if (failedTest) {
      rejectTest();
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
  await testValidInput("int f1() { return 0; }");

  // Test 2: Generic parsing error.
  await testInvalidInput(
    `
      int a a;
      bool a a;
    `,
    ERROR_CODE.ERR_LEX_PAR,
    `Parsing errors: Parsing error at line 2 column 13. No repair sequences found.\n`
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
      bool abc() { return true; }
    `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "abc"
First occurrence at line 2, column 11:
      int abc[3];
          ^^^
And again at line 3, column 12:
      bool abc() { return true; }
           ^^^
`
  );

  // Test 6: Two local variables, both with same name, in same scope.
  await testInvalidInput(
    `
      bool abc() {
        float aa <= 1;
        string aa <= "aa";
        return true;
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
        return true;
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
        return true;
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

  // Test 9: Expected variable, found function.
  await testInvalidInput(
    `
      int aaa;
      bool bbb() {
        float ccc <= aaa;
        ccc = aaa();
        return true;
      }
    `,
    ERROR_CODE.ERR_VARIABLE,
    `Variable identifier used as function: "aaa"
First occurrence at line 2, column 11:
      int aaa;
          ^^^
And again at line 5, column 15:
        ccc = aaa();
              ^^^
`
  );

  // Test 10: Expected function, found vector.
  await testInvalidInput(
    `
      bool bbb() {
        bbb[3] = aaa;
        return true;
      }
    `,
    ERROR_CODE.ERR_FUNCTION,
    `Function identifier used as vector: "bbb"
First occurrence at line 2, column 12:
      bool bbb() {
           ^^^
And again at line 3, column 9:
        bbb[3] = aaa;
        ^^^
`
  );

  // Test 11: Duplicated function definition.
  await testInvalidInput(
    `
        int f1(int a) { return 0; }
        int f1(int a) { return 0; }
      `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "f1"
First occurrence at line 2, column 13:
        int f1(int a) { return 0; }
            ^^
And again at line 3, column 13:
        int f1(int a) { return 0; }
            ^^`
  );

  // Test 12: Duplicated vector definition.
  await testInvalidInput(
    `
      int vec[10];
      int vec[10];
      `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "vec"
First occurrence at line 2, column 11:
      int vec[10];
          ^^^
And again at line 3, column 11:
      int vec[10];
          ^^^`
  );

  // O comando input deve ser seguido obrigatoriamente por um identificador do tipo int e float.
  // Caso contrário, o compilador deve lançar o erro ERR_WRONG_PAR_INPUT.

  // Test 13: expected int or float, received char
  await testInvalidInput(
    `
      int main() {
        char a;
        input a;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_INPUT,
    `Invalid argument for "input" command; expected variable of type "int" or "float", found "char";
First occurrence at line 3, column 14:
        char a;
             ^
And again at line 4, column 15:
        input a;
              ^`
  );

  // Test 14: expected int or float, received string
  await testInvalidInput(
    `
      int main() {
        string a;
        input a;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_INPUT,
    `Invalid argument for "input" command; expected variable of type "int" or "float", found "string";
First occurrence at line 3, column 16:
        string a;
               ^
And again at line 4, column 15:
        input a;
              ^`
  );

  // Test 15: expected int or float, received bool
  await testInvalidInput(
    `
      int main() {
        bool a;
        input a;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_INPUT,
    `Invalid argument for "input" command; expected variable of type "int" or "float", found "bool";
First occurrence at line 3, column 14:
        bool a;
             ^
And again at line 4, column 15:
        input a;
              ^`
  );

  // Test 16: valid int variable
  await testValidInput(
    `
      int main() {
        int a;
        input a;
        return 0;
      }
    `
  );

  // Test 17: valid float variable
  await testValidInput(
    `
      int main() {
        float a;
        input a;
        return 0;
      }
    `
  );

  // De maneira analoga, o comando output deve ser seguido por um identificador ou literal do tipo int e float.
  // Caso contrário, deve ser lançado o erro ERR_WRONG_PAR_OUTPUT.

  // Test 18: expected int or float, received bool
  await testInvalidInput(
    `
      int main() {
        bool a;
        output a;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_OUTPUT,
    `Invalid argument for "output" command; expected variable or literal of type "int" or "float", found "bool";
First occurrence at line 3, column 14:
        bool a;
             ^
And again at line 4, column 16:
        output a;
               ^`
  );

  // Test 19: expected int or float, received char
  await testInvalidInput(
    `
      int main() {
        char a;
        output a;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_OUTPUT,
    `Invalid argument for "output" command; expected variable or literal of type "int" or "float", found "char";
First occurrence at line 3, column 14:
        char a;
             ^
And again at line 4, column 16:
        output a;
               ^`
  );

  // Test 20: expected int or float, received string
  await testInvalidInput(
    `
      int main() {
        string a;
        output a;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_OUTPUT,
    `Invalid argument for "output" command; expected variable or literal of type "int" or "float", found "string";
First occurrence at line 3, column 16:
        string a;
               ^
And again at line 4, column 16:
        output a;
               ^`
  );

  // Test 21: valid int variable
  await testValidInput(
    `
      int main() {
        int a;
        output a;
        return 0;
      }
    `
  );

  // Test 22: valid float variable
  await testValidInput(
    `
      int main() {
        float a;
        output a;
        return 0;
      }
    `
  );
}

main();
