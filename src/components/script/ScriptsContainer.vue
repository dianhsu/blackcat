<template>
    <div style="margin-bottom: 10px;">
        <el-button type="default" @click="addScriptBlock">Add Snippet</el-button>
    </div>
    <div class="demo-collapse" v-if="snippets.length > 0">
        <el-collapse v-model="activeNames" @change="handleChange">
            <el-collapse-item :name="snippet.name" v-for="snippet in snippets" :title="snippet.title">
              <ScriptBlock/>
            </el-collapse-item>
        </el-collapse>
    </div>
    <div v-else>
        <el-empty :image-size="200" />
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
interface Snippet {
    name: string
    title: string
    env: string
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
        env: 'bash',
        content: 'echo "Hello, World!"'
    })
}

</script>