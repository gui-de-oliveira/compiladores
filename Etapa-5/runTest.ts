import { writeFileSync } from "fs";
import util from "util";
import { exec as execSync } from "child_process";

const exec = util.promisify(execSync);

const LogColors = {
  Reset: "\x1b[0m",
  Bright: "\x1b[1m",
  Dim: "\x1b[2m",
  Underscore: "\x1b[4m",
  Blink: "\x1b[5m",
  Reverse: "\x1b[7m",
  Hidden: "\x1b[8m",
  Black: "\x1b[30m",
  Red: "\x1b[31m",
  Green: "\x1b[32m",
  Yellow: "\x1b[33m",
  Blue: "\x1b[34m",
  Magenta: "\x1b[35m",
  Cyan: "\x1b[36m",
  White: "\x1b[37m",
};

let testsCounter = 0;

function log(message: string, color?: keyof typeof LogColors) {
  if (color !== undefined) {
    process.stdout.write(LogColors[color]);
  }

  console.log(message);
  process.stdout.write(LogColors.Reset);
}

type Success<T> = { success: true; value: T };
type Error<T> = { success: false; value: T };
type Result<S, E> = Success<S> | Error<E>;

function Ok<T>(value: T): Success<T> {
  return { success: true, value };
}

function Err<T>(value: T): Error<T> {
  return { success: false, value };
}

async function compile(
  input: string
): Promise<Result<{ compilerOutput: string }, { error: unknown }>> {
  try {
    const inputFile = `code.temp`;
    writeFileSync(inputFile, input);

    const { stdout: compilerOutput } = await exec(`./etapa5 < ${inputFile}`);

    return { success: true, value: { compilerOutput } };
  } catch (error) {
    return { success: false, value: { error } };
  }
}

type UnknownError = { type: "UNKNOWN"; error: unknown };
type IlocErrors =
  | UnknownError
  | { type: "NO_MEMORY_VALUES_SECTION_FIND"; ilocOutput: string }
  | { type: "COULDNT_SPLIT_LINES"; failedLines: string[]; ilocOutput: string }
  | {
      type: "VALUE_IS_NOT_A_NUMBER";
      failedLines: { memory: string; value: string }[];
      ilocOutput: string;
    };

async function getIloc(
  compilerOutput: string
): Promise<
  Result<{ memoryValues: { memory: number; value: number }[] }, IlocErrors>
> {
  const inputFile = `iloc.temp`;
  writeFileSync(inputFile, compilerOutput);

  const ilocResult = await exec(`python3 ilocsim.py -s -t ${inputFile}`).then(
    ({ stdout }) => Ok({ output: stdout }),
    (error: unknown) => Err<UnknownError>({ type: "UNKNOWN", error })
  );

  if (!ilocResult.success) {
    return Err<UnknownError>(ilocResult.value);
  }

  const { output } = ilocResult.value;

  const outputLines = output.split("\n").filter((value) => value !== "");

  const memoryValueLines = outputLines.findIndex(
    (line) => line.includes("memory") && line.includes("value")
  );

  if (memoryValueLines === -1) {
    return Err({
      type: "NO_MEMORY_VALUES_SECTION_FIND" as const,
      ilocOutput: output,
    });
  }

  const memoryValueSection = outputLines.slice(memoryValueLines + 1);

  const parsedLines = memoryValueSection.map((line) => {
    const values = line.split(" ").filter((value) => value !== "");

    if (values.length !== 2) {
      return Err({ line });
    }

    return Ok<[string, string]>(values as [string, string]);
  });

  const {
    failedLines: linesFailedToSplit,
    successfulLines: successfullySplittedLines,
  } = parsedLines.reduce<{
    failedLines: string[];
    successfulLines: [string, string][];
  }>(
    (previous, current) => {
      if (current.success) {
        previous.successfulLines.push(current.value);
      } else {
        previous.failedLines.push(current.value.line);
      }

      return previous;
    },
    {
      failedLines: [],
      successfulLines: [],
    }
  );

  if (linesFailedToSplit.length > 0) {
    return Err({
      type: "COULDNT_SPLIT_LINES" as const,
      failedLines: linesFailedToSplit,
      ilocOutput: output,
    });
  }

  const { failedLines, successfulLines } = successfullySplittedLines.reduce<{
    failedLines: { memory: string; value: string }[];
    successfulLines: { memory: number; value: number }[];
  }>(
    (previous, current) => {
      const memory = parseInt(current[0]);
      const value = parseInt(current[1]);
      if (isNaN(memory) || isNaN(value)) {
        previous.failedLines.push({ memory: current[0], value: current[1] });
      } else {
        previous.successfulLines.push({ memory, value });
      }

      return previous;
    },
    {
      failedLines: [],
      successfulLines: [],
    }
  );

  if (failedLines.length > 0) {
    return Err({
      type: "VALUE_IS_NOT_A_NUMBER" as const,
      failedLines,
      ilocOutput: output,
    });
  }

  return { success: true, value: { memoryValues: successfulLines } };
}

