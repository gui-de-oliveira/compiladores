const fs = require("fs").promises;
const util = require("util");
const exec = util.promisify(require("child_process").exec);

function getSymbolsFromFile(file) {
  return file
    .split("\n")
    .filter((line) => line !== "")
    .map((line) => line.split(" "))
    .reduce((array, value) => array.concat(value), [])
    .filter((element) => !element.endsWith(";")) // Remove labels "0x555f51974580 ([label="f1"];)"
    .filter((element) => !element.includes("call"))
    .map((element) => element.replace(",", "")) //  Remove comma separator from element "0x555f51974b00[,] 0x555f51974dc0"
    .map((symbol) => symbol.trim()); // remove whitespaces
}

function listOfSymbolsToDictionaryOfSymbols(listOfSymbols) {
  return listOfSymbols.reduce((previousValue, currentSymbol) => {
    if (previousValue[currentSymbol] === undefined) {
      const totalKeys = Object.keys(previousValue).length;
      const letter = String.fromCharCode(
        (totalKeys < 26 ? 65 : 71) + totalKeys
      ); // Skip the 6 ascii characters between upper and lower-case.
      previousValue[currentSymbol] = letter;
    }

    return previousValue;
  }, {});
}

function replaceFileMemoryWithSymbols(file) {
  const symbols = getSymbolsFromFile(file);
  const listOfSymbols = listOfSymbolsToDictionaryOfSymbols(symbols);

  const newFile = Object.keys(listOfSymbols).reduce(
    (previous, current) => previous.split(current).join(listOfSymbols[current]), // replaceAll
    file
  );

  return newFile;
}

const FontColor = {
  Reset: "\x1b[0m",
  Bright: "\x1b[1m",
  Dim: "\x1b[2m",
  Underscore: "\x1b[4m",
  Blink: "\x1b[5m",
  Reverse: "\x1b[7m",
  Hidden: "\x1b[8m",
  Fg: {
    Black: "\x1b[30m",
    Red: "\x1b[31m",
    Green: "\x1b[32m",
    Yellow: "\x1b[33m",
    Blue: "\x1b[34m",
    Magenta: "\x1b[35m",
    Cyan: "\x1b[36m",
    White: "\x1b[37m",
  },
  Bg: {
    Black: "\x1b[40m",
    Red: "\x1b[41m",
    Green: "\x1b[42m",
    Yellow: "\x1b[43m",
    Blue: "\x1b[44m",
    Magenta: "\x1b[45m",
    Cyan: "\x1b[46m",
    White: "\x1b[47m",
  },
};

var testsCounter = 0;
var memoryLeaks = 0;
var testMemoryLeak = false;

async function testInput(input, expected) {
  try {
    testsCounter++;

    const file = `.temp`;
    await fs.writeFile(file, input);

    const { stdout } = await exec(`./etapa4 < ${file}`);
    const rawOutput = stdout;
    const formattedOutput = replaceFileMemoryWithSymbols(rawOutput).trim();

    const formattedExpected = expected
      .split("\n")
      .filter((line) => line !== "")
      .map((line) => line.trim())
      .join("\n")
      .trim();

    if (formattedOutput !== formattedExpected) {
      console.log(FontColor.Fg.Red);
      console.log(`Test ${testsCounter} failed!`);
      console.log(`Input: \n${input}\n`);
      console.log(`Expected: \n${formattedExpected}\n`);
      console.log(`Received: \n${formattedOutput}\n`);
      console.log(`(no replacing): \n${rawOutput}\n`);
      console.log(FontColor.Reset);

      process.exit();
    }

    process.stdout.write(FontColor.Fg.Green);
    process.stdout.write(`Test ${testsCounter} passed!`);
    process.stdout.write(FontColor.Reset);

    if (testMemoryLeak) {
      try {
        await exec(
          `valgrind --leak-check=full --error-exitcode=2 --quiet ./etapa4 < ${file}`
        );
      } catch {
        process.stdout.write(FontColor.Fg.Red);
        process.stdout.write(` (Memory leaked!)`);
        process.stdout.write(FontColor.Reset);
        process.exit();
      }
    }

    console.log("");

    await exec(`rm ${file}`);
  } catch (error) {
    console.log(FontColor.Fg.Red);
    console.log(`Test ${testsCounter} CRASHED!`);
    console.log(`Input: "${input}"`);
    console.log("Error: ", error);
    console.log(FontColor.Reset);

    process.exit();
  }
}

