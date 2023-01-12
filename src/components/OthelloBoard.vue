<script setup>
import {ref, onMounted, computed} from 'vue';
import {tauri} from '@tauri-apps/api';

const boardRef = ref([]);
const playerRef = ref(0);
const candidatesRef = ref([]);

const turnRef = computed(() => {
  if (boardRef.value.length === 0) {
    return undefined;
  }

  let stoneCount = 0;
  for (let y = 0; y < 8; y++) {
    for (let x = 0; x < 8; x++) {
      const stone = boardRef.value[y][x];
      if (stone === 1 || stone === 2) {
        stoneCount++;
      }
    }
  }
  return stoneCount - 3;
});

onMounted(async () => {
  let result = await tauri.invoke('init_game', {
    'aiConfig1': {
      is_ai: false,
      level: 0,
    },
    'aiConfig2': {
      is_ai: true,
      level: 1,
    },
  });

  boardRef.value = result.board;
  playerRef.value = result.next_player;
  candidatesRef.value = result.next_candidates;
});

const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));

const clickCell = async (x, y) => {
  let result = await tauri.invoke('click', {x, y});
  if (result) {
    await updateBoard(result);
    // TODO Refactor this
    setTimeout(() => {
      waitAi();
    }, 0);
  }
};

const updateBoard = async (result) => {
  const point = result.placed_point;
  boardRef.value[point.y][point.x] = result.placed_stone;
  await sleep(100);

  const reversing = result.reversed_lines.map(points => {
    return reversePoints(boardRef.value, points, result.placed_stone);
  });

  await Promise.all(reversing);

  playerRef.value = result.next_player;
  candidatesRef.value = result.next_candidates;
};

const reversePoints = async (board, points, stone) => {
  for (const point of points) {
    board[point.y][point.x] = stone;
    await sleep(150);
  }
};

const waitAi = async () => {
  const player = playerRef.value;
  const result = await tauri.invoke('wait_ai', {});
  if (result) {
    await updateBoard(result);
    if (result.next_player === player) {
      waitAi().then();
    }
  }
};

const isCandidate = (x, y) => {
  return candidatesRef.value.find(p => p.x === x && p.y === y) !== undefined;
};

</script>

<template>
  <div class="othello-board">
    <div class="row" v-for="(row, y) in boardRef">
      <div class="cell" v-for="(stone, x) in row" @click="clickCell(x, y)" :class="{'candidate': isCandidate(x, y)}">
        <div class="star" v-if="(x === 1 && y === 1) || (x === 1 && y === 5) || (x === 5 && y === 1) || (x === 5 && y === 5)"></div>
        <div v-if="stone === 1 || stone === 2" class="stone"
             :class="{'black': stone === 1, 'white': stone === 2}"></div>
      </div>
    </div>
  </div>

  <div class="info-row">
    <div class="turn-info">
      <span>Turn {{turnRef}}</span>
      <div class="next-player" :class="{'black': playerRef === 1, 'white': playerRef === 2}"></div>
    </div>
    <div class="stone-info">
      <span></span>
      <span></span>
    </div>
  </div>
</template>

<style scoped lang="scss">
$board-size: 400px;
$board-color: #7bccc5;
$boarder-color: gray;

.othello-board {
  background-color: $board-color;
  display: flex;
  flex-direction: column;
  width: $board-size;
  height: $board-size;

  .row {
    display: flex;
    flex: 1;

    &:not(:last-child) {
      border-bottom: solid 1px $boarder-color;
    }
  }

  .cell {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;

    &:not(:last-child) {
      border-right: solid 1px $boarder-color;
    }

    &.candidate {
      background-color: #66afa8;
    }

    .stone {
      position: relative;
      width: 80%;
      height: 80%;

      &:before, &:after {
        position: absolute;
        transition: transform 0.5s 0s ease;
        backface-visibility: hidden;
        display: block;
        content: ' ';
        border-radius: 50%;
        width: 100%;
        height: 100%;
      }

      &:before {
        background-color: black;
      }

      &:after {
        background-color: white;
      }

      &.black {
        &:before {
          transform: rotateX(0deg);
        }

        &:after {
          transform: rotateX(180deg);
        }
      }

      &.white {
        &:before {
          transform: rotateX(180deg);
        }

        &:after {
          transform: rotateX(0deg);
        }
      }
    }

    .star {
      position: absolute;
      z-index: 2;
      bottom: -4px;
      right: -4px;
      background-color: $boarder-color;
      border-radius: 50%;
      width: 8px;
      height: 8px;
    }
  }
}

.info-row {
  margin-top: 5px;
  width: $board-size;
  display: flex;

  .turn-info {
    display: flex;
    align-items: center;

    .next-player {
      width: 30px;
      height: 30px;
      margin-left: 5px;
      border-radius: 50%;

      &.black {
        background-color: black;
      }

      &.white {
        background-color: white;
      }
    }
  }

  .stone-info {
    margin-left: auto;
  }
}
</style>
