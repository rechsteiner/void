import { Editor } from "./editor";

type Error = { message: string };

import("./pkg/static_void.js").then((lib) => {
  const pauseButton = document.getElementById("pause-button")!;
  const runButton = document.getElementById("run-button")!;
  const canvas = document.getElementsByTagName("canvas")[0];
  const editorElement = document.getElementById("editor")!;
  const editorErrors = document.getElementById("editor-errors")!;

  let game = new lib.Game();
  let parserError = false;

  function showErrors(errors?: Error[]) {
    if (errors && errors.length > 0) {
      editorErrors.classList.remove("hidden");
      editorErrors.innerHTML = "";
      for (let error of errors) {
        let paragraph = window.document.createElement("p");
        paragraph.textContent = error.message;
        editorErrors.appendChild(paragraph);
      }
    } else {
      editorErrors.classList.add("hidden");
    }
  }

  function changeProgram(document: string) {
    let errors = game.change_program(document);
    if (errors && errors.length > 0) {
      parserError = true;
    }
    showErrors(errors);
  }

  let editor = new Editor(editorElement, {
    onChange: (document) => {
      changeProgram(document);
    },
  });

  changeProgram(editor.document);

  let isPaused = false;

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
    game.keydown(e.key);
  });

  window.addEventListener("keyup", (e) => {
    game.keyup(e.key);
  });

  // Run game loop on each frame
  function animate() {
    if (!isPaused) {
      let error = game.tick();
      if (error) {
        showErrors([error]);
      } else if (!parserError) {
        showErrors([]);
      }
    }

    requestAnimationFrame(() => animate());
  }

  animate();
});
