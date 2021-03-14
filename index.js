import("./pkg/static_void.js").then((lib) => {
  console.log(lib.greet("Martin"));
  console.log(lib.greet("Alex"));
});
