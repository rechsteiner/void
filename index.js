import("./pkg/static_void.js").then((lib) => {
  const editor = document.getElementById("editor");
  const pauseButton = document.getElementById("pause-button");
  const runButton = document.getElementById("run-button");
  const canvas = document.getElementsByTagName("canvas")[0];
  let game = new lib.Game();
  let isPaused = false;
  let viewport_movement_input = {
    x: 0.0,
    y: 0.0,
    zoom: 0.0,
  };

  // Set canvas size attributes to match physical size of window
  canvas.setAttribute("height", window.innerHeight);
  canvas.setAttribute("width", window.innerWidth);

  // Save editor text
  editor.addEventListener("change", () => {
    window.localStorage.setItem("editorValue", editor.value);
  });

  if (window.localStorage.getItem("editorValue")) {
    editor.value = window.localStorage.getItem("editorValue");
  }

  // Hide buttons when not in use
  runButton.classList.add("hidden");

  // Button click handlers
  pauseButton.addEventListener("click", function () {
    isPaused = true;
    pauseButton.classList.add("hidden");
    runButton.classList.remove("hidden");
  });

  runButton.addEventListener("click", function () {
    isPaused = false;
    pauseButton.classList.remove("hidden");
    runButton.classList.add("hidden");
  });

  // Update canvas dimension attributes on window resize
  window.addEventListener("resize", function () {
    canvas.setAttribute("height", window.innerHeight);
    canvas.setAttribute("width", window.innerWidth);
  });

  // Navigate viewport
  window.addEventListener("keydown", (e) => {
    // Possibly a hack, but the idea is to only move the viewport when "nothing" is selected.
    // Otherwise the viewport moves when typing into textarea.
    if (document.activeElement.nodeName !== "BODY") return;

    // TODO: abstract away specific keys from this code
    switch (e.key) {
      case "w":
        viewport_movement_input.y = -1;
        break;
      case "s":
        viewport_movement_input.y = 1;
        break;
      case "a":
        viewport_movement_input.x = -1;
        break;
      case "d":
        viewport_movement_input.x = 1;
        break;
      case "z":
        viewport_movement_input.zoom = -1;
        break;
      case "x":
        viewport_movement_input.zoom = 1;
        break;
    }
  });

  window.addEventListener("keyup", (e) => {
    // Possibly a hack, but the idea is to only move the viewport when "nothing" is selected.
    // Otherwise the viewport moves when typing into textarea.
    if (document.activeElement.nodeName !== "BODY") return;

    // TODO: abstract away specific keys from this code
    switch (e.key) {
      case "w":
        viewport_movement_input.y = 0;
        break;
      case "s":
        viewport_movement_input.y = 0;
        break;
      case "a":
        viewport_movement_input.x = 0;
        break;
      case "d":
        viewport_movement_input.x = 0;
        break;
      case "z":
        viewport_movement_input.zoom = 0;
        break;
      case "x":
        viewport_movement_input.zoom = 0;
        break;
    }
  });

  // TODO: Consider handling in rust instead?
  function move_viewport() {
    const movement_step = 15.0;
    const zoom_step = 0.04;

    game.move_render_viewport(
      viewport_movement_input.x * movement_step,
      viewport_movement_input.y * movement_step,
      viewport_movement_input.zoom * zoom_step
    );
  }

  // Run game loop on each frame
  function animate() {
    move_viewport();

    if (!isPaused) {
      game.change_program(editor.value);
      game.next_simulation_step();
    }

    requestAnimationFrame(() => animate());
  }

  animate();
});
