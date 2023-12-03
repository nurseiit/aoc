// import { input } from "./demoInput";
import { input } from "./input";

import { getNeighbours, isInt, isNonNullable, isStar, toInt } from "./utils";

/**
 * the idea is to:
 *  1. convert all numbers to unique symbols and mark all cells that they occupy with those symbols
 *  2. lookup all symbols from neighbours of "star" characters and if that set's size is exactly 2, then add the product
 */
const solve = () => {
  const numbersTable = new Map<Symbol, number>();

  const lines = input.split('\n').map(line => line.trim().split(""));

  // this is where we map lines to maps of symbols for number in those lines
  const markedLineMaps = lines.map((line) => {
    const marked = new Map<number, Symbol>();

    const addNumber = (digits: string, endIndex: number) => {
      const symbol = Symbol();
      numbersTable.set(symbol, toInt(digits));

      const [from, to] = [endIndex - digits.length, endIndex];

      for (let i = from; i < to; i += 1) {
        marked.set(i, symbol);
      }
    }

    const width = line.length;

    let digits = "";

    line.forEach((char, i) => {
      if (!isInt(char)) {
        if (digits.length > 0) {
          addNumber(digits, i);
        }
        digits = "";
      } else {
        digits += char;
      }
    });

    if (digits.length > 0) {
      addNumber(digits, width);
    }

    return marked;
  });

  const height = lines.length;

  return lines.map((line, i) => {
    const width = line.length;

    return line.map((char, j) => {
      if (!isStar(char)) {
        return 0;
      }

      const neighboursSet = new Set(
        getNeighbours(i, j)
          .filter(([ni, nj]) => 0 <= ni && ni < height && 0 <= nj && nj < width)
          .map(([ni, nj]) => markedLineMaps[ni].get(nj))
          .filter(isNonNullable)
      );

      if (neighboursSet.size !== 2) {
        return 0;
      }

      const [first, second] = Array.from(neighboursSet).map(symbol => numbersTable.get(symbol)).filter(isNonNullable);
      return first * second;
    }).reduce((a, b) => a + b, 0);
  }).reduce((a, b) => a + b, 0);
}

console.log(solve());
