<template>
  <el-form :model="form" label-width="auto" style="max-width: 800px">
    <el-form-item label="Script Title">
      <el-input v-model="form.title" />
    </el-form-item>
    <el-form-item label="Script Type">
      <el-switch v-model="form.type" active-text="remote" inactive-text="local" inline-prompt />
    </el-form-item>
    <el-form-item label="Script Shell">
      <el-input v-model="form.shell" placeholder="/bin/bash, CMD or powershell" />
    </el-form-item>
    <el-form-item label="Script Content">
      <el-input v-model="form.script" type="textarea" />
    </el-form-item>
  </el-form>
</template>

<script lang="ts" setup>
import { reactive } from 'vue'

// do not use same name with ref
const form = reactive({
  title: "",
  type: "",
  shell: "",
  script: "",
})

</script>