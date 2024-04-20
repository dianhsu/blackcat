<template>
  <div class="demo-collapse" v-if="snippets.length > 0">
    <el-collapse v-model="activeNames" @change="handleChange">
      <el-collapse-item :name="snippet.name" v-for="snippet in snippets" :title="snippet.title">
        <el-form :model="snippet" label-width="auto" style="max-width: 800px">
          <el-form-item label="Script Title">
            <el-input v-model="snippet.title" />
          </el-form-item>
          <el-form-item label="Script Type">
            <el-switch v-model="snippet.remote" active-text="remote" inactive-text="local" inline-prompt />
          </el-form-item>
          <el-form-item label="Script Shell">
            <el-input v-model="snippet.shell" placeholder="/bin/bash, CMD or powershell" />
          </el-form-item>
          <el-form-item label="Script Content">
            <el-input v-model="snippet.script" type="textarea" />
          </el-form-item>
        </el-form>
      </el-collapse-item>
    </el-collapse>
  </div>
  <div style="margin-bottom: 10px;">
    <el-button type="default" @click="addScriptBlock">Add</el-button>
    <el-button type="primary" @click="executeScript">Execute</el-button>
  </div>
  <div v-if="snippets.length == 0">
    <el-empty :image-size="200" />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

interface Snippet {
  name: string
  title: string
  remote: boolean
  shell: string
  script: string
  content: string
}
const activeNames = ref<string[]>([])
const snippets = ref<Snippet[]>([])

const handleChange = (val: string[]) => {
  console.log(val)
}

const addScriptBlock = () => {
  snippets.value.push({
    name: `${snippets.value.length + 1}`,
    title: `Snippet ${snippets.value.length + 1}`,
    shell: '',
    remote: true,
    script: '',
    content: 'echo "Hello, World!"'
  })
}
const executeScript = () => {
  console.log(snippets.value)
  console.log('Execute script')
}

</script>