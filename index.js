// rust is a global defined in build/trace.js
rust('./build/trace_bg.wasm').then(main);

let app;
let canvas = document.getElementById("canvas");
let context = canvas.getContext("2d");

function main() {
	app = new rust.App(context, canvas.width, canvas.height);

	canvas.addEventListener("mousedown", function() {app.mouse_left(1)});
	canvas.addEventListener("mouseup", function() {app.mouse_left(0)});
	canvas.addEventListener("mousemove", mousemove);
	window.addEventListener("resize", resize);

	resize();
	update();
}

function mousemove(e) {
	app.mouse_move(e.clientX, e.clientY);
}

function resize() {
	canvas.width = canvas.clientWidth;
	canvas.height = canvas.clientHeight;
	app.resize(canvas.clientWidth, canvas.clientHeight);
}

function update() {
	let t0 = performance.now();
	app.update();
	requestAnimationFrame(update);
	//console.log(performance.now() - t0);
}