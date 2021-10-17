const fs = require("fs").promises;
const util = require("util");
const exec = util.promisify(require("child_process").exec);

/*
B Códigos de retorno
Os seguintes códigos de retorno devem ser utilizados quando o compilador encontrar erros semânticos.
O programa deve chamar exit utilizando esses códigos imediamente após a impressão da linha que descreve o erro.
Na ausência de qualquer erro, o programa deve retornar o valor zero.

#define ERR_UNDECLARED 10
#define ERR_DECLARED 11
#define ERR_VARIABLE 20
#define ERR_VECTOR 21
#define ERR_FUNCTION 22
#define ERR_WRONG_TYPE 30
#define ERR_STRING_TO_X 31
#define ERR_CHAR_TO_X 32
#define ERR_STRING_MAX 33
#define ERR_STRING_VECTOR 34
#define ERR_MISSING_ARGS 40
#define ERR_EXCESS_ARGS 41
#define ERR_WRONG_TYPE_ARGS 42
#define ERR_FUNCTION_STRING 43
#define ERR_WRONG_PAR_INPUT 50
#define ERR_WRONG_PAR_OUTPUT 51
#define ERR_WRONG_PAR_RETURN 52
#define ERR_WRONG_PAR_SHIFT 53

Estes valores são utilizados na avaliação objetiva.
*/

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
  console.log(Color.Green + `${++testsCounter} tests passed!` + Color.Reset);
}

function rejectTest(testName, reason) {
  console.log(
    Color.Red + `Test "${testName}" failed!\nReason: ${reason}!` + Color.Reset
  );
  process.exit();
}

async function testInvalidInput(
  testName,
  input,
  expectedReturnCode,
  expectedOutput
) {
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

    rejectTest(testName, " by succeeding");
  } catch (error) {
    const { code: receivedReturnCode, stdout: receivedOutput } = error;

    let failedTest = false;

    if (expectedReturnCode !== receivedReturnCode) {
      logError(`Expected:`);
      logError(expectedReturnCode);
      logError(`Received:`);
      logError(receivedReturnCode);
      logError(`Received Output:`);
      logError(receivedOutput);
      rejectTest(testName, "WRONG RETURN CODE!");
    }

    if (!receivedOutput.startsWith(expectedOutput)) {
      logError(`Expected:`);
      logError(expectedOutput);
      logError(`Received:`);
      logError(receivedOutput);
      rejectTest(testName, "WRONG OUTPUT!");
    }

    acceptTest();
  }
}

async function testValidInput(testName, input) {
  await fs.writeFile(`.temp`, input);

  await exec(`./etapa4 < .temp`).catch((error) => {
    console.log("Error:", error);
    rejectTest(testName, "INPUT SHOULD BE VALID!");
  });

  acceptTest();
}

