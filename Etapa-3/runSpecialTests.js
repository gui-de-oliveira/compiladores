const fs = require("fs").promises;
const util = require("util");
const exec = util.promisify(require("child_process").exec);

function numberToLetter(value) {
  // Skip the 6 ascii characters between upper and lower-case.
  return String.fromCharCode((value < 26 ? 65 : 71) + value);
}

function normalizeMemoryValues(string) {
  const match = string.match(/0*x[0-9a-z]+/g);
  const memoryValues = match === null ? [] : [...new Set(match)];
  const replaced = memoryValues.reduce((previous, memoryValue) => {
    const id = memoryValues.findIndex((m) => m === memoryValue);
    const letter = numberToLetter(id);

    return previous.split(memoryValue).join(letter);
  }, string);

  return replaced;
}

function formatOutput(rawOutput) {
  const formattedOutput = normalizeMemoryValues(rawOutput)
    .split(" ->")
    .join(",")
    .split("\n")
    .filter((line) => !line.startsWith("digraph"))
    .filter((line) => !line.startsWith("}"))
    .filter((line) => line !== "")
    .join("\n");

  return formattedOutput;
}

function toGraph(string) {
  return `digraph {
${string.split(",").join("->")}
}`;
}

async function testFile(file) {
  const expectedOutput = await fs.readFile(`TestsE3/${file}.ref.dot`, "utf-8");

  const formattedExpectedOutput = formatOutput(expectedOutput);

  const { stdout: receivedOutput } = await exec(`./etapa3 < TestsE3\/${file}`);
  const formattedReceivedOutput = normalizeMemoryValues(receivedOutput).trim();

  const result = formattedExpectedOutput === formattedReceivedOutput;

  if (!result) {
    console.log("Expected:");
    console.log(toGraph(formattedExpectedOutput));
    console.log("");
    console.log("Received:");
    console.log(toGraph(formattedReceivedOutput));
  }

  return result;
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

async function test() {
  const files = (await fs.readdir("TestsE3")).filter(
    (file) => !file.endsWith(".ref.dot")
  );

  let totalSuccess = 0;
  let totalFailed = 0;
  let testCounter = 0;

  for (const file of files) {
    const result = await testFile(file);
    testCounter++;

    if (result) {
      totalSuccess += 1;
      console.log(
        FontColor.Fg.Green + `Test ${testCounter} SUCCESS!` + FontColor.Reset
      );
    } else {
      totalFailed += 1;
      console.log(
        FontColor.Fg.Red + `Test ${testCounter} FAILED!` + FontColor.Reset
      );
      process.exit();
    }
  }
}

test();
