import fs from "fs";

const fileName = String(process.argv[2]);
fs.readFileSync(fileName, { encoding: "utf8"} )
    .toString()
    .split("\n")
    .forEach(line => {
        const print = parseInt(line);
        if (isNaN(print)) {
            return console.log('line not an number');
        }
        console.log(line)
    });
