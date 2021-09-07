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
        sleep 0.001

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
        exit
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

basicTypes=("int" "char" "float" "bool" "string")
literalValues=("1" "'c'" "1.0" "true" "false" "\"string\"")

# global variable declarations
for type in ${basicTypes[@]}; do
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

for basicType in ${basicTypes[@]}; do
    testValidInput "$basicType functionName() { }"
    testValidInput "$basicType functionName($basicType a) { }"
    testValidInput "static $basicType functionName() { }"
    testValidInput "static $basicType functionName($basicType a) { }"
done

# Command block / commands
for basicType in ${basicTypes[@]}; do
    testValidInput "int f() { $basicType id; }"
    testValidInput "int f() { static $basicType id; }"
    testValidInput "int f() { const $basicType id; }"
    testValidInput "int f() { static const $basicType id; }"
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

# Comando de Atribuição

# Existe apenas uma forma de atribuição para identificadores.
# Identificadores podem receber valores assim
# (primeiro caso de um identificador simples; segundo caso de um identificador que e um vetor):
# identificador = expressão
# identificador[expressão] = expressão

expressionExamples=("id" "1" "1.0" "f()" "1+id" "id1==id2")

for expressionExample in ${expressionExamples[@]}; do
    testValidInput "int f() { id = $expressionExample; }"
done

for expressionExample in ${expressionExamples[@]}; do
    testValidInput "int f() { id[1] = $expressionExample; }"
done

for expressionExample in ${expressionExamples[@]}; do
    testValidInput "int f() { id[$expressionExample] = 1; }"
done

testValidInput "int f() { int id1 <= id2; int id1 <= id2; }"
testValidInput "int f() { id = 1; id = 3; }"
testValidInput "int f() { id = 2; int id1 <= 4; }"
testValidInput "int f() { int id1 <= id2; id = 7; }"

testValidInput "int f() { input id1; }"
testValidInput "int f() { output id1; }"

for literalValue in ${literalValues[@]}; do
    testValidInput "int f() { output $literalValue; }"
done

# Chamada de função
# Uma chamada de funcão consiste no nome da função, seguida de argumentos entre parenteses separados por vírgula. [...]

testValidInput "int f() { funcName(); }"
testValidInput "int f() { funcName(id1); }"
testValidInput "int f() { funcName(id1, id2, id3); }"

for literalValue in ${literalValues[@]}; do
    testValidInput "int f() { funcName($literalValue); }"
done

testValidInput "int f() { funcName(1, 'z', 5.0); }"

# Um argumento pode ser uma expressao. 
testValidInput "int f() { funcName(1 + 1 + 1); }"

# Comandos de Shift

# Sendo numero um literal inteiro positivo, temos os exemplos válidos abaixo.
# Os exemplos são dados com <<, mas as entradas são sintaticamente válidas tambem para >>.
# O numero deve ser representado por um literal inteiro.
# identificador << número
# identificador[expressão] << número

testValidInput "int f() { id << 1; }"
testValidInput "int f() { id >> 1; }"
testValidInput "int f() { id[1] << 1; }"
testValidInput "int f() { id[1] >> 1; }"

testValidInput "int f() { id << +1; }"
testValidInput "int f() { id >> +1; }"
testValidInput "int f() { id[1] << +1; }"
testValidInput "int f() { id[1] >> +1; }"

testValidInput "int f() { id[1 + 1] << +1; }"

# TODO: Block negative values
# testInvalidInput "int f() { id << -1; }"
# testInvalidInput "int f() { id >> -1; }"
# testInvalidInput "int f() { id[1] << -1; }"
# testInvalidInput "int f() { id[1] >> -1; }"

# Comando de Retorno, Break, Continue

# Retorno é a palavra reservada return seguida de uma expressão.
testValidInput "int f() { return 5 + 5; }"

# Os comandos break e continue sao simples.
testValidInput "int f() { break; }"
testValidInput "int f() { continue; }"

# Comandos de Controle de Fluxo

# A linguagem possui construções condicionais, iterativas e de
# seleção para controle estruturado de fluxo. As condicionais
# incluem o if com o else opcional, assim:
# if (<expressão>) bloco
# if (<expressão>) bloco else bloco

testValidInput "int f() { if (1 + 1) { } }"
testValidInput "int f() { if (1 + 1) { id = 1; } }"
testValidInput "int f() { if (5 + 5 + 5) { id = 1; } }"
testValidInput "int f() { if (1 + 1) { id = 1; } else { id = 2; } }"

# As construções iterativas são as seguintes no formato:
# for (atrib: <expressão>: <atrib>) bloco
# while (<expressão>) do bloco

testValidInput "int f() { for (x = 10 : x <= 10 : x = x + 1) { } }"
testValidInput "int f() { for (x = 10 : x <= 10 : x = x + 1) { id = 1; } }"
testInvalidInput "int f() { for (10 + 10 : x <= 10 : x = x + 1) { id = 1; } }"
testInvalidInput "int f() { for (x = 10 : x <= 10 : 10 + 10) { id = 1; } }"

