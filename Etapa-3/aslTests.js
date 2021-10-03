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
      const letter = String.fromCharCode((totalKeys < 26 ? 65 : 71) + totalKeys); // Skip the 6 ascii characters between upper and lower-case.
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

    const { stdout } = await exec(`./etapa3 < ${file}`);
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
          `valgrind --leak-check=full --error-exitcode=2 --quiet ./etapa3 < ${file}`
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
  await exec("make");

  await testInput("int x;", "");
  await testInput(`int f1() { }`, `A [label="f1"];`);
  await testInput(`int f2() { }`, `A [label="f2"];`);

  await testInput(
    `
    int f1() { }
    int f2() { }`,
    `
    A, B
    A [label="f1"];
    B [label="f2"];
    `
  );

  await testInput(
    `
    int f1() { }
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
    int f1() { }
    int f2() { }
    int f3() { }
    `,
    `
    A, B
    B, C
    A [label="f1"];
    B [label="f2"];
    C [label="f3"];
    `
  );

  await testInput(
    `
    int f1() { }
    int x;
    `,
    `
    A [label="f1"];
    `
  );

  await testInput(
    `
    int x;
    int f1() { }
    `,
    `
    A [label="f1"];
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
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="="];
    C [label="a"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int b;
      b = 1;
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="="];
    C [label="b"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int a, b;
      a = 1;
      b = 1;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    A [label="f1"];
    B [label="="];
    C [label="a"];
    D [label="1"];
    E [label="="];
    F [label="b"];
    G [label="1"];
    `
  );

  const literalAsserts = [
    { declaration: 2, expected: 2 },
    { declaration: "1.0", expected: "1.000000" },
    { declaration: "'a'", expected: "a" },
    { declaration: '"XXX"', expected: "XXX" },
    { declaration: "true", expected: "true" },
    { declaration: "false", expected: "false" },
    { declaration: "id", expected: "id" },
  ];

  for (const literal of literalAsserts) {
    await testInput(
      `
      int f1() {
        a = ${literal.declaration};
      }
      `,
      `
      A, B
      B, C
      B, D
      A [label="f1"];
      B [label="="];
      C [label="a"];
      D [label="${literal.expected}"];
      `
    );
  }

  await testInput(
    `
    int f1() {
      x = v[3];
    }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    D, F
    A [label="f1"];
    B [label="="];
    C [label="x"];
    D [label="[]"];
    E [label="v"];
    F [label="3"];
    `
  );

  await testInput(
    `
    int f1() {
      g1();
    }
    `,
    `
    A, B
    A [label="f1"];
    B [label="call g1"];
    `
  );

  await testInput(
    `
    int f1() {
      x = g1();
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="="];
    C [label="x"];
    D [label="call g1"];
    `
  );

  await testInput(
    `
    int f1() {
      x = g1(1, 2, 3);
    }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    E, F
    F, G 
    A [label="f1"];
    B [label="="];
    C [label="x"];
    D [label="call g1"];
    E [label="1"];
    F [label="2"];
    G [label="3"];
    `
  );

  const valueAsserts = ["continue", "break"];

  for (const value of valueAsserts) {
    await testInput(
      `
      int f1() {
        ${value};
      }
      `,
      `
      A, B
      A [label="f1"];
      B [label="${value}"];
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
      input id;
    }
    `,
    `
    A, B
    B, C
    A [label="f1"];
    B [label="input"];
    C [label="id"];
    `
  );

  await testInput(
    `
    int f1() {
      id << 1;
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="<<"];
    C [label="id"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      id >> 1;
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label=">>"];
    C [label="id"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      id[1] << 1;
    }
    `,
    `
    A, B
    B, C
    C, D
    C, E
    B, F
    A [label="f1"];
    B [label="<<"];
    C [label="[]"];
    D [label="id"];
    E [label="1"];
    F [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      id[1] >> 1;
    }
    `,
    `
    A, B
    B, C
    C, D
    C, E
    B, F
    A [label="f1"];
    B [label=">>"];
    C [label="[]"];
    D [label="id"];
    E [label="1"];
    F [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1;
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int b, a <= 1;
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, b;
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int c, a <= 1, b;
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, b <= 2;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, c, b <= 2;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    `
  );

  await testInput(
    `
    int f1() {
      int a <= 1, b <= 2, c;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    `
  );

  await testInput(
    `
    int f1() {
      int c, a <= 1, b <= 2;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    A [label="f1"];
    B [label="<="];
    C [label="a"];
    D [label="1"];
    E [label="<="];
    F [label="b"];
    G [label="2"];
    `
  );

  await testInput(
    `
    int f1() {
      if(true){
        continue;
      };
    }
    `,
    `
    A, B
    B, C
    B, D
    A [label="f1"];
    B [label="if"];
    C [label="true"];
    D [label="continue"];
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
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    A [label="f1"];
    B [label="if"];
    C [label="true"];
    D [label="continue"];
    E [label="break"];
    `
  );

  await testInput(
    `
    int f(){
      int i;
      for(i = 0 : i : i = 2) {
         i = 3;
         i = 4;
      };
    }
    `,
    `
    A, B
    B, C
    C, D
    C, E
    B, F
    B, G
    G, H
    G, I
    B, J
    J, K
    J, L
    J, M
    M, N
    M, O
    A [label="f"];
    B [label="for"];
    C [label="="];
    D [label="i"];
    E [label="0"];
    F [label="i"];
    G [label="="];
    H [label="i"];
    I [label="2"];
    J [label="="];
    K [label="i"];
    L [label="3"];
    M [label="="];
    N [label="i"];
    O [label="4"];
    `
  );

  await testInput(
    `
    int f(){
      while (true) do {
         i = 3;
      };
    }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    D, F
    A [label="f"];
    B [label="while"];
    C [label="true"];
    D [label="="];
    E [label="i"];
    F [label="3"];
    `
  );

  //w65
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
      x = a - b * c;
      y = d / e + f;
      z = (g - h) / i;
      w = j * (k + l);
   }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    D, F
    F, G
    F, H
    B, I
    I, J
    I, K
    K, L
    L, M
    L, N
    K, O
    I, P
    P, Q
    P, R
    R, S
    S, T
    S, U
    R, V
    P, W
    W, X
    W, Y
    Y, Z
    Y, a
    a, b
    a, c
    A [label="f"];
    B [label="="];
    C [label="x"];
    D [label="-"];
    E [label="a"];
    F [label="*"];
    G [label="b"];
    H [label="c"];
    I [label="="];
    J [label="y"];
    K [label="+"];
    L [label="/"];
    M [label="d"];
    N [label="e"];
    O [label="f"];
    P [label="="];
    Q [label="z"];
    R [label="/"];
    S [label="-"];
    T [label="g"];
    U [label="h"];
    V [label="i"];
    W [label="="];
    X [label="w"];
    Y [label="*"];
    Z [label="j"];
    a [label="+"];
    b [label="k"];
    c [label="l"];
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
   }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    E, F
    F, G
    G, H
    G, I
    F, J
    E, K
    D, L
    B, M
    M, N
    M, O
    O, P
    P, Q
    Q, R
    R, S
    R, T
    Q, U
    P, V
    O, W
    M, X
    X, Y
    X, Z
    Z, a
    a, b
    b, c
    c, d
    c, e
    b, f
    a, g
    Z, h
    A [label="f"];
    B [label="="];
    C [label="x"];
    D [label="-"];
    E [label="-"];
    F [label="+"];
    G [label="+"];
    H [label="a"];
    I [label="b"];
    J [label="c"];
    K [label="d"];
    L [label="e"];
    M [label="="];
    N [label="y"];
    O [label="/"];
    P [label="/"];
    Q [label="*"];
    R [label="*"];
    S [label="f"];
    T [label="g"];
    U [label="h"];
    V [label="i"];
    W [label="j"];
    X [label="="];
    Y [label="z"];
    Z [label="+"];
    a [label="+"];
    b [label="*"];
    c [label="*"];
    d [label="k"];
    e [label="l"];
    f [label="m"];
    g [label="n"];
    h [label="o"];
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
   }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    D, F
    F, G
    F, H
    B, I
    I, J
    I, K
    K, L
    L, M
    L, N
    K, O
    I, P
    P, Q
    P, R
    R, S
    S, T
    S, U
    R, V
    P, W
    W, X
    W, Y
    Y, Z
    Y, a
    a, b
    a, c
    A [label="f"];
    B [label="="];
    C [label="x"];
    D [label="+"];
    E [label="a"];
    F [label="*"];
    G [label="b"];
    H [label="c"];
    I [label="="];
    J [label="y"];
    K [label="-"];
    L [label="/"];
    M [label="d"];
    N [label="e"];
    O [label="f"];
    P [label="="];
    Q [label="z"];
    R [label="*"];
    S [label="+"];
    T [label="g"];
    U [label="h"];
    V [label="i"];
    W [label="="];
    X [label="w"];
    Y [label="/"];
    Z [label="j"];
    a [label="-"];
    b [label="k"];
    c [label="l"];
    `
  );

  //w68
  await testInput(
    `
    int k(int x, int y, int z){}
    int f(){
            k(10+11,20+21,30+31);
    }
    `,
    `
    A, B
    B, C
    C, D
    D, E
    D, F
    D, G
    G, H
    G, I
    G, J
    J, K
    J, L
    A [label="k"];
    B [label="f"];
    C [label="call k"];
    D [label="+"];
    E [label="10"];
    F [label="11"];
    G [label="+"];
    H [label="20"];
    I [label="21"];
    J [label="+"];
    K [label="30"];
    L [label="31"];
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
    }
    `,
    `
    A [label="f"];
    `
  );

  //w70
  await testInput(
    `
    int f()
    {
      int b <= 10;
      int a <= b;
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    E, G
    A [label="f"];
    B [label="<="];
    C [label="b"];
    D [label="10"];
    E [label="<="];
    F [label="a"];
    G [label="b"];
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
    }
    `,
    `
    A, B
    B, C
    B, D
    B, E
    E, F
    F, G
    F, H
    H, I
    H, J
    E, K
    A [label="f"];
    B [label="="];
    C [label="c"];
    D [label="2"];
    E [label="="];
    F [label="[]"];
    G [label="a"];
    H [label="+"];
    I [label="c"];
    J [label="c"];
    K [label="2"];
    `
  );

  //w72
  await testInput(
    `
    int f()
    {
      int a;
      int b;
      int c;
      a = 2*c+a/b;
    }
    `,
    `
    A, B
    B, C
    B, D
    D, E
    E, F
    E, G
    D, H
    H, I
    H, J
    A [label="f"];
    B [label="="];
    C [label="a"];
    D [label="+"];
    E [label="*"];
    F [label="2"];
    G [label="c"];
    H [label="/"];
    I [label="a"];
    J [label="b"];
    `
  );

  //w73
  await testInput(
    `
    int f()
    {
      c[5] = a[2*c+k];
    }
    
    `,
    `
    A, B
    B, C
    C, D
    C, E
    B, F
    F, G
    F, H
    H, I
    I, J
    I, K
    H, L
    A [label="f"];
    B [label="="];
    C [label="[]"];
    D [label="c"];
    E [label="5"];
    F [label="[]"];
    G [label="a"];
    H [label="+"];
    I [label="*"];
    J [label="2"];
    K [label="c"];
    L [label="k"];
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
    }

    `,
    `
    A, B
    B, C
    C, D
    C, E
    B, F
    F, G
    G, H
    G, I
    I, J
    F, K
    A [label="f"];
    B [label="="];
    C [label="[]"];
    D [label="c"];
    E [label="5"];
    F [label="+"];
    G [label="*"];
    H [label="3"];
    I [label="call f"];
    J [label="a"];
    K [label="a"];
    `
  );

  //w75
  await testInput(
    `
    int f()
    {
      if (a == 2) { };
      if ((a+b) <= 10) { };
    }
    
    `,
    `
    A, B
    B, C
    C, D
    C, E
    B, F
    F, G
    G, H
    H, I
    H, J
    G, K
    A [label="f"];
    B [label="if"];
    C [label="=="];
    D [label="a"];
    E [label="2"];
    F [label="if"];
    G [label="<="];
    H [label="+"];
    I [label="a"];
    J [label="b"];
    K [label="10"];
    `
  );

  //w76
  await testInput(
    `
    int v1;
    int f1 (float k, int l) 
    {
      int a;
    }
    int v2[100];
    char v3;
    `,
    `
    A [label="f1"];
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

  await exec("make clean");
}

test();
