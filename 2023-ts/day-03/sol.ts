// import { input } from "./demoInput";
import { input } from "./input";
import { getNeighbours, isInt, isSymbol, toInt } from "./utils";

const solve = () => {
  const lines = input.split('\n').map(line => line.trim().split(""));

  const height = lines.length;

  const marked = lines.map((line, i) =>
    line.map((char, j) => {
      if (!isInt(char)) {
        return false;
      }
      const width = line.length;
      const hasSymbolNeighbour = getNeighbours(i, j)
        .filter(([ni, nj]) => (0 <= ni && ni < height && 0 <= nj && nj < width))
        .map(([ni, nj]) => lines[ni][nj])
        .some(isSymbol);
      return hasSymbolNeighbour;
    }),
  );

  let result = 0;

  for (let i = 0; i < height; i += 1) {
    let currentNumber = 0;
    let isMarked = false; 

    const width = lines[i].length;

    for (let j = 0; j < width; j += 1) {
      const char = lines[i][j];
      if (!isInt(char)) {
        if (isMarked) {
          result += currentNumber;
        }
        isMarked = false;
        currentNumber = 0;
      } else {
        currentNumber = currentNumber * 10 + toInt(char);
        isMarked = isMarked || marked[i][j];
      }
    }

    if (isMarked) {
      result += currentNumber;
    }
  }

  return result;
}

console.log(solve());
