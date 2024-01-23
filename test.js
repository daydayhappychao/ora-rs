const ora = require("ora");
const myOra = require("./index");

// const spinner = ora("Loading unicorns").start();
// spinner.text = "loading";

const o = new myOra.Ora({
  text: "s",
  prefixText: "你",
  prefixTextFn: "",
  suffixText: "我",
  color: "cyan",
  hideCursour: false,
  indent: 0,
  interval: 100,
  isEnable: true,
  isSilent: false,
  discardStdin: false,
});
o.start("haluomotuo");
while (true) {}
// console.log(124);
