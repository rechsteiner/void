import { Editor } from "./editor";
import { keys } from "./config";

type ProgramVariable = { name: string; value: { Integer: any } };

import("./pkg/static_void.js").then((lib) => {
  const pauseButton = document.getElementById("pause-button")!;
  const runButton = document.getElementById("run-button")!;
  const canvas = document.getElementsByTagName("canvas")[0];
  const editorElement = document.getElementById("editor")!;
  const editorVariablesList = document.getElementById("editor-variables-list")!;

  let game = new lib.Game();

  let editor = new Editor(editorElement, {
    onChange: (document) => {
      game.change_program(document);
    },
  });

  game.change_program(editor.document);

  let isPaused = false;
  let viewport_movement_input = {
    x: 0.0,
    y: 0.0,
    zoom: 0.0,
  };

  // Set canvas size attributes to match physical size of window
  canvas.setAttribute("height", `${window.innerHeight}`);
  canvas.setAttribute("width", `${window.innerWidth}`);

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
    canvas.setAttribute("height", `${window.innerHeight}`);
    canvas.setAttribute("width", `${window.innerWidth}`);
  });

  // Navigate viewport
  window.addEventListener("keydown", (e) => {
    // Possibly a hack, but the idea is to only move the viewport when "nothing" is selected.
    // Otherwise the viewport moves when typing into textarea.
    if (document.activeElement?.nodeName !== "BODY") return;

    // TODO: abstract away specific keys from this code
    switch (e.key) {
      case keys.VIEWPORT_UP:
        viewport_movement_input.y = -1;
        break;
      case keys.VIEWPORT_DOWN:
        viewport_movement_input.y = 1;
        break;
      case keys.VIEWPORT_LEFT:
        viewport_movement_input.x = -1;
        break;
      case keys.VIEWPORT_RIGHT:
        viewport_movement_input.x = 1;
        break;
      case keys.VIEWPORT_ZOOM_OUT:
        viewport_movement_input.zoom = -1;
        break;
      case keys.VIEWPORT_ZOOM_IN:
        viewport_movement_input.zoom = 1;
        break;
    }
  });

  window.addEventListener("keyup", (e) => {
    // Possibly a hack, but the idea is to only move the viewport when "nothing" is selected.
    // Otherwise the viewport moves when typing into textarea.
    if (document.activeElement?.nodeName !== "BODY") return;

    // TODO: abstract away specific keys from this code
    switch (e.key) {
      case keys.VIEWPORT_UP:
        viewport_movement_input.y = 0;
        break;
      case keys.VIEWPORT_DOWN:
        viewport_movement_input.y = 0;
        break;
      case keys.VIEWPORT_LEFT:
        viewport_movement_input.x = 0;
        break;
      case keys.VIEWPORT_RIGHT:
        viewport_movement_input.x = 0;
        break;
      case keys.VIEWPORT_ZOOM_OUT:
        viewport_movement_input.zoom = 0;
        break;
      case keys.VIEWPORT_ZOOM_IN:
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
      // TODO: Only update the program when the editor changes. We currently
      // re-interpret the whole program on each frame which is very unnecessary.
      game.tick();
    }

    let variables: ProgramVariable[] = game.get_program_variables();
    let sortedVariables = Object.entries(variables).sort((a, b) =>
      a[0] > b[0] ? 1 : -1
    );

    editorVariablesList.innerHTML = ""; // Clear list
    sortedVariables.forEach((variable) => {
      let variableElement = document.createElement("li");

      // Span with name
      let nameElement = document.createElement("span");
      nameElement.classList.add("variable-name");
      nameElement.textContent = variable[0];
      variableElement.appendChild(nameElement);

      // Span with value
      let valueElement = document.createElement("span");
      valueElement.classList.add("variable-value");
      let value = Object.values(variable[1])[0];
      // @ts-ignore
      valueElement.textContent = (value as number).toFixed(2);
      variableElement.appendChild(valueElement);

      editorVariablesList.appendChild(variableElement);
    });

    requestAnimationFrame(() => animate());
  }

  animate();
});
