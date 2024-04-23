<template>
  <el-breadcrumb separator="/">
    <el-breadcrumb-item v-for="directory in directories">
      <div>
        {{ directory }}
      </div>
    </el-breadcrumb-item>
    <el-breadcrumb v-if="directories.length == 0">
      {{ "/" }}
    </el-breadcrumb>
  </el-breadcrumb>
  <el-table :data="current_files" style="width: 100%" @row-click="rowClick">
    <el-table-column label="Type" width="180">
      <template #default="scope">
        <div style="display: flex; align-items: center">
          <el-icon v-if="scope.row.is_dir">
            <Folder />Folder
          </el-icon>
          <el-icon v-else>
            <Document />Doc
          </el-icon>
        </div>
      </template>
    </el-table-column>
    <el-table-column prop="name" label="Name" width="180" />
    <el-table-column prop="size" label="Size" width="180" />
    <el-table-column prop="is_dir" label="IsDirectory" />
  </el-table>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';
interface File {
  name: string;
  path: string;
  size: number;
  is_dir: boolean;
}
const directories = ref<String[]>([])
const current_files = ref<File[]>([])
const getDirectories = async () => {
  await invoke('list_folder', { directory: "/" + directories.value.join("/") }).then((res: any) => {
    current_files.value = [];
    for (let i = 0; i < res.length; i++) {
      current_files.value.push(res[i])
    }
  })
}

const rowClick = (row: any) => {
  if (row.is_dir) {
    directories.value.push(row.name)
    getDirectories()
  }
}
onMounted(() => {
  getDirectories()
})

</script>