testValidInput "int f() { while (1 + 1) do { } }"
testValidInput "int f() { while (1 + 1) do { id = 1; } }"

# Os dois marcadores atrib do comando for representa
# o comando de atribuição, unico aceito nestas posições. 
# Em todas as construções de controle de fluxo, o termo bloco
# indica um bloco de comandos. Este não tem ponto-e-vírgula nestas situações. 

#TODO :
# testValidInput "int f() { while (1 + 1) do { id = 1; }; }"
# testValidInput "int f() { while (1 + 1) do { 
#         id = 1; 
#     } }"

# Expr. Aritméticas, Lógicas

# As expressões podem ser de dois tipos: aritméticas e lógicas.
# As expressoes aritméticas podem ter como operandos: 

#    (a) identificadores, opcionalmente seguidos de expressao inteira entre colchetes, para acesso a vetores;
#    (b) literais numéricos como inteiro e ponto-flutuante;
#    (c) chamada de função.

expressionArgs=("1" "1.0" "id" "id[1]" "func()" "func(1)" "func(id)" "func(1,5)" "id[id1+id2]" "func(id1+id2)")

# As expressões aritméticas podem ser formadas recursivamente com operadores aritmeticos, assim como permitem o
# uso de parenteses para forçar uma associatividade ou precedencia diferente daquela tradicional.
# A associatividade é a esquerda.

# Expressões lógicas podem ser formadas através dos operadores relacionais aplicados a expressões aritméticas,
testValidInput "int f() { x = 1 + 1 == 2;}"

# ou de operadores lógicos aplicados a expressões lógicas, recursivamente.
testValidInput "int f() { x = 1 == 1 && 2 == 2;}"

# Outras expressões podem ser formadas considerando variaveis lógicas do tipo bool.
testValidInput "int f() { x = isOpen;}"
testValidInput "int f() { x = isOpen != isClosed;}"
testValidInput "int f() { x = isOpen == !isClosed;}"

# A descrição sintática deve aceitar qualquer operadores e subexpressao de um desses
# tipos como válidos, deixando para a análise semantica das proximas etapas do projeto
# a tarefa de verificar a validade dos operandos e operadores.

# Os operadores são os seguintes:

# • Unarios (todos prefixados)
#   – + sinal positivo explícito
#   – - inverte o sinal
#   – ! negação lógica
#   – & acesso ao endereço da variável
#   – * acesso ao valor do ponteiro
#   – ? avalia uma expressao para true ou false
#   – # acesso a um identificador como uma tabela hash

#TODO: testValidInput "int f() {1+1;}"

unaryOperators=("+" "-" "!" "?")
for unaryOperator in ${unaryOperators[@]}; do
    testValidInput "int f() { 1 + ${unaryOperator}id; }"
    testValidInput "int f() { 1 + ${unaryOperator}func(); }"
    testValidInput "int f() { 1 + ${unaryOperator}1; }"
    testValidInput "int f() { 1 + ${unaryOperator}1.0; }"
done

testValidInput "int f() { 1 + &id; }"
testValidInput "int f() { 1 + \*id; }"
testValidInput "int f() { 1 + #id; }"
testValidInput "int f() { 1 + \*&\*&id; }"

# TODO:
# testInvalidInput  "int f() { 1 + &1; }"
# testInvalidInput  "int f() { 1 + \*1; }"
# testInvalidInput  "int f() { 1 + #1; }"

# • Binários
#   – + soma
#   – - subtração
#   – * multiplicação
#   – / divisão
#   – % resto da divisão inteira
#   – | bitwise OR
#   – & bitwise AND
#   – ˆ exponenciação
#   – todos os comparadores relacionais
#   – todos os operadores logicos ( && para o e lógico, || para o ou lógico) 

argBinaryOperator=("+" "-" "\*" "/" "%" "|" "&" "^" "!=" "==" "<=" ">=" "&&" "||")
for expressionArg in ${expressionArgs[@]}; do
for binaryOperator in ${argBinaryOperator[@]}; do
    testValidInput "int f() { $expressionArg $binaryOperator $expressionArg; }"
    testValidInput "int f() { $expressionArg $binaryOperator $expressionArg $binaryOperator $expressionArg; }"
done
done

# • Ternários
#   – ? seguido de :, conforme a sintaxe expressão ? expressão : expressão

# As regras de associatividade e precedência de operadores matemáticos são
# aquelas tradicionais de linguagem de programação e da matemática.
# Recomenda-se que tais regras sejam já incorporadas na solução desta etapa,
# ou através de construções gramaticais ou através de comandos do bison 
# específicos para isso (%left, %right). A solução via construções gramaticais
# e recomendada. Enfim, nos casos não cobertos por esta regra geral, temos as
# seguintes regras de associatividade:

# • Associativos à direita
#   – &, * (acesso ao valor do ponteiro), #

echo "RESULTS:"
echo "Passed tests: $successfulTestsCounter"
echo "Failed tests: $failedTestsCounter"

make clean