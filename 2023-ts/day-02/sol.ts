// import { input } from "./demoInput";
import { input } from "./input";

const toInt = (str: string) => parseInt(str, 10);

type IColor = "red" | "green" | "blue";

const validColorCounts: { [color in IColor]: number } = {
  "red": 12,
  "green": 13,
  "blue": 14,
};

const solve = () => {
  const lines = input.split("\n");

  const validGames = lines.map((line) => {
    const [gameInfo, gameContent] = line.split(":");
    const gameNumber = toInt(gameInfo.split(" ")[1]);

    const gameTurnsValidity = gameContent.split(";").map((turn) => {
      const turnColors: [number, IColor][] = turn
        .split(",")
        .map(x => x.trim().split(" "))
        .map(([count, color]) => [toInt(count), color as IColor]);

      return turnColors.every(([count, color]) => validColorCounts[color] >= count);
    });

    return gameTurnsValidity.every(Boolean) ? gameNumber : 0;
  });

  return validGames.reduce((a, b) => a + b, 0);
};

console.log(solve());
