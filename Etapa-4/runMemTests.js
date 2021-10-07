const util = require("util");
const exec = util.promisify(require("child_process").exec);

async function testFile(i) {
  let f_one = `w`;
  let f_two = i < 10 ? `0` : ``;
  let f_three = i.toString();
  let file = `./TestsE3/`.concat(f_one).concat(f_two).concat(f_three);
  await exec(
    `valgrind --leak-check=full --error-exitcode=2 ./etapa4 < ${file}`
  );
  console.log(`Test ${i} over.`);
}

async function test() {
  const promises = [];
  for (let i = 0; i <= 76; i++) {
    console.log(`Started test ${i}...`);
    promises.push(testFile(i));
  }

  Promise.all(promises);
}

test();
