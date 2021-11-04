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
    process.stdout.write(`Test ${testsCounter} passed!`);
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
    `loadI 1024 => rfp
loadI 1024 => rsp
loadI 7 => rbss
jumpI => L0
L0: addI rsp, 0 => rsp
halt
`
  );

  process.stdout.write(FontColor.Fg.Green);
  console.log("ALL TESTS PASSED!");
  process.stdout.write(FontColor.Reset);
}

test();
