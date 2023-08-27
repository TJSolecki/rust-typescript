import fs from "fs";

fs.readFileSync("text.txt", { encoding: "utf8"} )
    .toString()
    .split("\n")
    .filter((_: string, i: number) => i % 2 === 0)
    .filter((_: string, i: number) => i > 1 && i < 4)
    .forEach(line => console.log(line));
