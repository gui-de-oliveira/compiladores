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
    { declaration: "1.0", expected: "1.0" },
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
    B, D
    C, E
    C, F
    A [label="f1"];
    B [label="<<"];
    C [label="[]"];
    E [label="id"];
    F [label="1"];
    D [label="1"];
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
    B, D
    C, E
    C, F
    A [label="f1"];
    B [label=">>"];
    C [label="[]"];
    E [label="id"];
    F [label="1"];
    D [label="1"];
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
    B, D
    B, E
    B, F
    C, G
    C, H
    E, I
    E, J
    F, K
    F, L
    F, M
    M, N
    M, O
    A [label="f"];
    B [label="for"];
    C [label="="];
    G [label="i"];
    H [label="0"];
    D [label="i"];
    E [label="="];
    I [label="i"];
    J [label="2"];
    F [label="="];
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
    Y, Z
    Y, a
    a, b
    a, c
    A [label="f"];
    B [label="="];
    C [label="x"];
    D [label="-"];
    F [label="a"];
    G [label="*"];
    H [label="b"];
    I [label="c"];
    E [label="="];
    J [label="y"];
    K [label="+"];
    M [label="/"];
    O [label="d"];
    P [label="e"];
    N [label="f"];
    L [label="="];
    Q [label="z"];
    R [label="/"];
    T [label="-"];
    V [label="g"];
    W [label="h"];
    U [label="i"];
    S [label="="];
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
    Z, a
    Z, b
    a, c
    a, d
    c, e
    c, f
    e, g
    e, h
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
    a [label="+"];
    c [label="*"];
    e [label="*"];
    g [label="k"];
    h [label="l"];
    f [label="m"];
    d [label="n"];
    b [label="o"];
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
    Y, Z
    Y, a
    a, b
    a, c
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
    E, G
    F, H
    F, I
    I, J
    I, K
    A [label="f"];
    B [label="="];
    C [label="c"];
    D [label="2"];
    E [label="="];
    F [label="[]"];
    H [label="a"];
    I [label="+"];
    J [label="c"];
    K [label="c"];
    G [label="2"];
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
    D, F
    E, G
    E, H
    F, I
    F, J
    A [label="f"];
    B [label="="];
    C [label="a"];
    D [label="+"];
    E [label="*"];
    G [label="2"];
    H [label="c"];
    F [label="/"];
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
    B, D
    C, E
    C, F
    D, G
    D, H
    H, I
    H, J
    I, K
    I, L
    A [label="f"];
    B [label="="];
    C [label="[]"];
    E [label="c"];
    F [label="5"];
    D [label="[]"];
    G [label="a"];
    H [label="+"];
    I [label="*"];
    K [label="2"];
    L [label="c"];
    J [label="k"];
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
    B, D
    C, E
    C, F
    D, G
    D, H
    G, I
    G, J
    J, K
    A [label="f"];
    B [label="="];
    C [label="[]"];
    E [label="c"];
    F [label="5"];
    D [label="+"];
    G [label="*"];
    I [label="3"];
    J [label="call f"];
    K [label="a"];
    H [label="a"];
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
    B, D
    C, E
    C, F
    D, G
    G, H
    G, I
    H, J
    H, K
    A [label="f"];
    B [label="if"];
    C [label="=="];
    E [label="a"];
    F [label="2"];
    D [label="if"];
    G [label="<="];
    H [label="+"];
    J [label="a"];
    K [label="b"];
    I [label="10"];
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
}

test();
