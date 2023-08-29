type NumOrUndefined = number | undefined;
function practice(arr: NumOrUndefined[]) {
    return arr.map((val: NumOrUndefined, i: number) => (
        val !== undefined ? val * 5 : i * 5
    ))
}

const arr = [undefined, 4,undefined];
const newArr = practice(arr);
console.log(newArr);
