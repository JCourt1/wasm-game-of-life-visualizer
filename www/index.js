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
}
const initialConditions = initialConditionsMap[0];

const runOnce = false;
var timeBetweenTicks = 60;

//


const universe = Universe.new(initialConditions);


const width = universe.width();
const height = universe.height();

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


const getIndex = (row, column) => {
    return row * width + column;
}

let firstRun = true;

const drawCells = () => {

    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

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
                  () => {console.log(timeBetweenTicks); requestAnimationFrame(renderLoop)},
                  timeBetweenTicks
                );
            })
        ;
    }
}

requestAnimationFrame(renderLoop);