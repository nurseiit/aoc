// import { input } from "./demoInput";
import { input } from "./input";

const toInt = (str: string) => parseInt(str, 10);

type IColor = "red" | "green" | "blue";
type ColorMap = { [color in IColor]: number };

const solve = () => {
  const lines = input.split("\n");

  const validGames = lines.map((line) => {
    const [, gameContent] = line.split(":");

    const maxColors: ColorMap = {
      "red": 0,
      "green": 0,
      "blue": 0
    };

    gameContent.split(";").forEach((turn) => {
      const turnColors: [number, IColor][] = turn
        .split(",")
        .map(x => x.trim().split(" "))
        .map(([count, color]) => [toInt(count), color as IColor]);

      turnColors.forEach(([count, color]) => {
        maxColors[color] = Math.max(maxColors[color], count);
      });
    });

    return maxColors.blue * maxColors.green * maxColors.red;
  });

  return validGames.reduce((a, b) => a + b, 0);
};

console.log(solve());
