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
      console.log(`Input: \n${input}\n` + FontColor.Reset);

      const expectedLines = formattedExpected.split("\n");
      const receivedLines = formattedOutput.split("\n");

      console.log(`Output:`);
      for (
        let expectedI = 0, receivedI = 0;
        expectedI < expectedLines.length || receivedI < receivedLines.length;

      ) {
        if (expectedI >= expectedLines.length) {
          console.log(
            FontColor.Fg.Red +
              `RECEIVED: ${receivedLines[receivedI]}` +
              FontColor.Reset
          );
          receivedI++;
          continue;
        }

        const receivedLine = receivedLines[receivedI];
        const expectedLine = expectedLines[expectedI];

        if (expectedLine === receivedLine) {
          console.log(receivedLine);
          expectedI += 1;
          receivedI += 1;
          continue;
        }

        const expectedLineOnReceivedLine = receivedLines.findIndex(
          (line, i) => i > receivedI && line === expectedLine
        );

        const isExpectedLineOnReceivedArray = expectedLineOnReceivedLine !== -1;
        if (!isExpectedLineOnReceivedArray) {
          console.log(
            FontColor.Fg.Green +
              `EXPECTED: ${expectedLines[expectedI]}` +
              FontColor.Reset
          );
          expectedI += 1;
          continue;
        }

        for (; receivedI < expectedLineOnReceivedLine; receivedI++) {
          console.log(
            FontColor.Fg.Red +
              `RECEIVED: ${receivedLines[receivedI]}` +
              FontColor.Reset
          );
        }
      }

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
  loadI 19 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  addI rsp, 4 => rsp
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
  addI rsp, 4 => rsp
  loadI 5 => r1
  storeAI r1 => rfp, 16
  loadAI rfp, 0 => r2
  loadAI rfp, 4 => r3
  loadAI rfp, 8 => r4
  i2i r3 => rsp
  i2i r4 => rfp
  jump -> r2
  `
  );

  await testInput(
    `
  int main() {
    int a <= 5;
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
  addI rsp, 4 => rsp
  loadI 5 => r1
  storeAI r1 => rfp, 16
  loadAI rfp, 0 => r2
  loadAI rfp, 4 => r3
  loadAI rfp, 8 => r4
  i2i r3 => rsp
  i2i r4 => rfp
  jump -> r2
  `
  );

  await testInput(
    `
  int main() {
    int a;
    int b <= 5;
    int c <= 10;
  }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 25 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  loadI 5 => r1
  storeAI r1 => rfp, 20
  addI rsp, 4 => rsp
  loadI 10 => r2
  storeAI r2 => rfp, 24
  loadAI rfp, 0 => r3
  loadAI rfp, 4 => r4
  loadAI rfp, 8 => r5
  i2i r4 => rsp
  i2i r5 => rfp
  jump -> r3
  `
  );

  await testInput(
    `
  int main() {
    int a;
    int b <= 5;
    int c <= 10;
    a = b + c;
  }
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
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  loadI 5 => r1
  storeAI r1 => rfp, 20
  addI rsp, 4 => rsp
  loadI 10 => r2
  storeAI r2 => rfp, 24
  loadI 15 => r3
  storeAI r3 => rfp, 16
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
    int b <= 30;
    int c <= 30;
    a = b + c;
  }
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
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  loadI 30 => r1
  storeAI r1 => rfp, 20
  addI rsp, 4 => rsp
  loadI 30 => r2
  storeAI r2 => rfp, 24
  loadI 60 => r3
  storeAI r3 => rfp, 16
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
    int b;
    b = 30;
    int c;
    c = 30;
    a = b + c;
  }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 29 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  loadI 30 => r1
  storeAI r1 => rfp, 20
  addI rsp, 4 => rsp
  loadI 30 => r2
  storeAI r2 => rfp, 24
  loadAI rfp, 20 => r3
  loadAI rfp, 24 => r4
  add r3, r4 => r5
  storeAI r5 => rfp, 16
  loadAI rfp, 0 => r6
  loadAI rfp, 4 => r7
  loadAI rfp, 8 => r8
  i2i r7 => rsp
  i2i r8 => rfp
  jump -> r6
  `
  );

  await testInput(
    `
  int main() {
    int a;
    int b <= 30;
    int c;
    c = 30;
    a = b + c;
  }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 29 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  loadI 30 => r1
  storeAI r1 => rfp, 20
  addI rsp, 4 => rsp
  loadI 30 => r2
  storeAI r2 => rfp, 24
  loadAI rfp, 24 => r3
  loadI 30 => r4
  add r3, r4 => r5
  storeAI r5 => rfp, 16
  loadAI rfp, 0 => r6
  loadAI rfp, 4 => r7
  loadAI rfp, 8 => r8
  i2i r7 => rsp
  i2i r8 => rfp
  jump -> r6
  `
  );

  await testInput(
    `
  int main() {
    int a;
    int b;
    int c <= 30;
    b = 30;
    a = b + c;
  }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 29 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  loadI 30 => r1
  storeAI r1 => rfp, 24
  loadI 30 => r2
  storeAI r2 => rfp, 20
  loadAI rfp, 20 => r3
  loadI 30 => r4
  add r3, r4 => r5
  storeAI r5 => rfp, 16
  loadAI rfp, 0 => r6
  loadAI rfp, 4 => r7
  loadAI rfp, 8 => r8
  i2i r7 => rsp
  i2i r8 => rfp
  jump -> r6
  `
  );

  await testInput(
    `
  int main() {
    int a;
    int b <= 15;
    int c <= 30;
    int d <= 50;
    a = b + c + d;
  }
  `,
    `
  loadI 1024 => rfp
  loadI 1024 => rsp
  loadI 30 => rbss
  loadI 8 => r0
  storeAI r0 => rsp, 0
  storeAI rsp => rsp, 4
  storeAI rfp => rsp, 8
  jumpI -> L0
  halt
  L0: nop
  i2i rsp => rfp
  addI rsp, 16 => rsp
  addI rsp, 4 => rsp
  addI rsp, 4 => rsp
  loadI 15 => r1
  storeAI r1 => rfp, 20
  addI rsp, 4 => rsp
  loadI 30 => r2
  storeAI r2 => rfp, 24
  addI rsp, 4 => rsp
  loadI 50 => r3
  storeAI r3 => rfp, 28
  loadI 95 => r4
  storeAI r4 => rfp, 16
  loadAI rfp, 0 => r5
  loadAI rfp, 4 => r6
  loadAI rfp, 8 => r7
  i2i r6 => rsp
  i2i r7 => rfp
  jump -> r5
  `
  );
  process.stdout.write(FontColor.Fg.Green);
  console.log("ALL TESTS PASSED!");
  process.stdout.write(FontColor.Reset);
}

test();
