import {Core, hash_string} from "mech-wasm";

let mech_core = Core.new();

let balls = hash_string("ball");

// ## Controls

let controls = document.createElement("div");
controls.setAttribute("class", "controls");

let compile = document.createElement("button");
compile.setAttribute("id", "compile");
compile.innerHTML =  "Compile";
controls.appendChild(compile);

let view_core = document.createElement("button");
view_core.setAttribute("id", "view core");
view_core.innerHTML =  "View Core";
controls.appendChild(view_core);

let view_runtime = document.createElement("button");
view_runtime.setAttribute("id", "view runtime");
view_runtime.innerHTML =  "View Runtime";
controls.appendChild(view_runtime);

let clear_core = document.createElement("button");
clear_core.setAttribute("id", "clear core");
clear_core.innerHTML =  "Clear Core";
controls.appendChild(clear_core);

let get_ball = document.createElement("button");
get_ball.setAttribute("id", "get balls");
get_ball.innerHTML =  "Get Balls";
controls.appendChild(get_ball);

let txn = document.createElement("button");
txn.setAttribute("id", "txn");
txn.innerHTML =  "Add Txn";

// ## Editor

let editor = document.createElement("div");
editor.setAttribute("class", "editor");

let code = document.createElement("textarea");
code.setAttribute("class", "code");
code.setAttribute("id", "code");
code.innerHTML =  "#ball = [1 2 3; 4 5 6; 7 8 9]";

let canvas = document.createElement("canvas");
canvas.setAttribute("class", "canvas");
canvas.setAttribute("width", "500");
canvas.setAttribute("height", "420");
canvas.style.backgroundColor = 'rgb(226, 79, 94)';

editor.appendChild(code);
editor.appendChild(canvas);

// ## Container

let container = document.createElement("div");
container.setAttribute("class","container");

container.appendChild(controls);
container.appendChild(editor);

document.body.appendChild(container);

// ## Event handlers

document.getElementById("compile").addEventListener("click", function(click) {
  console.log(click);
  let code = document.getElementById("code");
  mech_core.compile_code(code.value);
});

document.getElementById("view core").addEventListener("click", function() {
  mech_core.display_core();
});

document.getElementById("view runtime").addEventListener("click", function() {
  mech_core.display_runtime();
});

document.getElementById("clear core").addEventListener("click", function() {
  mech_core.clear();
});

document.getElementById("get balls").addEventListener("click", function() {
  let column = mech_core.get_table(balls);
  console.log(column);
});

/*document.getElementById("txn").addEventListener("click", function() {
  mech_core.process_transaction("test",1,1,3);
});*/