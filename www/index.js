import { Universe, Cell } from "web-assembly-game-of-life";
import { memory } from "web-assembly-game-of-life/web_assembly_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";


// settings that can be tweaked

const initialConditionsMap = {
    0: "",
    1: "random",
    2: "copper_head_spaceship",
    3: "glider"
}
const initialConditions = initialConditionsMap[0];

const runOnce = false;
var timeBetweenTicks = 60;

//


const universe = Universe.new(initialConditions);


const width = universe.width();
const height = universe.height();



let minSpeed = 0;
let maxSpeed = 150;
const speedSlider = document.getElementById("speedSlider");
speedSlider.setAttribute("min", 0);
speedSlider.setAttribute("max", maxSpeed);
speedSlider.setAttribute("value", maxSpeed / 2);


speedSlider.oninput = function() {
  console.log(this.value);
  timeBetweenTicks = maxSpeed - this.value;
  console.log(`timeBetweenTicks = ${timeBetweenTicks}`);
}


const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }


    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                         j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}


let firstRun = true;


const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
};


const drawCells = () => {

    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = universe.get_index(row, col);

            ctx.fillStyle = bitIsSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            )
        }
    }
    firstRun = false;
}


let renderOnce = () => {
    return new Promise (function(resolve, reject) {
        // debugger;
        drawGrid();
        drawCells();

        universe.tick();
        resolve();
    })
}

let renderLoop;

if (runOnce) {
    renderLoop = renderOnce;
} else {
    renderLoop = () => {
        renderOnce()
            .then(() => {
                setTimeout(
                  () => {requestAnimationFrame(renderLoop)},
                  timeBetweenTicks
                );
            })
        ;
    }
}

requestAnimationFrame(renderLoop);