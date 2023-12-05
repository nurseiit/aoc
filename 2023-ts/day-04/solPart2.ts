// import { input } from "./demoInput";
import { input } from "./input";

const toInt = (str: string) => parseInt(str, 10);

const solve = () => {
  const lines = input.split("\n");

  const cards = lines.map((line) => {
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
    const winningCount = currentNumbers.reduceRight((total, currentNumber) => total + (winningSet.has(currentNumber) ? 1 : 0), 0)

    return { winningCount, cardCount: 1 };
  });

  for (let i = 0; i < cards.length; i += 1) {
    const { winningCount, cardCount } = cards[i];
    for (let j = i + 1; j < i + 1 + winningCount && j < cards.length; j += 1) {
      cards[j].cardCount += cardCount;
    }
  }
  
  return cards.reduce((a, b) => a + b.cardCount, 0);
}

console.log(solve());
