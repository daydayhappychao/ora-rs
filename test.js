const myOra = require("./bundle");

// const spinner = ora("Loading unicorns").start();
// spinner.text = "loading";
// console.log(myOra);
const o = myOra.default("很很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你很高兴遇见你");
o.start();
let startDate = Date.now();
// console.log(Date.now())
while (true) {
  if(Date.now() - startDate > 3 * 1000) {
    break;
  }
}
// console.log(Date.now())

o.stop();
console.log('真的很高兴，真的')
while(true){}
// console.log(124);
