import("./pkg/static_void.js").then((lib) => {
  let editor = document.getElementById("editor");
  let game = new lib.Game();

  editor.addEventListener("input", function () {
    console.log(editor.value);
    game.change_program(editor.value);
  });
});
