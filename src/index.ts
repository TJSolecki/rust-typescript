type Custom = {
    age: number,
    name: string,
}

type Item = number | string | Custom;

function append(items: Item[]) {
    items.push("Hello Fem!");
}

const items: Item[] = [1, "bang", { age: 1, name: "thomas"}];
append(items);
console.log(items);

const numbers: number[] = [1, 2, 3];
console.log(numbers);
append(numbers);
console.log(numbers);
