enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

function print_color(color: Color) {
    switch (color) {
        case Color.Red:
            console.log("red");
            break;
        case Color.Green:
            console.log("green");
            break;
        case Color.Blue:
            console.log("blue");
            break;
    }
}

const red: Color = Color.Yellow;
print_color(red);
