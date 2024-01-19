import test from "ava";

import { Width, plusOne } from "../index.js";
import ora from "ora";

test("sum from native", async (t) => {
  const width = new Width(1);

  // plusOne.call(width);
  const spinner = ora("Loading unicorns").start();
  spinner.text = "loading";
  setTimeout(() => {
    spinner.color = "yellow";
    spinner.text = "Loading rainbows";
  }, 1000);
  // while (true) {}
  // await new Promise((resolve, reject) => {
  //   setTimeout(() => {
  //     resolve();
  //   }, 3000);
  // });
  t.is(3, 3);
});
