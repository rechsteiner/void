import("./pkg/static_void.js").then((lib) => {
  const editor = document.getElementById("editor");
  const pauseButton = document.getElementById("pause-button");
  const runButton = document.getElementById("run-button");
  let game = new lib.Game();
  let isPaused = false;

  editor.addEventListener("change", () => {
    window.localStorage.setItem("editorValue", editor.value);
  });

  if (window.localStorage.getItem("editorValue")) {
    editor.value = window.localStorage.getItem("editorValue");
  }

  runButton.classList.add("hidden");

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

  function animate() {
    if (!isPaused) {
      game.change_program(editor.value);
      game.next_simulation_step();
    }

    requestAnimationFrame(() => animate());
  }

  animate();
});
