<script setup>
import { ref, onMounted } from 'vue';
import { tauri } from '@tauri-apps/api';

const boardRef = ref([]);

onMounted(async () => {
  let result = await tauri.invoke('init_game', {
    'ai_1': null,
    'ai_2': {
     level: 1
    },
  });

  boardRef.value = result.board;
});

const clickCell = async (x, y) => {
  let result = await tauri.invoke('click', {x, y});
  if (result) {
    boardRef.value = result.board;
  }
  waitAi();
};

const waitAi = async () => {
  let result = await tauri.invoke('wait_ai', {x, y});
  if (result) {
    boardRef.value = result.board;
  }
};

</script>

<template>
  <div class="othello-board">
    <div class="row" v-for="(row, y) in boardRef">
      <div class="cell" v-for="(stone, x) in row" @click="clickCell(x, y)">
        <div class="star" v-if="(x === 1 && y === 1) || (x === 1 && y === 5) || (x === 5 && y === 1) || (x === 5 && y === 5)"></div>
        <div v-if="stone === 1" class="stone black"></div>
        <div v-if="stone === 2" class="stone white"></div>
      </div>
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
        bottom: -4px; right: -4px;
        background-color: $boarder-color;
        border-radius: 50%;
        width: 8px;
        height: 8px;
      }
    }
  }

</style>
