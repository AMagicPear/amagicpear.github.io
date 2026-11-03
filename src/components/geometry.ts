export const initialPieces = [
  { name: '红色圆盘', shape: 'disc', x: 8, y: 8, angle: 0 },
  { name: '蓝色拱门', shape: 'arch', x: 51, y: 7, angle: 0 },
  { name: '黄色三角', shape: 'triangle', x: 9, y: 53, angle: -15 },
  { name: '条纹方块', shape: 'stripes', x: 53, y: 53, angle: 0 },
  { name: '黑色圆环', shape: 'ring', x: 33, y: 32, angle: 0 },
];

// Keep each 32%-wide piece inside its board, including keyboard moves.
export function movePiece(piece: { x: number; y: number }, dx: number, dy: number) {
  return { x: Math.max(0, Math.min(68, piece.x + dx)), y: Math.max(0, Math.min(68, piece.y + dy)) };
}

export function arrangePieces(arrangement: number) {
  const layouts = [[8, 8, 51, 7, 9, 53, 53, 53, 33, 32], [5, 44, 48, 3, 44, 57, 6, 4, 37, 30], [34, 3, 5, 35, 57, 32, 34, 65, 33, 34]];
  return initialPieces.map((piece, i) => ({ ...piece, x: layouts[arrangement][i * 2], y: layouts[arrangement][i * 2 + 1], angle: piece.angle + arrangement * 45 }));
}
