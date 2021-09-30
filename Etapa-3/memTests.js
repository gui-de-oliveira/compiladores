const util = require("util");
const exec = util.promisify(require("child_process").exec);

async function test() {
    await exec("make");

    for (let i = 0; i <= 76; i++) {
        let f_one = `w`;
        let f_two = i < 10 ? `0` : ``;
        let f_three = i.toString();
        let file = `./TestsE3/`.concat(f_one).concat(f_two).concat(f_three);
        await exec(
            `valgrind --leak-check=full --error-exitcode=2 ./etapa3 < ${file}`
        );
        console.log(`Test ${i} over.`);
    }

    await exec("make clean");
}

test()
