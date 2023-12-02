import { input } from "./input";

const numberWords = "one, two, three, four, five, six, seven, eight, nine".split(", ");

const numberMap = Object.fromEntries(numberWords.map((word, index) => [word, `${index + 1}`]));

const isNonNullable = <T>(x: T): x is NonNullable<T> => x != null;

const getFirstAndLastNumberWord = (str: string): [{word: string, index: number}, { word: string, index: number }] | null => {
  const wordFirstIndexes = numberWords.map(word => {
    const index = str.indexOf(word);
    if (index !== -1) {
      return { word, index };
    }
  }).filter(isNonNullable).sort((a, b) => a.index - b.index);

  const wordLastIndexes = numberWords.map(word => {
    const index = str.lastIndexOf(word);
    if (index !== -1) {
      return { word, index };
    }
  }).filter(isNonNullable).sort((a, b) => a.index - b.index);

  if (wordFirstIndexes.length === 0) {
    return null;
  }

  return [wordFirstIndexes[0], wordLastIndexes[wordLastIndexes.length - 1]];
}

const normalizeString = (str: string) => {
  const firstAndLast = getFirstAndLastNumberWord(str);
  if (firstAndLast == null) {
    return str;
  }
  const [first, last] = firstAndLast;
  return str.slice(0, first.index) + numberMap[first.word]
    + str.slice(first.index, last.index) + numberMap[last.word]
    + str.slice(last.index);
}

const isDigit = (char: string) => char >= "0" && char <= "9";

const filterOnlyNumbers = (str: string) => str.split("").filter(isDigit).join("");

const solveLine = (line: string) => {
  const normalized = normalizeString(line);
  const filtered = filterOnlyNumbers(normalized);
  if (filtered.length === 0) {
    return 0;
  }
  return +filtered[0] * 10 + +filtered[filtered.length - 1];
}

const solve = (lines: string[]) => lines.map(solveLine).reduce((a, b) => a + b, 0);

const inputArray = input.split("\n");

console.log(solve(inputArray));

// inputArray.forEach(line => {
//   console.log(line, solveLine(line));
// });
