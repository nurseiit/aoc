export const isNonNullable = <T>(x: T): x is NonNullable<T> => x != null;

export const isInt = (char: string) => '0' <= char && char <= '9';

export const toInt = (str: string) => parseInt(str, 10);

export const isStar = (char: string) => char === '*';

export const isDot = (char: string) => char === '.';

export const isSymbol = (char: string) => !isInt(char) && !isDot(char);

export const getNeighbours = (i: number, j: number): [number, number][] => {
  const dx = [-1, 0, 1];
  const neighbours: [number, number][] = [];

  dx.forEach(di => {
    dx.forEach(dj => {
      if (di === 0 && dj === 0) {
        return;
      }
      neighbours.push([i + di, j + dj]);
    })
  })

  return neighbours;
}
