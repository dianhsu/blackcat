<template>
  <el-breadcrumb separator="/">
    <el-breadcrumb-item>
      <el-link :underline="false" @click="onBreadcrumbClick">
        {{ "Root" }}
      </el-link>
    </el-breadcrumb-item>
    <el-breadcrumb-item v-for="directory in directories">
      <el-link :underline="false" @click="onBreadcrumbClick">
        {{ directory }}
      </el-link>
    </el-breadcrumb-item>
  </el-breadcrumb>
  <el-table :data="current_files" style="width: 100%; margin: 10px" @row-click="rowClick">
    <el-table-column label="Type" width="60">
      <template #default="scope">
        <div style="display: flex; align-items: center">
          <el-icon v-if="scope.row.is_dir">
            <img style="width: 50px" src="../../assets/folder.svg" alt="Folder" />
          </el-icon>
          <el-icon v-else>
            <img style="width: 50px" src="../../assets/document.svg" alt="Document" />
          </el-icon>
        </div>
      </template>
    </el-table-column>
    <el-table-column prop="name" label="Name" />
    <el-table-column label="Size" width="200">
      <template #default="scope">
        <span v-if="!scope.row.is_dir">{{ scope.row.size }}</span>
      </template>
    </el-table-column>
  </el-table>
  <el-affix style="position: fixed; bottom: 10px; right: 10px;z-index: 100">
    <el-row>
      <el-link :underline="false">
        <el-icon :size="30">
          <Refresh />
        </el-icon>
      </el-link>
    </el-row>
    <el-row>
      <el-link :underline="false">
        <el-icon :size="30">
          <Download />
        </el-icon>
      </el-link>
    </el-row>
    <el-row>
      <el-link :underline="false">
        <el-icon :size="30">
          <Upload />
        </el-icon>
      </el-link>
    </el-row>
    <el-row>
      <el-link :underline="false">
        <el-icon :size="30">
          <Edit />
        </el-icon>
      </el-link>
    </el-row>
    <el-row>
      <el-link :underline="false">
        <el-icon :size="30">
          <Delete />
        </el-icon>
      </el-link>
    </el-row>
  </el-affix>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';
interface File {
  name: string;
  path: string;
  size: string;
  is_dir: boolean;
}
const directories = ref<String[]>([])
const current_files = ref<File[]>([])
const size_units = ref(["bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"]);
const getDirectories = async () => {

  current_files.value = [];
  await invoke('list_folder', { directory: "/" + directories.value.join("/") }).then((res: any) => {
    for (let i = 0; i < res.length; i++) {
      let file = res[i];
      for (let j = 0; j < size_units.value.length; j++) {
        if (file.size < 1024) {
          file.size = (file.size == 0 ? file.size : file.size.toFixed(2)) + " " + size_units.value[j];
          break;
        }
        file.size = (file.size / 1024);
      }
      current_files.value.push(file)
    }
    current_files.value = current_files.value.sort((a: File, b: File) => {
      if (a.is_dir && !b.is_dir) {
        return -1
      } else if (!a.is_dir && b.is_dir) {
        return 1
      } else {
        return a.name.localeCompare(b.name)
      }
    })
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

const onBreadcrumbClick = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  const index = target.innerText
  if (index === "Root") {
    directories.value = []
  } else {
    directories.value = directories.value.slice(0, directories.value.indexOf(index) + 1)
  }
  getDirectories()
}
</script>
