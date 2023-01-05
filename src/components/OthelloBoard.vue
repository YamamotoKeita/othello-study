<script setup>
import { ref } from "vue";
import { tauri } from '@tauri-apps/api';

const board = ref([]);

for (let i = 0; i < 8; i++) {
  let row = [];
  for (let j = 0; j < 8 ; j++) {
    row.push('');
  }
  board.value.push(row);
}

const clickCell = async (x, y) => {
  console.log(`click ${x}, ${y}`);
  await tauri.invoke('init_game', {
    'ai_1': null,
    'ai_2': {
    },
  });
};

</script>

<template>
  <div class="othello-board">
    <div class="row" v-for="(row, y) in board">
      <div class="cell" v-for="(cell, x) in row" @click="clickCell(x, y)">
        <div class="star" v-if="(x === 1 && y === 1) || (x === 1 && y === 5) || (x === 5 && y === 1) || (x === 5 && y === 5)"></div>
        <div class="stone white"></div>
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
