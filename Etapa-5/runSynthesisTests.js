const fs = require("fs").promises;
const util = require("util");
const exec = util.promisify(require("child_process").exec);

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

async function testInput(input, expected) {
  try {
    testsCounter++;

    const file = `.temp`;
    await fs.writeFile(file, input);

    const { stdout } = await exec(`./etapa5 < ${file}`);
    const rawOutput = stdout;

    const formattedOutput = rawOutput
      .split("\n")
      .filter((line) => line !== "")
      .map((line) => line.trim())
      .join("\n")
      .trim();

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
      console.log(FontColor.Reset);

      process.exit();
    }

    process.stdout.write(FontColor.Fg.Green);
    process.stdout.write(`Test ${testsCounter} passed!\n`);
    process.stdout.write(FontColor.Reset);

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
  await testInput(
    "int main() { }",
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 18 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  loadAI rfp, 0 => r1
  loadAI rfp, 4 => r2
  loadAI rfp, 8 => r3
  i2i r2 => rsp
  i2i r3 => rfp
  jump -> r1
`
  );

  await testInput(
    "int main() { int a; }",
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 18 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  loadAI rfp, 0 => r1
  loadAI rfp, 4 => r2
  loadAI rfp, 8 => r3
  i2i r2 => rsp
  i2i r3 => rfp
  jump -> r1
  `
  );

  await testInput(
    `
  int global;
  int main() { }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 18 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  loadAI rfp, 0 => r1
  loadAI rfp, 4 => r2
  loadAI rfp, 8 => r3
  i2i r2 => rsp
  i2i r3 => rfp
  jump -> r1
  `
  );

  await testInput(
    `
  int funcA() { }
  int main() { }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 27 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L1
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  loadAI rfp, 0 => r1
  loadAI rfp, 4 => r2
  loadAI rfp, 8 => r3
  i2i r2 => rsp
  i2i r3 => rfp
  jump -> r1
  L1: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  loadAI rfp, 0 => r4
  loadAI rfp, 4 => r5
  loadAI rfp, 8 => r6
  i2i r5 => rsp
  i2i r6 => rfp
  jump -> r4
`
  );

  await testInput(
    `
  int main() { }
  int funcA() { }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 27 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  loadAI rfp, 0 => r1
  loadAI rfp, 4 => r2
  loadAI rfp, 8 => r3
  i2i r2 => rsp
  i2i r3 => rfp
  jump -> r1
  L1: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  loadAI rfp, 0 => r4
  loadAI rfp, 4 => r5
  loadAI rfp, 8 => r6
  i2i r5 => rsp
  i2i r6 => rfp
  jump -> r4
`
  );

  await testInput(
    `
    int main() {
      int a;
      a = 5;
    }
    `,
    `
    loadI 1024 => rfp
    loadI 1024 => rsp
    loadI 21 => rbss
    loadI 8 => r0
    storeAI r0 => rsp, 0
    storeAI rsp => rsp, 4
    storeAI rfp => rsp, 8
    jumpI -> L0
    halt
    L0: nop
    i2i rsp => rfp
    addI rsp, 16 => rsp
    addI 4, rsp => rsp
    loadI 5 => r1
    storeAI r1 => rfp, 16
    loadAI rfp, 0 => r1
    loadAI rfp, 4 => r2
    loadAI rfp, 8 => r3
    i2i r2 => rsp
    i2i r3 => rfp
    jump -> r1
  `
  );

  process.stdout.write(FontColor.Fg.Green);
  console.log("ALL TESTS PASSED!");
  process.stdout.write(FontColor.Reset);
}

test();
