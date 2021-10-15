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

  // Test 23: expected literal int or float, received string
  await testInvalidInput(
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

  // Test 24: expected literal int or float, received char
  await testInvalidInput(
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

  // Test 25: expected literal int or float, received bool
  await testInvalidInput(
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

  // Test 26: expected literal int or float, received bool
  await testInvalidInput(
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

  // Test 27: valid literal int
  await testValidInput(
    `
      int main() {
        output 0;
        return 0;
      }
    `
  );

  // Test 28: valid literal float
  await testValidInput(
    `
      int main() {
        output 0.0;
        return 0;
      }
    `
  );
}

main();

/*
  TODO = EACH NEW TEST SHOULD REMOVE A SPECIFICATION LINE BELOW.
  The project will be complete when there are no untested specification line.

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

  2.2 Verificação de declarações
  Todos os identificadores devem ter sido declarados no momento do seu uso, seja como variável, como vetor ou como função.
  Todas as entradas na tabela de símbolos devem ter um tipo associado conforme a declaração, verificando-se se não houve dupla declaração ou se o símbolo nao foi declarado.
  Caso o identificador ja tenha sido declarado, deve-se lançar o erro ERR_DECLARED.
  Caso o identificador não tenha sido declarado no seu uso, deve-se lançar o erro ERR_UNDECLARED.
  Variáveis com o mesmo nome podem co-existir em escopos diferentes, efetivamente mascarando as variaveis que estão em escopos superiores.
  As regras de escopo sao delineadas no anexo.

  2.3 Uso correto de identificadores
  O uso de identificadores deve ser compatível com sua declaração e com seu tipo.
  Variáveis somente podem ser usadas sem indexação, vetores somente podem ser utilizados com indexação, e funções apenas devem ser usadas como chamada de função, isto é, seguidas da lista de argumentos, esta possivelmente vazia conforme a sintaxe da E2, entre parênteses.
  Caso o identificador dito variável seja usado como vetor ou como função, deve-se lançar o erro ERR_VARIABLE.
  Caso o identificador dito vetor seja usado como variável ou função, deve-se lançar o erro ERR_VECTOR.
  Enfim, caso o identificador dito função seja utilizado como variável ou vetor, deve-se lançar o erro ERR_FUNCTION.

  2.4 Verificação de tipos e tamanho dos dados
  Uma declaração de variável deve permitir ao compilador definir o tipo e a ocupação em memória da variável na sua entrada na tabela de símbolos.
  Com o auxílio desta informação, quando necessário, os tipos de dados corretos devem ser inferidos onde forem usados, em expressões aritméticas, relacionais, lógicas, ou para índices de vetores.
  Por simplificação, isso nos leva a situação onde todos os nós da AST devem ser anotados com um tipo definido de acordo com as regras de inferência de tipos.
  Um nó da AST deve ter portanto um novo campo que registra o seu tipo de dado.

  O processo de inferência de tipos está descrito abaixo.
  Como não temos coerção de variáveis do tipo string e char, o compilador deve lançar o erro ERR_STRING_TO_X quando a variável do tipo string estiver em uma situação onde ela deve ser convertida para qualquer outro tipo.
  De maneira análoga, o erro ERR_CHAR_TO_X deve ser lançado quando uma variável do tipo char deve ser convertida implicitamente.
  Enfim, vetores não podem ser do tipo string.
  Caso um vetor tenha sido declarado com o tipo string, o erro ERR-STRING-VECTOR deve ser lançado.

  2.5 Retorno, argumentos e parâmetros de funções
  A lista de argumentos fornecidos em uma chamada de função deve ser verificada contra a lista de parâmetros formais na declaração da mesma função.
  Cada chamada de função deve prover um argumento para cada parâmetro, e ter o seu tipo compatível.
  Tais verificações devem ser realizadas levando-se em conta as informações registradas na tabela de símbolos, registradas no momento da declaração/definição da função.
  Na hora da chamada da função, caso houver um número menor de argumentos que o necessário, deve-se lançar o erro ERR-MISSING_ARGS.
  Caso houver um número maior de argumentos que o necessário, deve-se lançar o erro ERR_EXCESS_ARGS.
  Enfim, quando o número de argumentos é correto, mas os tipos dos argumentos são incompatíveis com os tipos registrados na tabela de símbolo, deve-se lançar o erro ERR_WRONG_TYPE_ARGS.
  Retorno, argumentos e parâmetros de funções não podem ser do tipo string.
  Quando estes casos acontecerem, lançar o erro ERR_FUNCTION_STRING.

  2. Verificação de tipos em comandos
  Prevalece o tipo do identificador que recebe um valor em um comando de atribuição.
  O erro ERR_WRONG_TYPE deve ser lançado quando o tipo do valor a ser atribuído a um identificador for incompatível com o tipo deste identificador.
  Os demais comandos simples da linguagem devem ser verificados semanticamente para obedecer as seguintes regras.
  O comando input deve ser seguido obrigatoriamente por um identificador do tipo int e float.
  Caso contrário, o compilador deve lançar o erro ERR_WRONG_PAR_INPUT.
  De maneira análoga, o comando output deve ser seguido por um identificador ou literal do tipo int e float.
  Caso contrário, deve ser lançado o erro ERR_WRONG_PAR_OUTPUT.
  O comando de retorno return deve ser seguido obrigatoriamente por uma expressão cujo tipo é compatível com o tipo de retorno da função.
  Caso não seja o caso, o erro ERR_WRONG_PAR RETURN deve ser lançado pelo compilador.
  Nos comandos de shift (esquerda e direta), deve-se lançar o erro ERR_WRONG_PAR_SHIFT caso o parâmetro após o token de shift for um número maior que 16.

  2.7 Mensagens de erro
  Mensagens de erro significativas devem ser fornecidas.
  Elas devem descrever em linguagem natural o erro semântico, as linhas envolvidas, os identificadores e a natureza destes.

  A Sistema de tipos da Linguagem
  Regras de Escopo.
  A verificação de declaração prévia de tipos deve considerar o escopo da linguagem.
  O escopo pode ser global, local da função e local de um bloco, sendo que este pode ser recursivamente aninhado.
  Uma forma de se implementar estas regras de escopo é através de uma pilha de tabelas de símbolos.
  Para verificar se uma variável foi declarada, verificase primeiramente no escopo atual (topo da pilha) e enquanto não encontrar, deve-se descer na pilha até chegar no escopo global (base da pilha, sempre presente).
  Caso o identificador não seja encontrado, isso indica que ele não foi declarado.
  Para se "declarar"um símbolo, basta inseri-lo na tabela de símbolos do escopo que encontra-se no topo da pilha.

  Conversão implícita.
  As regras de coerção de tipos da Linguagem são as seguintes.
  Não há conversão implícita para os tipos string e char.
  Um tipo int pode ser convertido implicitamente para float e para bool.
  Um tipo bool pode ser convertido implicitamente para float e para int.
  Um tipo float pode ser convertido implicitamente para int e para bool, perdendo precisão.
  Inferência.
  As regras de inferência de tipos da linguagem são as seguintes.
  A partir de int e int, inferese int.
  A partir de float e float, infere-se float.
  A partir de bool e bool, infere-se bool.
  A partir de float e int, infere-se float.
  A partir de bool e int, infere-se int.
  A partir de bool e float, infere-se float.
  A matriz abaixo resume:

  Tamanho.
  O tamanho dos tipos da linguagem é definido da seguinte forma.
  Um char ocupa 1 byte.
  Um string ocupa 1 byte para cada caractere.
  O tamanho máximo de um string é definido na sua inicialização (com o operador de inicialização).
  Uma string não inicializada ocupa 0 bytes e não pode receber valores cujo tamanho excede àquele máximo da inicialização.
  Caso o tamanho de um string a ser atribuído exceder o máximo, deve-se emitir o erro ERR-STRING-MAX.
  Um int ocupa 4 bytes.
  Um float ocupa 8 bytes.
  Um bool ocupa 1 byte.
  Um vetor ocupa o seu tamanho vezes o seu tipo.

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

  C Arquivo main.c
  Utilize o mesmo main.c da E3.
  Cuide da alocação dinâmica das tabelas.
*/
