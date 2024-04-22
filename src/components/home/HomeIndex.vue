<template>
  <div>
    <el-space fill wrap :fill-ratio="fillRatio" :direction="direction" style="width: 100%">
      <el-card class="box-card" shadow="hover">
        <template #header>
          <div class="card-header">
            <span>Host</span>
          </div>
        </template>
        <div class="text item">localhost</div>
      </el-card>
      <el-card class="box-card" shadow="hover">
        <template #header>
          <div class="card-header">
            <span>User</span>
          </div>
        </template>
        <div class="text item">Administrator</div>
      </el-card>
      <el-card class="box-card" shadow="hover">
        <template #header>
          <div class="card-header">
            <span>OS</span>
          </div>
        </template>
        <div class="text item">Windows 11</div>
      </el-card>
      <el-card class="box-card" shadow="hover">
        <template #header>
          <div class="card-header">
            <span>Architecture</span>
          </div>
        </template>
        <div class="text item">AMD 64</div>
      </el-card>
      <el-card class="box-card" shadow="hover">
        <template #header>
          <div class="card-header">
            <span>Memory</span>
          </div>
        </template>
        <div class="text item">16 GB</div>
      </el-card>
      <el-card class="box-card" shadow="hover">
        <template #header>
          <div class="card-header">
            <span>Disk</span>
          </div>
        </template>
        <div class="text item">1.82 TB</div>
      </el-card>
    </el-space>

    <!--
        <el-divider />
        <el-button>Open in external terminal</el-button>
        <el-button>Open in vscode</el-button>
        -->
  </div>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue'

const direction = ref('horizontal')
const fillRatio = ref(30)
async function getOsType() {
    return await invoke("get_os_type");
}
onMounted(() => {
  console.log('mounted')
  getOsType().then((osType) => {
    console.log(osType)
  })
})
</script>

<style scoped>
.card-header {
  font-weight: bold;
}
</style>