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

o.text = "打个中国结，再系个\n红腰带，天天好运来"

while (true) {
  if(Date.now() - startDate > 6 * 1000) {
    break;
  }
}

o.spinner = "balloon2";

while (true) {
  if(Date.now() - startDate > 9 * 1000) {
    break;
  }
}


o.stop();
console.log('真的很高兴，真的')
while(true){}
// console.log(124);