async function test() {
  await testInput("int x;", "");
  await testInput(`int f1() { }`, `A [label="f1"];`);
  await testInput(`int f2() { }`, `A [label="f2"];`);

  await testInput(
    `
    int f1() { return 0; }
    int f2() { return 0; }`,
    `
    A, B
    A, C
    B, D
    C, E
    E, F
    A [label="f1"];
    B [label="return"];
    D [label="0"];
    C [label="f2"];
    E [label="return"];
    F [label="0"];
    `
  );

  await testInput(
    `
    int f1() { return 0; }
    int f2() { return 0; }
    int f3() { return 0; }
    `,
    `
    A, B
    A, C
    B, D
    C, E
    C, F
    E, G
    F, H
    H, I
    A [label="f1"];
    B [label="return"];
    D [label="0"];
    C [label="f2"];
    E [label="return"];
    G [label="0"];
    F [label="f3"];
    H [label="return"];
    I [label="0"];
    `
  );

  await testInput(
    `
    int f1() { return 0; }
    int x;
    `,
    `
    A, B
    B, C
    A [label="f1"];
    B [label="return"];
    C [label="0"];
    `
  );

  await testInput(
    `
    int x;
    int f1() { return 0; }
    `,
    `
    A, B
    B, C
    A [label="f1"];
    B [label="return"];
    C [label="0"];
    `
  );

  await testInput(
    `
    int f1() { }
    int x;
    int f2() { }
    `,
    `
    A, B
    A [label="f1"];
    B [label="f2"];
    `
  );

  await testInput(
    `
    int f1() {
      int a;
      a = 1;
      return 0;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    A [label="f1"];
    B [label="="];
    C [label="a"];
    D [label="1"];
    E [label="return"];
    F [label="0"];
    `
  );

  await testInput(
    `
    int f1() {
      int b;
      b = 1;
      return 0;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    A [label="f1"];
    B [label="="];
    C [label="b"];
    D [label="1"];
    E [label="return"];
    F [label="0"];
    `
  );

  await testInput(
    `
    int f1() {
      int a, b;
      a = 1;
      b = 1;
      return 0;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f1"];
    B [label="="];
    C [label="a"];
    D [label="1"];
    E [label="="];
    F [label="b"];
    G [label="1"];
    H [label="return"];
    I [label="0"];
    `
  );

  const literalAsserts = [
    { varType: "int", declaration: 2, expected: 2 },
    { varType: "float", declaration: "1.0", expected: "1.0" },
    { varType: "char", declaration: "'a'", expected: "a" },
    { varType: "bool", declaration: "true", expected: "true" },
    { varType: "bool", declaration: "false", expected: "false" },
    { varType: "int", declaration: "id", expected: "id" },
  ];

  for (const literal of literalAsserts) {
    await testInput(
      `
      int f1() {
        ${literal.varType} b;
        ${literal.varType} a;
        a = ${literal.declaration};
        return 0;
      }
      `,
      `
      A, B
      B, C
      B, D
      B, E
      E, F
      A [label="f1"];
      B [label="="];
      C [label="a"];
      D [label="${literal.expected}"];
      E [label="return"];
      F [label="0"];
      `
    );
  }

  await testInput(
    `
    int v[5];
    int f1() {
      int x;
      x = v[3];
      return 0;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    D, F
    D, G
    E, H
    A [label="f1"];
    B [label="="];
    C [label="x"];
    D [label="[]"];
    F [label="v"];
    G [label="3"];
    E [label="return"];
    H [label="0"];
    `
  );

  await testInput(
    `
    int g1() { return 0; }
    int f1() {
      return g1();
    }
    `,
    `
    A, B
    A, C
    B, D
    C, E
    E, F
    A [label="g1"];
    B [label="return"];
    D [label="0"];
    C [label="f1"];
    E [label="return"];
    F [label="call g1"];
    `
  );

  await testInput(
    `
    int x;
    int g1() { return 0; }
    int f1() {
      x = g1();
      return x;
    }
    `,
    `
    A, B
    A, C
    B, D
    C, E
    E, F
    E, G
    E, H
    H, I
    A [label="g1"];
    B [label="return"];
    D [label="0"];
    C [label="f1"];
    E [label="="];
    F [label="x"];
    G [label="call g1"];
    H [label="return"];
    I [label="x"];
    `
  );

  await testInput(
    `
    int g1(int a, int b, int c) { return a + b + c; }
    int f1() {
      x = g1(1, 2, 3);
      return x;
    }
    `,
    `
    A, B
    A, C
    B, D
    D, E
    D, F
    E, G
    E, H
    C, I
    I, J
    I, K
    I, L
    K, M
    M, N
    N, O
    L, P
    A [label="g1"];
    B [label="return"];
    D [label="+"];
    E [label="+"];
    G [label="a"];
    H [label="b"];
    F [label="c"];
    C [label="f1"];
    I [label="="];
    J [label="x"];
    K [label="call g1"];
    M [label="1"];
    N [label="2"];
    O [label="3"];
    L [label="return"];
    P [label="x"];
    `
  );

  const valueAsserts = ["continue", "break"];

  for (const value of valueAsserts) {
    await testInput(
      `
      int f1() {
        ${value};
        return 0;
      }
      `,
      `
      A, B
      B, C
      C, D
      A [label="f1"];
      B [label="${value}"];
      C [label="return"];
      D [label="0"];
      `
    );
  }

  await testInput(
    `
    int f1() {
      return 1;
    }
    `,
    `
    A, B
    B, C
    A [label="f1"];
    B [label="return"];
    C [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int id;
      input id;
      return id;
    }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    A [label="f1"];
    B [label="input"];
    C [label="id"];
    D [label="return"];
    E [label="id"];
    `
  );

  await testInput(
    `
    int f1() {
      int id <= 2;
      id << 1;
      return id;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f1"];
    B [label="<="];
    C [label="id"];
    D [label="2"];
    E [label="<<"];
    F [label="id"];
    G [label="1"];
    H [label="return"];
    I [label="id"];
    `
  );

  await testInput(
    `
    int f1() {
      int id <= 2;
      id >> 1;
      return id;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f1"];
    B [label="<="];
    C [label="id"];
    D [label="2"];
    E [label=">>"];
    F [label="id"];
    G [label="1"];
    H [label="return"];
    I [label="id"];
    `
  );

  await testInput(
    `
    int id[2];
    int f1() {
      id[1] << 1;
      return id[1];
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    C, F
    C, G
    E, H
    H, I
    H, J
    A [label="f1"];
    B [label="<<"];
    C [label="[]"];
    F [label="id"];
    G [label="1"];
    D [label="1"];
    E [label="return"];
    H [label="[]"];
    I [label="id"];
    J [label="1"];
    `
  );

  await testInput(
    `
    int id[2];
    int f1() {
      id[1] >> 1;
      return id[1];
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    C, F
    C, G
    E, H
    H, I
    H, J
    A [label="f1"];
    B [label=">>"];
    C [label="[]"];
    F [label="id"];
    G [label="1"];
    D [label="1"];
    E [label="return"];
    H [label="[]"];
    I [label="id"];
    J [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1;
      return a;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="return"];
    F [label="a"];
    `
  );

  await testInput(
    `
    int f1() {
      int b, a <= 1;
      return a;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="return"];
    F [label="a"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, b;
      return a;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="return"];
    F [label="a"];
    `
  );

  await testInput(
    `
    int f1() {
      int c, a <= 1, b;
      return a;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="return"];
    F [label="a"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, b <= 2;
      return a;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    H [label="return"];
    I [label="a"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, c, b <= 2;
      return b;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    H [label="return"];
    I [label="b"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, b <= 2, c;
      return a;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    H [label="return"];
    I [label="a"];
    `
  );

  await testInput(
    `
    int f1() {
      int c, a <= 1, b <= 2;
      return b;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    H [label="return"];
    I [label="b"];
    `
  );

  await testInput(
    `
    int f1() {
      while(true) do {
        if(true) {
          continue;
        };
      };
      return 0;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    D, F
    D, G
    E, H
    A [label="f1"];
    B [label="while"];
    C [label="true"];
    D [label="if"];
    F [label="true"];
    G [label="continue"];
    E [label="return"];
    H [label="0"];
    `
  );

  await testInput(
    `
    int f1() {
      if(true){
        continue;
      } else {
        break;
      };
      return 0;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    B, F
    F, G
    A [label="f1"];
    B [label="if"];
    C [label="true"];
    D [label="continue"];
    E [label="break"];
    F [label="return"];
    G [label="0"];
    `
  );

  await testInput(
    `
    int f(){
      int i;
      int j <= 0;
      for(i = 0 : i < 5 : i = i + 1) {
         j = 3;
      };
      return j;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    E, I
    E, J
    F, K
    F, L
    G, M
    G, N
    H, O
    H, P
    P, Q
    P, R
    I, S
    I, T
    J, U
    A [label="f"];
    B [label="<="];
    C [label="j"];
    D [label="0"];
    E [label="for"];
    F [label="="];
    K [label="i"];
    L [label="0"];
    G [label="<"];
    M [label="i"];
    N [label="5"];
    H [label="="];
    O [label="i"];
    P [label="+"];
    Q [label="i"];
    R [label="1"];
    I [label="="];
    S [label="j"];
    T [label="3"];
    J [label="return"];
    U [label="j"];
    `
  );

  await testInput(
    `
    int f(){
      int i <= 2;
      while (i != 3) do {
         i = 3;
      };
      return i;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    F, I
    F, J
    G, K
    G, L
    H, M
    A [label="f"];
    B [label="<="];
    C [label="i"];
    D [label="2"];
    E [label="while"];
    F [label="!="];
    I [label="i"];
    J [label="3"];
    G [label="="];
    K [label="i"];
    L [label="3"];
    H [label="return"];
    M [label="i"];
    `
  );

  //w65
  await testInput(
    `
    int f(){
      int a <= 1;
      int x <= 1;
      int b <= 1;
      int c <= 1;
      int y <= 1;
      int d <= 1;
      int e;
      int f;
      int z;
      int g;
      int h;
      int i;
      int w;
      int j;
      int k;
      int l;
      x = a - b * c;
      y = d / e + f;
      z = (g - h) / i;
      w = j * (k + l);
      return 0;
   }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    H, J
    H, K
    K, L
    K, M
    K, N
    N, O
    N, P
    N, Q
    Q, R
    Q, S
    Q, T
    T, U
    T, V
    T, W
    V, X
    V, Y
    Y, Z
    Y, a
    W, b
    W, c
    W, d
    c, e
    c, f
    e, g
    e, h
    d, i
    d, j
    d, k
    j, l
    j, m
    l, n
    l, o
    k, p
    k, q
    k, r
    q, s
    q, t
    t, u
    t, v
    r, w
    A [label="f"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="x"];
    G [label="1"];
    H [label="<="];
    I [label="b"];
    J [label="1"];
    K [label="<="];
    L [label="c"];
    M [label="1"];
    N [label="<="];
    O [label="y"];
    P [label="1"];
    Q [label="<="];
    R [label="d"];
    S [label="1"];
    T [label="="];
    U [label="x"];
    V [label="-"];
    X [label="a"];
    Y [label="*"];
    Z [label="b"];
    a [label="c"];
    W [label="="];
    b [label="y"];
    c [label="+"];
    e [label="/"];
    g [label="d"];
    h [label="e"];
    f [label="f"];
    d [label="="];
    i [label="z"];
    j [label="/"];
    l [label="-"];
    n [label="g"];
    o [label="h"];
    m [label="i"];
    k [label="="];
    p [label="w"];
    q [label="*"];
    s [label="j"];
    t [label="+"];
    u [label="k"];
    v [label="l"];
    r [label="return"];
    w [label="0"];
    `
  );

  //w66
  await testInput(
    `
    int f(){
      int x;
      int a;
      int b;
      int c;
      int d;
      int e;
      int y;
      int f;
      int g;
      int h;
      int i;
      int j;
      int z;
      int k;
      int l;
      int m;
      int n;
      int o;
      x = a + b + c - d - e;
      y = f * g * h / i / j;
      z = k * l * m + n + o;
      return 0;
   }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    D, F
    D, G
    F, H
    F, I
    H, J
    H, K
    J, L
    J, M
    E, N
    E, O
    E, P
    O, Q
    O, R
    Q, S
    Q, T
    S, U
    S, V
    U, W
    U, X
    P, Y
    P, Z
    P, a
    Z, b
    Z, c
    b, d
    b, e
    d, f
    d, g
    f, h
    f, i
    a, j
    A [label="f"];
    B [label="="];
    C [label="x"];
    D [label="-"];
    F [label="-"];
    H [label="+"];
    J [label="+"];
    L [label="a"];
    M [label="b"];
    K [label="c"];
    I [label="d"];
    G [label="e"];
    E [label="="];
    N [label="y"];
    O [label="/"];
    Q [label="/"];
    S [label="*"];
    U [label="*"];
    W [label="f"];
    X [label="g"];
    V [label="h"];
    T [label="i"];
    R [label="j"];
    P [label="="];
    Y [label="z"];
    Z [label="+"];
    b [label="+"];
    d [label="*"];
    f [label="*"];
    h [label="k"];
    i [label="l"];
    g [label="m"];
    e [label="n"];
    c [label="o"];
    a [label="return"];
    j [label="0"];
    `
  );

  //w67
  await testInput(
    `
    int f(){
      int a;
      int x;
      int b;
      int c;
      int y;
      int d;
      int e;
      int f;
      int z;
      int g;
      int h;
      int i;
      int w;
      int j;
      int k;
      int l;
      x = a + b * c;
      y = d / e - f;
      z = (g + h) * i;
      w = j / (k - l);
      return 0;
   }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    D, F
    D, G
    G, H
    G, I
    E, J
    E, K
    E, L
    K, M
    K, N
    M, O
    M, P
    L, Q
    L, R
    L, S
    R, T
    R, U
    T, V
    T, W
    S, X
    S, Y
    S, Z
    Y, a
    Y, b
    b, c
    b, d
    Z, e
    A [label="f"];
    B [label="="];
    C [label="x"];
    D [label="+"];
    F [label="a"];
    G [label="*"];
    H [label="b"];
    I [label="c"];
    E [label="="];
    J [label="y"];
    K [label="-"];
    M [label="/"];
    O [label="d"];
    P [label="e"];
    N [label="f"];
    L [label="="];
    Q [label="z"];
    R [label="*"];
    T [label="+"];
    V [label="g"];
    W [label="h"];
    U [label="i"];
    S [label="="];
    X [label="w"];
    Y [label="/"];
    a [label="j"];
    b [label="-"];
    c [label="k"];
    d [label="l"];
    Z [label="return"];
    e [label="0"];
    `
  );

  //w68
  await testInput(
    `
    int k(int x, int y, int z){ return x + y + z; }
    int f(){
            return k(10+11,20+21,30+31);
    }
    `,
    `
    A, B
    A, C
    B, D
    D, E
    D, F
    E, G
    E, H
    C, I
    I, J
    J, K
    K, L
    K, M
    K, N
    N, O
    N, P
    N, Q
    Q, R
    Q, S
    A [label="k"];
    B [label="return"];
    D [label="+"];
    E [label="+"];
    G [label="x"];
    H [label="y"];
    F [label="z"];
    C [label="f"];
    I [label="return"];
    J [label="call k"];
    K [label="+"];
    L [label="10"];
    M [label="11"];
    N [label="+"];
    O [label="20"];
    P [label="21"];
    Q [label="+"];
    R [label="30"];
    S [label="31"];
    `
  );

  //w69
  await testInput(
    `
    int f ()
    {
      {
        {
          {};
        };
      };
      return 0;
    }
    `,
    `
    A, B
    B, C
    A [label="f"];
    B [label="return"];
    C [label="0"];
    `
  );

  //w70
  await testInput(
    `
    int f()
    {
      int b <= 10;
      int a <= b;
      return b;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    A [label="f"];
    B [label="<="];
    C [label="b"];
    D [label="10"];
    E [label="<="];
    F [label="a"];
    G [label="b"];
    H [label="return"];
    I [label="b"];
    `
  );

  //w71
  await testInput(
    `
    int a[10];
    int f()
    {
      int c;
      c = 2;
      a[c+c] = 2;
      return a[c+c];
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    F, I
    F, J
    J, K
    J, L
    H, M
    M, N
    M, O
    O, P
    O, Q
    A [label="f"];
    B [label="="];
    C [label="c"];
    D [label="2"];
    E [label="="];
    F [label="[]"];
    I [label="a"];
    J [label="+"];
    K [label="c"];
    L [label="c"];
    G [label="2"];
    H [label="return"];
    M [label="[]"];
    N [label="a"];
    O [label="+"];
    P [label="c"];
    Q [label="c"];
    `
  );

  //w72
  await testInput(
    `
    int f()
    {
      int a <= 1;
      int b <= 2;
      int c <= 3;
      a = 2*c+a/b;
      return a;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    E, H
    H, I
    H, J
    H, K
    K, L
    K, M
    K, N
    M, O
    M, P
    O, Q
    O, R
    P, S
    P, T
    N, U
    A [label="f"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    H [label="<="];
    I [label="c"];
    J [label="3"];
    K [label="="];
    L [label="a"];
    M [label="+"];
    O [label="*"];
    Q [label="2"];
    R [label="c"];
    P [label="/"];
    S [label="a"];
    T [label="b"];
    N [label="return"];
    U [label="a"];
    `
  );

  //w73
  await testInput(
    `
    int c[6];
    int a[7];
    int k;
    int f()
    {
      c[5] = a[2*c+k];
      return c[5];
    }
    
    `,
    `
    A, B
    B, C
    B, D
    B, E
    C, F
    C, G
    D, H
    D, I
    I, J
    I, K
    J, L
    J, M
    E, N
    N, O
    N, P
    A [label="f"];
    B [label="="];
    C [label="[]"];
    F [label="c"];
    G [label="5"];
    D [label="[]"];
    H [label="a"];
    I [label="+"];
    J [label="*"];
    L [label="2"];
    M [label="c"];
    K [label="k"];
    E [label="return"];
    N [label="[]"];
    O [label="c"];
    P [label="5"];
    `
  );

  //w74
  await testInput(
    `
    int c[10];
    int f(int a)
    {
      int a;
      c[5] = 3 * f(a) + a;
      return c[5];
    }

    `,
    `
    A, B
    B, C
    B, D
    B, E
    C, F
    C, G
    D, H
    D, I
    H, J
    H, K
    K, L
    E, M
    M, N
    M, O
    A [label="f"];
    B [label="="];
    C [label="[]"];
    F [label="c"];
    G [label="5"];
    D [label="+"];
    H [label="*"];
    J [label="3"];
    K [label="call f"];
    L [label="a"];
    I [label="a"];
    E [label="return"];
    M [label="[]"];
    N [label="c"];
    O [label="5"];
    `
  );

  //w75
  await testInput(
    `
    int a;
    int b;
    int f()
    {
      if (a == 2) { return 0; };
      if ((a+b) <= 10) { return 1; };
      return 2;
    }
    
    `,
    `
    A, B
    B, C
    B, D
    B, E
    C, F
    C, G
    D, H
    E, I
    E, J
    E, K
    I, L
    I, M
    L, N
    L, O
    J, P
    K, Q
    A [label="f"];
    B [label="if"];
    C [label="=="];
    F [label="a"];
    G [label="2"];
    D [label="return"];
    H [label="0"];
    E [label="if"];
    I [label="<="];
    L [label="+"];
    N [label="a"];
    O [label="b"];
    M [label="10"];
    J [label="return"];
    P [label="1"];
    K [label="return"];
    Q [label="2"];
    `
  );

  //w76
  await testInput(
    `
    int v1;
    int f1 (float k, int l) 
    {
      int a;
      return l;
    }
    int v2[100];
    char v3;
    `,
    `
    A, B
    B, C
    A [label="f1"];
    B [label="return"];
    C [label="l"];
    `
  );

  process.stdout.write(FontColor.Fg.Green);
  console.log("ALL TESTS PASSED!");
  process.stdout.write(FontColor.Reset);

  if (!testMemoryLeak) {
    console.log("Not tested for memory leaks.");
  } else {
    if (memoryLeaks === 0) {
      process.stdout.write(FontColor.Fg.Green);
      console.log("NO MEMORY LEAKS!");
      process.stdout.write(FontColor.Reset);
    } else {
      process.stdout.write(FontColor.Fg.Red);
      console.log(`${memoryLeaks} MEMORY LEAKS!`);
      process.stdout.write(FontColor.Reset);
    }
  }
}

test();