async function main() {
  await testValidInput("Basic valid input", "int f1() { return 0; }");

  await testInvalidInput(
    "Generic parsing error",
    `
      int a a;
      bool a a;
    `,
    ERROR_CODE.ERR_LEX_PAR,
    `Parsing errors: Parsing error at line 2 column 13. No repair sequences found.\n`
  );

  await testInvalidInput(
    "Two same-name global vars, in same scope",
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

  await testInvalidInput(
    "One global vec and one global var, both with same name, in same scope",
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

  await testInvalidInput(
    "One global vec and one global function, both with same name, in same scope",
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

  await testInvalidInput(
    "Two local variables, both with same name, in same scope",
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

  // Todos os identificadores devem ter sido declarados no momento do seu uso, seja como variável, como vetor ou como função.
  // Caso o identificador não tenha sido declarado no seu uso, deve-se lançar o erro ERR_UNDECLARED.

  await testInvalidInput(
    "Uninitialized variable",
    `
      int main() {
        float xxx <= aaa;
        return 0;
      }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "aaa"
Occurrence at line 3, column 22:
        float xxx <= aaa;
                     ^^^`
  );

  await testInvalidInput(
    "Uninitialized vector",
    `
      int main() {
        float xxx;
        xxx = aaa[5];
        return 0;
      }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "aaa"
Occurrence at line 4, column 15:
        xxx = aaa[5];
              ^^^`
  );

  await testInvalidInput(
    "Uninitialized function",
    `
      int main() {
        aaa();
        return 0;
      }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "aaa"
Occurrence at line 3, column 9:
        aaa();
        ^^^`
  );

  await testInvalidInput(
    "Left shift on an undeclared variable.",
    `
    int main() {
      a << 15;
      return 0;
    }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "a"
Occurrence at line 3, column 7:
      a << 15;
      ^`
  );

  await testInvalidInput(
    "Right shift on an undeclared variable.",
    `
    int main() {
      a >> 15;
      return 0;
    }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "a"
Occurrence at line 3, column 7:
      a >> 15;
      ^`
  );

  await testInvalidInput(
    "Left shift on an undeclared vector.",
    `
    int main() {
      a[0] << 15;
      return 0;
    }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "a"
Occurrence at line 3, column 7:
      a[0] << 15;
      ^`
  );

  await testInvalidInput(
    "Right shift on an undeclared vector.",
    `
    int main() {
      a[0] >> 15;
      return 0;
    }
    `,
    ERROR_CODE.ERR_UNDECLARED,
    `Usage of undeclared identifier: "a"
Occurrence at line 3, column 7:
      a[0] >> 15;
      ^`
  );

  // O uso de identificadores deve ser compatível com sua declaração e com seu tipo.
  // Variáveis somente podem ser usadas sem indexação, vetores somente podem ser utilizados com indexação, e funções apenas devem ser usadas como chamada de função, isto é, seguidas da lista de argumentos, esta possivelmente vazia conforme a sintaxe da E2, entre parênteses.

  // Caso o identificador dito variável seja usado como vetor ou como função, deve-se lançar o erro ERR_VARIABLE.

  await testInvalidInput(
    "Expected variable, found function",
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
              ^^^`
  );

  // Caso o identificador dito vetor seja usado como variável ou função, deve-se lançar o erro ERR_VECTOR.

  await testInvalidInput(
    "Expected vector, found variable",
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

  // Enfim, caso o identificador dito função seja utilizado como variável ou vetor, deve-se lançar o erro ERR_FUNCTION.

  await testInvalidInput(
    "Expected function, found vector",
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

  // Todas as entradas na tabela de símbolos devem ter um tipo associado conforme a declaração, verificando-se se não houve dupla declaração ou se o símbolo nao foi declarado.
  //  Caso o identificador ja tenha sido declarado, deve-se lançar o erro ERR_DECLARED.

  await testInvalidInput(
    "Duplicated variable declaration",
    `
        int main() {
          int aaa;
          float aaa;
          return 0;
        }
      `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "aaa"
First occurrence at line 3, column 15:
          int aaa;
              ^^^
And again at line 4, column 17:
          float aaa;
                ^^^`
  );

  await testInvalidInput(
    "Duplicated function definition",
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

  await testInvalidInput(
    "Duplicated vector definition.",
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

  // Variáveis com o mesmo nome podem co-existir em escopos diferentes, efetivamente mascarando as variaveis que estão em escopos superiores.

  await testValidInput(
    "Variable shadowing.",
    `
      int aaa;
      int main( ) {
        int aaa;
        return 0;
      }
    `
  );

  await testValidInput(
    "Variable shadowing function declaration.",
    `
      int aaa ( ) { return 0; }
      int main( ) {
        int aaa;
        return 0;
      }
    `
  );

  await testValidInput(
    "Variable shadowing vector declaration.",
    `
      int aaa[5];
      int main( ) {
        int aaa;
        return 0;
      }
    `
  );

  // O comando input deve ser seguido obrigatoriamente por um identificador do tipo int e float.
  // Caso contrário, o compilador deve lançar o erro ERR_WRONG_PAR_INPUT.

  await testInvalidInput(
    "when input command receives char, should fail",
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

  await testInvalidInput(
    "when input command receives string, should fail",
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

  await testInvalidInput(
    "when input command receives bool, should fail",
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

  await testValidInput(
    "when input command receives int, should succeed",
    `
      int main() {
        int a;
        input a;
        return 0;
      }
    `
  );

  await testValidInput(
    "when input command receives float, should succeed",
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

  await testInvalidInput(
    "when output command receives bool, should fail",
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

  await testInvalidInput(
    "when output command receives char, should fail",
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

  await testInvalidInput(
    "when output command receives string, should fail",
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

  await testValidInput(
    "when output command receives int, should succeed",
    `
      int main() {
        int a;
        output a;
        return 0;
      }
    `
  );

  await testValidInput(
    "when output command receives float, should succeed",
    `
      int main() {
        float a;
        output a;
        return 0;
      }
    `
  );

  await testInvalidInput(
    "when output command receives literal string, should fail",
    `
      int main() {
        output \"string\";
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_OUTPUT,
    `Invalid argument for "output" command; expected variable or literal of type "int" or "float", found "string";
Occurrence at line 3, column 16:
        output "string";
               ^^^^^^^^`
  );

  await testInvalidInput(
    "when output command receives literal char, should fail",
    `
      int main() {
        output \'c\';
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_OUTPUT,
    `Invalid argument for "output" command; expected variable or literal of type "int" or "float", found "char";
Occurrence at line 3, column 16:
        output 'c';
               ^^^`
  );

  await testInvalidInput(
    "when output command receives false, should fail",
    `
      int main() {
        output false;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_OUTPUT,
    `Invalid argument for "output" command; expected variable or literal of type "int" or "float", found "bool";
Occurrence at line 3, column 16:
        output false;
               ^^^^^`
  );

  await testInvalidInput(
    "when output command receives true, should fail",
    `
      int main() {
        output true;
        return 0;
      }
    `,
    ERROR_CODE.ERR_WRONG_PAR_OUTPUT,
    `Invalid argument for "output" command; expected variable or literal of type "int" or "float", found "bool";
Occurrence at line 3, column 16:
        output true;
               ^^^^`
  );

  await testValidInput(
    "when output command receives literal int, should succeed",
    `
      int main() {
        output 0;
        return 0;
      }
    `
  );

  await testValidInput(
    "when output command receives literal float, should succeed",
    `
      int main() {
        output 0.0;
        return 0;
      }
    `
  );

  // A Sistema de tipos da Linguagem
  // Regras de Escopo.
  // A verificação de declaração prévia de tipos deve considerar o escopo da linguagem.
  // O escopo pode ser global, local da função e local de um bloco, sendo que este pode ser recursivamente aninhado.
  // Uma forma de se implementar estas regras de escopo é através de uma pilha de tabelas de símbolos.
  // Para verificar se uma variável foi declarada, verificase primeiramente no escopo atual (topo da pilha) e enquanto não encontrar, deve-se descer na pilha até chegar no escopo global (base da pilha, sempre presente).

  await testValidInput(
    "Variable shadowing with command block.",
    `
      int aaa;
      int main( ) {
        int aaa;
        {
          int aaa;
          return 0;
        };
      }
    `
  );

  await testInvalidInput(
    "Variable redefinition inside command block.",
    `
      int aaa;
      int main( ) {
        int aaa;
        {
          int aaa;
          int aaa;
          return 0;
        };
      }
      `,
    ERROR_CODE.ERR_DECLARED,
    `Same-scope identifier redeclaration: "aaa"
First occurrence at line 6, column 15:
          int aaa;
              ^^^
And again at line 7, column 15:
          int aaa;
              ^^^`
  );

  // Enfim, vetores não podem ser do tipo string.
  // Caso um vetor tenha sido declarado com o tipo string, o erro ERR_STRING_VECTOR deve ser lançado.

  await testValidInput(
    "Declaring a vector of float types.",
    `
      float aaa[1];
    `
  );

  await testInvalidInput(
    "Declaring a vector of string types.",
    `
    string aaa[1];
    `,
    ERROR_CODE.ERR_STRING_VECTOR,
    `Invalid usage of "string" type for vector declaration: "aaa"
Occurrence at line 2, column 5:
    string aaa[1];
    ^^^^^^^^^^^^^`
  );

  // Conversão implícita.
  // As regras de coerção de tipos da Linguagem são as seguintes.
  // Não há conversão implícita para os tipos string e char.
  // Um tipo int pode ser convertido implicitamente para float e para bool.
  // Um tipo bool pode ser convertido implicitamente para float e para int.
  // Um tipo float pode ser convertido implicitamente para int e para bool, perdendo precisão.
  // Inferência.
  // As regras de inferência de tipos da linguagem são as seguintes.
  // A partir de int e int, inferese int.
  // A partir de float e float, infere-se float.
  // A partir de bool e bool, infere-se bool.
  // A partir de float e int, infere-se float.
  // A partir de bool e int, infere-se int.
  // A partir de bool e float, infere-se float.
  // A matriz abaixo resume:

  // Unary positive:

  await testValidInput(
    "Valid unary positive with literal int.",
    `
    int main() {
      int aaa;
      aaa = + 1;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary positive with var int.",
    `
    int main() {
      int aaa;
      int bbb <= 1;
      aaa = +bbb;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary positive with literal float.",
    `
    int main() {
      int aaa;
      aaa = +1.1;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary positive with var float.",
    `
    float main() {
      float aaa;
      float bbb <= 1.1;
      aaa = +bbb;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary positive with literal bool.",
    `
    int main() {
      int aaa;
      aaa = +true;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary positive with var bool.",
    `
    int main() {
      int aaa;
      bool bbb <= true;
      aaa = +bbb;
      return aaa;
    }
  `
  );

  await testInvalidInput(
    "Invalid unary positive with literal char.",
    `
    int main() {
      char aaa;
      aaa = +'1';
      return 0;
    }
  `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "int or float"
Occurrence at line 4, column 13:
      aaa = +'1';
            ^`
  );

  await testInvalidInput(
    "Invalid unary positive with var char.",
    `
    int main() {
      char aaa;
      char bbb <= '1';
      aaa = +bbb;
      return 0;
    }
  `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "int or float"
Occurrence at line 5, column 13:
      aaa = +bbb;
            ^`
  );

  await testInvalidInput(
    "Invalid unary positive with literal string.",
    `
    int main() {
      string aaa <= " ";
      aaa = +"1";
      return 0;
    }
  `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "int or float"
Occurrence at line 4, column 13:
      aaa = +"1";
            ^`
  );

  await testInvalidInput(
    "Invalid unary positive with var string.",
    `
    int main() {
      string aaa <= " ";
      string bbb <= "1";
      aaa = +bbb;
      return 0;
    }
  `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "int or float"
Occurrence at line 5, column 13:
      aaa = +bbb;
            ^`
  );

  // Unary negative:

  await testValidInput(
    "Valid unary negative with literal int.",
    `
    int main() {
      int aaa;
      aaa = -1;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary negative with var int.",
    `
      int main() {
        int aaa;
        int bbb <= 1;
        aaa = -bbb;
        return aaa;
      }
    `
  );

  await testValidInput(
    "Valid unary negative with literal float.",
    `
    float main() {
      float aaa;
      aaa = -1.1;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary negative with var float.",
    `
      float main() {
        float aaa;
        float bbb <= 1.1;
        aaa = -bbb;
        return aaa;
      }
    `
  );

  await testValidInput(
    "Valid unary negative with literal bool.",
    `
    int main() {
      int aaa;
      aaa = - true;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary negative with var bool.",
    `
    int main() {
      int aaa;
      bool bbb <= true;
      aaa = -bbb;
      return aaa;
    }
  `
  );

  await testInvalidInput(
    "Invalid unary negative with literal char.",
    `
    int main() {
      char aaa;
      aaa = - '1';
      return 0;
    }
  `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "int or float"
Occurrence at line 4, column 13:
      aaa = - '1';
            ^`
  );

  await testInvalidInput(
    "Invalid unary negative with var char.",
    `
    int main() {
      char aaa;
      char bbb <= '1';
      aaa = -bbb;
      return 0;
    }
  `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "int or float"
Occurrence at line 5, column 13:
      aaa = -bbb;
            ^`
  );

  await testInvalidInput(
    "Invalid unary negative with literal string.",
    `
    int main() {
      string aaa <= " ";
      aaa = - "1";
      return 0;
    }
  `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "int or float"
Occurrence at line 4, column 13:
      aaa = - "1";
            ^`
  );

  await testInvalidInput(
    "Invalid unary negative with var string.",
    `
    int main() {
      string aaa <= " ";
      string bbb <= "1";
      aaa = -bbb;
      return 0;
    }
  `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "int or float"
Occurrence at line 5, column 13:
      aaa = -bbb;
            ^`
  );

  // Unary negation:

  await testValidInput(
    "Valid unary negation with literal int.",
    `
    bool main() {
      bool aaa;
      aaa = !1;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary negative with var int.",
    `
      bool main() {
        bool aaa;
        int bbb <= 1;
        aaa = !bbb;
        return aaa;
      }
    `
  );

  await testValidInput(
    "Valid unary negation with literal float.",
    `
    bool main() {
      bool aaa;
      aaa = !1.1;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary negation with var float.",
    `
      bool main() {
        bool aaa;
        float bbb <= 1.1;
        aaa = !bbb;
        return aaa;
      }
    `
  );

  await testValidInput(
    "Valid unary negation with literal bool.",
    `
    bool main() {
      bool aaa;
      aaa = !true;
      return aaa;
    }
  `
  );

  await testValidInput(
    "Valid unary negation with var bool.",
    `
    bool main() {
      bool aaa;
      int bbb <= 1;
      aaa = !bbb;
      return aaa;
    }
  `
  );

  await testInvalidInput(
    "Invalid unary negation with literal char.",
    `
    int main() {
      bool aaa;
      aaa = !'1';
      return 0;
    }
  `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "bool"
Occurrence at line 4, column 13:
      aaa = !'1';
            ^`
  );

  await testInvalidInput(
    "Invalid unary negation with var char.",
    `
      int main() {
        bool aaa;
        char bbb <= '1';
        aaa = !bbb;
        return 0;
      }
    `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "bool"
Occurrence at line 5, column 15:
        aaa = !bbb;
              ^`
  );

  await testInvalidInput(
    "Invalid unary negation with literal string.",
    `
    int main() {
      bool aaa;
      aaa = !"1";
      return 0;
    }
  `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "bool"
Occurrence at line 4, column 13:
      aaa = !"1";
            ^`
  );

  await testInvalidInput(
    "Invalid unary negation with var string.",
    `
    int main() {
      bool aaa;
      string bbb <= "1";
      aaa = !bbb;
      return 0;
    }
  `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "bool"
Occurrence at line 5, column 13:
      aaa = !bbb;
            ^`
  );

  // Unary boolean:

  await testValidInput(
    "Valid unary boolean with literal int.",
    `
      bool main() {
        bool aaa;
        aaa = ?1;
        return aaa;
      }
    `
  );

  await testValidInput(
    "Valid unary boolean with var int.",
    `
        bool main() {
          bool aaa;
          int bbb <= 1;
          aaa = ?bbb;
          return aaa;
        }
      `
  );

  await testValidInput(
    "Valid unary boolean with literal float.",
    `
      bool main() {
        bool aaa;
        aaa = ?1.1;
        return aaa;
      }
    `
  );

  await testValidInput(
    "Valid unary boolean with var float.",
    `
        bool main() {
          bool aaa;
          float bbb <= 1.1;
          aaa = ?bbb;
          return aaa;
        }
      `
  );

  await testValidInput(
    "Valid unary boolean with literal bool.",
    `
      bool main() {
        bool aaa;
        aaa = ?true;
        return aaa;
      }
    `
  );

  await testValidInput(
    "Valid unary boolean with var bool.",
    `
      bool main() {
        bool aaa;
        int bbb <= 1;
        aaa = ?bbb;
        return aaa;
      }
    `
  );

  await testInvalidInput(
    "Invalid unary boolean with literal char.",
    `
    int main() {
      bool aaa;
      aaa = ?'1';
      return 0;
    }
  `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "bool"
Occurrence at line 4, column 13:
      aaa = ?'1';
            ^`
  );

  await testInvalidInput(
    "Invalid unary boolean with var char.",
    `
        int main() {
          bool aaa;
          char bbb <= '1';
          aaa = ?bbb;
          return 0;
        }
      `,
    ERROR_CODE.ERR_CHAR_TO_X,
    `Invalid type conversion from "char" to "bool"
Occurrence at line 5, column 17:
          aaa = ?bbb;
                ^`
  );

  await testInvalidInput(
    "Invalid unary boolean with literal string.",
    `
    int main() {
      bool aaa;
      aaa = ?"1";
      return 0;
    }
  `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "bool"
Occurrence at line 4, column 13:
      aaa = ?"1";
            ^`
  );

  await testInvalidInput(
    "Invalid unary boolean with var string.",
    `
      int main() {
        bool aaa;
        string bbb <= "1";
        aaa = ?bbb;
        return 0;
      }
    `,
    ERROR_CODE.ERR_STRING_TO_X,
    `Invalid type conversion from "string" to "bool"
Occurrence at line 5, column 15:
        aaa = ?bbb;
              ^`
  );

  // Nos comandos de shift (esquerda e direta), deve-se lançar o erro ERR_WRONG_PAR_SHIFT caso o parâmetro após o token de shift for um número maior que 16.

  await testInvalidInput(
    "Left shift with a value above 16 on a variable.",
    `
    int main() {
      int a;
      a << 17;
      return 0;
    }
    `,
    ERROR_CODE.ERR_WRONG_PAR_SHIFT,
    `Invalid number parameter on shift command; expected number lower or equal to 16, found "17";
Occurrence at line 4, column 12:
      a << 17;
           ^^`
  );

  await testInvalidInput(
    "Right shift with a value above 16 on a variable.",
    `
    int main() {
      int a;
      a >> 17;
      return 0;
    }
    `,
    ERROR_CODE.ERR_WRONG_PAR_SHIFT,
    `Invalid number parameter on shift command; expected number lower or equal to 16, found "17";
Occurrence at line 4, column 12:
      a >> 17;
           ^^`
  );

  await testInvalidInput(
    "Left shift with a value above 16 on a vector.",
    `
    int a[1];
    int main() {
      a[0] << 17;
      return 0;
    }
    `,
    ERROR_CODE.ERR_WRONG_PAR_SHIFT,
    `Invalid number parameter on shift command; expected number lower or equal to 16, found "17";
Occurrence at line 4, column 15:
      a[0] << 17;
              ^^`
  );

  await testInvalidInput(
    "Right shift with a value above 16 on a vector.",
    `
    int a[1];
    int main() {
      a[0] >> 17;
      return 0;
    }
    `,
    ERROR_CODE.ERR_WRONG_PAR_SHIFT,
    `Invalid number parameter on shift command; expected number lower or equal to 16, found "17";
Occurrence at line 4, column 15:
      a[0] >> 17;
              ^^`
  );

  await testValidInput(
    "Right shift with a value below 16 on a vector.",
    `
    int a[1];
    int main() {
      a[0] >> 15;
      return 0;
    }
    `
  );

  await testValidInput(
    "Left shift with a value below 16 on a vector.",
    `
    int a[1];
    int main() {
      a[0] << 15;
      return 0;
    }
    `
  );

  await testValidInput(
    "Right shift with a value below 16 on a variable.",
    `
    int main() {
      int a;
      a >> 15;
      return 0;
    }
    `
  );

  await testValidInput(
    "Left shift with a value below 16 on a variable.",
    `
    int main() {
      int a;
      a << 15;
      return 0;
    }
    `
  );

  // Tamanho.
  // O tamanho dos tipos da linguagem é definido da seguinte forma.
  // Um char ocupa 1 byte.
  // Um string ocupa 1 byte para cada caractere.
  // Um int ocupa 4 bytes.
  // Um float ocupa 8 bytes.
  // Um bool ocupa 1 byte.
  // Um vetor ocupa o seu tamanho vezes o seu tipo.

  // O tamanho máximo de um string é definido na sua inicialização (com o operador de inicialização).
  // Uma string não inicializada ocupa 0 bytes e não pode receber valores cujo tamanho excede àquele máximo da inicialização.
  // Caso o tamanho de um string a ser atribuído exceder o máximo, deve-se emitir o erro ERR_STRING_MAX.

  await testInvalidInput(
    "Setting a literal string larger than its allocation",
    `
    int main() {
      string s <= "123";
      s = "1234";
      return 0;
    }
    `,
    ERROR_CODE.ERR_STRING_MAX,
    `Invalid attribution of type "string" value, size exceeds that of variable declaration.
Variable declaration size is 3 and string size is 4.
Occurrence at line 4, column 11:
      s = "1234";
          ^^^^^^`
  );

  await testInvalidInput(
    "Setting a literal string to an unintialized string",
    `
    int main() {
      string s;
      s = "1";
      return 0;
    }
    `,
    ERROR_CODE.ERR_STRING_MAX,
    `Invalid attribution of type "string" value, size exceeds that of variable declaration.
Variable declaration size is 0 and string size is 1.
Occurrence at line 4, column 11:
      s = "1";
          ^^^`
  );

  await testValidInput(
    "Setting a literal valid string to a string variable",
    `
    int main() {
      string s <= "1";
      s = "0";
      return 0;
    }
    `
  );

  // Prevalece o tipo do identificador que recebe um valor em um comando de atribuição.
  // O erro ERR_WRONG_TYPE deve ser lançado quando o tipo do valor a ser atribuído a um identificador for incompatível com o tipo deste identificador.

  for (const isInitialization of [true, false]) {
    for (const usingVariable of [true, false]) {
      for (const type of ["int", "bool", "float"]) {
        await testInvalidInput(
          `Initializing a ${type} variable with an literal char [${isInitialization}, ${usingVariable}]`,
          `
    int main() {
      ${
        isInitialization
          ? usingVariable
            ? `char c <= 'c'; ${type} a <= c;`
            : `${type} a <= 'c';`
          : usingVariable
          ? `char c <= 'c'; ${type} a; a = c;`
          : `${type} a; a = 'c';`
      }
      return 0;
    }
    `,
          ERROR_CODE.ERR_WRONG_TYPE,
          `Incompatible type in attribution.
Expected int, float or bool but received a "char".`
        );

        await testInvalidInput(
          `Initializing a ${type} variable with an literal string [${isInitialization}, ${usingVariable}]`,
          `
    int main() {
      ${
        isInitialization
          ? usingVariable
            ? `string s <= "string"; ${type} a <= s;`
            : `${type} a <= "string";`
          : usingVariable
          ? `string s <= "string"; ${type} a; a = s;`
          : `${type} a; a = "string";`
      }
      return 0;
    }
    `,
          ERROR_CODE.ERR_WRONG_TYPE,
          `Incompatible type in attribution.
Expected int, float or bool but received a "string".`
        );

        for (const value of [
          { type: "int", value: "0" },
          { type: "bool", value: "true" },
          { type: "bool", value: "false" },
          { type: "float", value: "0.0" },
        ]) {
          await testValidInput(
            `Initializing a int variable with an literal ${value.type} [${isInitialization}, ${usingVariable}]`,
            `
      int main() {
        ${
          isInitialization
            ? usingVariable
              ? `${value.type} v <= ${value.value}; ${type} a <= v;`
              : `${type} a <= ${value.value};`
            : usingVariable
            ? `${value.type} v <= ${value.value}; ${type} a; a = v;`
            : `${type} a; a = ${value.value};`
        }
        return 0;
      }
      `
          );
        }
      }

      for (const value of [
        { type: "int", value: "0" },
        { type: "bool", value: "true" },
        { type: "bool", value: "false" },
        { type: "float", value: "0.0" },
        { type: "char", value: "'c'" },
      ]) {
        await testInvalidInput(
          `Initializing a string variable with an literal ${value.type} [${isInitialization}, ${usingVariable}]`,
          `
    int main() {
      ${
        isInitialization
          ? usingVariable
            ? `${value.type} v <= ${value.value}; string s <= v;`
            : `string s <= ${value.value};`
          : usingVariable
          ? `${value.type} v <= ${value.value}; string s <= "123456"; s = v;`
          : `string s <= "123456"; s = ${value.value};`
      }
      return 0;
    }
    `,
          ERROR_CODE.ERR_STRING_TO_X,
          `Invalid type conversion from "string" to "${value.type}"`
        );
      }

      await testValidInput(
        `Initializing a string variable with an literal string [${isInitialization}, ${usingVariable}]`,
        `
  int main() {
    ${
      isInitialization
        ? usingVariable
          ? `string x <= "string"; string y <= x;`
          : `string s <= "string";`
        : usingVariable
        ? `string x <= "string"; string y <= "string"; x = y;`
        : `string s <= "string"; s = "string";`
    }
    return 0;
  }
  `
      );

      for (const value of [
        { type: "int", value: "0" },
        { type: "bool", value: "true" },
        { type: "bool", value: "false" },
        { type: "float", value: "0.0" },
        { type: "string", value: '"string"' },
      ]) {
        await testInvalidInput(
          `Initializing a char variable with an literal ${value.type} [${isInitialization}, ${usingVariable}]`,
          `
    int main() {
      ${
        isInitialization
          ? usingVariable
            ? `${value.type} v <= ${value.value}; char c <= v;`
            : `char c <= ${value.value};`
          : usingVariable
          ? `${value.type} v <= ${value.value}; char c; c = v;`
          : `char c; c = ${value.value};`
      }
      return 0;
    }
    `,
          ERROR_CODE.ERR_CHAR_TO_X,
          `Invalid type conversion from "char" to "${value.type}"`
        );
      }

      await testValidInput(
        `Initializing a char variable with an literal char [${isInitialization}, ${usingVariable}]`,
        `
  int main() {
    ${
      isInitialization
        ? usingVariable
          ? `char x <= 'c'; char y <= x;`
          : `char c <= 'c';`
        : usingVariable
        ? `char x <= 'c'; char y; y = x;`
        : `char c; c = 'c';`
    }
    return 0;
  }
  `
      );
    }
  }
}

var start = new Date().getTime();
main().then(() => {
  var end = new Date().getTime();
  var totalInMs = end - start;
  console.log(`Completed in ${(totalInMs / 1000.0).toFixed(1)} seconds.`);
});

/*
  TODO = EACH NEW TEST SHOULD REMOVE A SPECIFICATION LINE BELOW.
  The project will be complete when there are no untested specification line.

  O processo de inferência de tipos está descrito abaixo.
  Como não temos coerção de variáveis do tipo string e char, o compilador deve lançar o erro ERR_STRING_TO_X quando a variável do tipo string estiver em uma situação onde ela deve ser convertida para qualquer outro tipo.
  De maneira análoga, o erro ERR_CHAR_TO_X deve ser lançado quando uma variável do tipo char deve ser convertida implicitamente.

  2.5 Retorno, argumentos e parâmetros de funções
  A lista de argumentos fornecidos em uma chamada de função deve ser verificada contra a lista de parâmetros formais na declaração da mesma função.
  Cada chamada de função deve prover um argumento para cada parâmetro, e ter o seu tipo compatível.
  Tais verificações devem ser realizadas levando-se em conta as informações registradas na tabela de símbolos, registradas no momento da declaração/definição da função.
  Na hora da chamada da função, caso houver um número menor de argumentos que o necessário, deve-se lançar o erro ERR_MISSING_ARGS.
  Caso houver um número maior de argumentos que o necessário, deve-se lançar o erro ERR_EXCESS_ARGS.
  Enfim, quando o número de argumentos é correto, mas os tipos dos argumentos são incompatíveis com os tipos registrados na tabela de símbolo, deve-se lançar o erro ERR_WRONG_TYPE_ARGS.
  Retorno, argumentos e parâmetros de funções não podem ser do tipo string.
  Quando estes casos acontecerem, lançar o erro ERR_FUNCTION_STRING.

  2. Verificação de tipos em comandos
 
  Os demais comandos simples da linguagem devem ser verificados semanticamente para obedecer as seguintes regras.
  O comando de retorno return deve ser seguido obrigatoriamente por uma expressão cujo tipo é compatível com o tipo de retorno da função.
  Caso não seja o caso, o erro ERR_WRONG_PAR_RETURN deve ser lançado pelo compilador.


*/

/*
  NÃO-Testáveis + Testes manuais
  // Checar essa lista antes de enviar o trabalho

  1 Introdução

  A quarta etapa do trabalho de implementação de um compilador para a linguagem consiste em verificações semânticas.
  Estas verificações fazem parte do sistema de tipos da linguagem com um conjunto de regras detalhado a seguir.
  Toda a verificação de tipos é feita em tempo de compilação.
  Todos os nós da Arvore Sintática Abstrata (AST), é gerada na E3, terão agora um novo campo que indica o seu tipo (se e inteiro, ponto-flutuante, etc).
  O tipo de um determinado no da AST pode, em algumas situações, não ser definido diretamente (para os comandos de fluxo de controle, por exemplo).
  Na maioria dos casos, no entanto, seu tipo e definido seguindo as regras de inferência da linguagem.

  2 Funcionalidades Necessárias

  2.1 Implementar uma tabela de símbolos

  A tabela de símbolos guarda informações a respeito dos símbolos (identificadores e literais) encontrados na entrada.
  Cada entrada na tabela de símbolos tem uma chave e um conteúdo.
  A chave única identifica o símbolo, e o conteudo deve ter os campos:
  • localização (linha e coluna, esta opcional)
  • natureza (literal, variavel, função, etc)
  • tipo (qual o tipo de dado deste símbolo)
  • tamanho (derivado do tipo e se vetor)
  • argumentos e seus tipos (no caso de funções)
  • dados do valor do token pelo yylval (veja E3)
  A implementação deve prever que várias tabelas de símbolos possam coexistir, uma para cada escopo.
  As regras de escopo sao delineadas a no anexo.

  2.4 Verificação de tipos e tamanho dos dados
  Uma declaração de variável deve permitir ao compilador definir o tipo e a ocupação em memória da variável na sua entrada na tabela de símbolos.
  Com o auxílio desta informação, quando necessário, os tipos de dados corretos devem ser inferidos onde forem usados, em expressões aritméticas, relacionais, lógicas, ou para índices de vetores.
  Por simplificação, isso nos leva a situação onde todos os nós da AST devem ser anotados com um tipo definido de acordo com as regras de inferência de tipos.
  Um nó da AST deve ter portanto um novo campo que registra o seu tipo de dado.

  2.7 Mensagens de erro
  Mensagens de erro significativas devem ser fornecidas.
  Elas devem descrever em linguagem natural o erro semântico, as linhas envolvidas, os identificadores e a natureza destes.

  C Arquivo main.c
  Utilize o mesmo main.c da E3.
  Cuide da alocação dinâmica das tabelas.
*/
