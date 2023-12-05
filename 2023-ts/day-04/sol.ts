// import { input } from "./demoInput";
import { input } from "./input";

const toInt = (str: string) => parseInt(str, 10);

const solve = () => {
  const lines = input.split("\n");

  return lines.map((line) => {
    const [, gameInfo] = line.split(":");

    const [currentNumbers, winningNumbers] = gameInfo
      .split("|")
      .map((x) => x.trim())
      .map((str) =>
        str
        .split(" ")
        .map((x) => x.trim())
        .filter((x) => x.length > 0)
        .map(toInt)
      );

    const winningSet = new Set(winningNumbers);

    return currentNumbers.reduceRight((total, currentNumber) => {
      if (winningSet.has(currentNumber)) {
        return total === 0 ? 1 : total * 2;
      } else {
        return total;
      }
    }, 0)
  }).reduce((a, b) => a + b, 0);
}

console.log(solve());