async function test(input: string, expectedValues: number[]) {
  const outputFile = `output.temp`;

  const compileResult = await compile(input);

  if (!compileResult.success) {
    log(`Test ${testsCounter} CRASHED!`, "Red");
    log(`Input: "${input}"`, "Red");
    log(`Error: ${compileResult.value.error}`, "Red");

    process.exit();
  }

  const ilocResult = await getIloc(compileResult.value.compilerOutput);

  if (!ilocResult.success) {
    log(`TEST ${testsCounter} FAILED!`, "Red");
    log(`Input: "${input}"`, "Red");
    log(`PARSE ERROR!`, "Red");

    switch (ilocResult.value.type) {
      case "COULDNT_SPLIT_LINES":
        log(
          `The following lines resulted couldnt be splitted in just two values: ${ilocResult.value.failedLines}!`,
          "Red"
        );
        break;

      case "NO_MEMORY_VALUES_SECTION_FIND":
        log(`Memory Values section not found!`, "Red");
        break;

      case "VALUE_IS_NOT_A_NUMBER":
        log(
          `The following values couldnt be parsed into numbers: ${ilocResult.value.failedLines}`,
          "Red"
        );
        break;

      case "UNKNOWN":
        log(`UNKNOWN ERROR: ${ilocResult.value.error}`, "Red");
        break;
    }

    if (ilocResult.value.type !== "UNKNOWN") {
      log(`\nILOC output: ${ilocResult.value.ilocOutput}`, "Red");
    }

    process.exit();
  }

  const { memoryValues } = ilocResult.value;

  const assert = <T>(expected: T, received: T) => {
    if (expected === received) return;

    log(`TEST ${testsCounter} FAILED!`, "Red");
    log(`Input: "${input}"`, "Red");

    log(`ASSERT ERROR!`, "Red");
    log(`Expected: ${expected}`, "Green");
    log(`Received: ${received}`, "Red");

    log(
      `memoryValues: ${memoryValues.map(
        (value) => `{memory: ${value.memory} value:${value.value}}`
      )}`,
      "Red"
    );

    process.exit();
  };

  const DEFAULT_MEMORY_VALUES = 3;
  assert(expectedValues.length + DEFAULT_MEMORY_VALUES, memoryValues.length);

  assert(8, memoryValues[0].value);
  assert(1024, memoryValues[1].value);
  assert(1024, memoryValues[2].value);

  memoryValues.slice(DEFAULT_MEMORY_VALUES).forEach((memoryValue, i) => {
    assert(expectedValues[i], memoryValue.value);
  });

  log(`TEST ${testsCounter} SUCCEEDED!`, "Green");
}

async function runTests() {
  await test("int main() { }", []);

  log("ALL TESTS PASSED!", "Green");
}

runTests();
