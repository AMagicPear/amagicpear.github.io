<script lang="ts">
  import { arrangePieces, movePiece } from './geometry';
  let pieces = $state(arrangePieces(0));
  let board: HTMLDivElement;
  let arrangement = $state(0);
  let dragging = $state<number | null>(null);
  let drag: { pointer: number; x: number; y: number; startX: number; startY: number; moved: boolean } | null = null;

  function move(event: PointerEvent, index: number) {
    if (!drag || drag.pointer !== event.pointerId || dragging !== index) return;
    const bounds = board.getBoundingClientRect();
    const dx = event.clientX - drag.x;
    const dy = event.clientY - drag.y;
    drag.moved ||= Math.hypot(dx, dy) > 4;
    Object.assign(pieces[index], movePiece({ x: drag.startX, y: drag.startY }, dx / bounds.width * 100, dy / bounds.height * 100));
  }

  function release(event: PointerEvent, index: number) {
    if (!drag || drag.pointer !== event.pointerId) return;
    if (!drag.moved) pieces[index].angle += 45;
    drag = null;
    dragging = null;
  }

  function keyboard(event: KeyboardEvent, index: number) {
    const delta: Record<string, [number, number]> = { ArrowLeft: [-3, 0], ArrowRight: [3, 0], ArrowUp: [0, -3], ArrowDown: [0, 3] };
    if (!delta[event.key]) return;
    event.preventDefault();
    Object.assign(pieces[index], movePiece(pieces[index], ...delta[event.key]));
  }

  function rearrange() {
    arrangement = (arrangement + 1) % 3;
    pieces = arrangePieces(arrangement);
  }
</script>

<div class="geometry-playground">
  <div class="geometry-board" bind:this={board}>
    <span class="board-axis axis-x" aria-hidden="true"></span>
    <span class="board-axis axis-y" aria-hidden="true"></span>
    {#each pieces as piece, index}
      <button
        class={`geometry-piece ${piece.shape}`}
        class:dragging={dragging === index}
        style={`--x:${piece.x}%;--y:${piece.y}%;--angle:${piece.angle}deg`}
        aria-label={`${piece.name}：点击旋转，方向键移动`}
        onpointerdown={(event) => {
          if (event.button !== 0 || drag) return;
          event.currentTarget.setPointerCapture(event.pointerId);
          dragging = index;
          drag = { pointer: event.pointerId, x: event.clientX, y: event.clientY, startX: piece.x, startY: piece.y, moved: false };
        }}
        onpointermove={(event) => move(event, index)}
        onpointerup={(event) => release(event, index)}
        onlostpointercapture={() => { drag = null; dragging = null; }}
        onpointercancel={() => { drag = null; dragging = null; }}
        onkeydown={(event) => keyboard(event, index)}
        onclick={(event) => { if (event.detail === 0) piece.angle += 45; }}
      ><span aria-hidden="true"></span></button>
    {/each}
    <span class="board-caption" aria-hidden="true">形状没有标准答案。</span>
  </div>
  <div class="playground-controls">
    <p>拖一拖，转一转。<span>点击旋转 · 方向键移动</span></p>
    <button onclick={rearrange}>换个构图 <span aria-hidden="true">↻</span></button>
  </div>
</div>

<style lang="scss">
  .geometry-playground { width: 100%; }
  .geometry-board { position: relative; width: 100%; aspect-ratio: 1; background-color: #dce8e3; background-image: radial-gradient(#28282435 1px, transparent 1px); background-size: 24px 24px; }
  .board-axis { position: absolute; background: #28282425; pointer-events: none; }
  .axis-x { width: 100%; height: 1px; top: 50%; }
  .axis-y { height: 100%; width: 1px; left: 50%; }
  .geometry-piece { position: absolute; width: 32%; aspect-ratio: 1; left: var(--x); top: var(--y); padding: 0; background: transparent; transform: rotate(var(--angle)); cursor: grab; touch-action: none; transition: transform .3s, filter .2s; }
  .geometry-piece:hover { filter: drop-shadow(7px 9px 0 #25252025); }
  .geometry-piece.dragging { z-index: 10; cursor: grabbing; transition: none; filter: drop-shadow(10px 12px 0 #25252030); }
  .geometry-piece span { display: block; width: 100%; height: 100%; pointer-events: none; }
  .disc span { border-radius: 50%; background: #e44030; }
  .disc span::after { content: ''; display: block; width: 50%; height: 100%; border-radius: 100% 0 0 100% / 50% 0 0 50%; background: #bb3025; }
  .arch span { border-radius: 50% 50% 0 0; background: #244ad5; }
  .arch span::after { content: ''; position: absolute; inset: 38% 30% 0; border-radius: 50% 50% 0 0; background: #dce8e3; }
  .triangle span { background: #f1c731; clip-path: polygon(50% 0, 100% 100%, 0 100%); }
  .stripes span { background: repeating-linear-gradient(0deg, #262622 0 8px, transparent 8px 18px); }
  .ring span { border: 22px solid #262622; border-radius: 50%; }
  .board-caption { position: absolute; bottom: -12px; left: 2%; font-size: 12px; transform: rotate(-90deg); transform-origin: left; pointer-events: none; }
  .playground-controls { display: flex; justify-content: space-between; align-items: center; gap: 12px; margin: 25px 0 0; font-size: 13px; }
  .playground-controls p span { display: block; margin-top: 6px; color: #68675f; font-size: 11px; }
  .playground-controls button { padding: 10px 16px; border: 1px solid currentColor; background: #f1c731; box-shadow: 3px 3px 0 #262622; }
  .playground-controls button:active { transform: translate(3px, 3px); box-shadow: none; }
  .playground-controls button span { display: inline-block; margin-left: 12px; font-size: 21px; vertical-align: middle; }
  @media (max-width: 600px) { .ring span { border-width: 16px; } }
  @media (prefers-reduced-motion: reduce) { .geometry-piece { transition: none; } }
</style>
