import fs from "fs";

const fileName = String(process.argv[2]);
fs.readFileSync(fileName, { encoding: "utf8"} )
    .toString()
    .split("\n")
    .forEach(line => console.log(line));
