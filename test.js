const myOra = require("./bundle");

// const spinner = ora("Loading unicorns").start();
// spinner.text = "loading";
// console.log(myOra);
const o = myOra.default("很高兴遇见你");
o.start();
let startDate = Date.now();
// console.log(Date.now())
while (true) {
  if(Date.now() - startDate > 3 * 1000) {
    break;
  }
}
// console.log(Date.now())

o.stop()
while(true){}
// console.log(124);
