import("./pkg/static_void.js").then((lib) => {
  const editor = document.getElementById("editor");
  const pauseButton = document.getElementById("pause-button");
  const runButton = document.getElementById("run-button");
  const canvas = document.getElementsByTagName("canvas")[0];
  let game = new lib.Game();
  let isPaused = false;

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

  // Run game loop on each frame
  function animate() {
    if (!isPaused) {
      game.change_program(editor.value);
      game.next_simulation_step();
    }

    requestAnimationFrame(() => animate());
  }

  animate();
});
