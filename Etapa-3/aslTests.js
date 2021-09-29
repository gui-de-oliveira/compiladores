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
    .map((element) => element.replace(",", "")) //  Remove comma separator from element "0x555f51974b00[,] 0x555f51974dc0"
    .map((symbol) => symbol.trim()); // remove whitespaces
}

function listOfSymbolsToDictionaryOfSymbols(listOfSymbols) {
  return listOfSymbols.reduce((previousValue, currentSymbol) => {
    if (previousValue[currentSymbol] === undefined) {
      const totalKeys = Object.keys(previousValue).length;
      const letter = String.fromCharCode(65 + totalKeys);
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
      console.log(`Input: "${input}"`);
      console.log(`Expected: "${formattedExpected}"`);
      console.log(`Received: "${formattedOutput}"`);
      console.log(`(no replacing): "${rawOutput}"`);
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
      }
      process.exit();
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
  ];

  for (const literal of literalAsserts) {
    await testInput(
      `
      int f1() {
        int a;
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
