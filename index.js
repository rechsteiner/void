import("./pkg/static_void.js").then((lib) => {
  let editor = document.getElementById("editor");
  let runButton = document.getElementById("run-button");
  let game = new lib.Game();

  runButton.addEventListener("click", function () {
    console.log(editor.value);
    game.change_program(editor.value);
  });

  function animate() {
    game.next_simulation_step();
    requestAnimationFrame(() => animate());
  }

  animate();
});
