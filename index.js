import("./pkg/static_void.js").then((lib) => {
  let game = new lib.Game();
  game.changeProgram("5 + 5;");
});
