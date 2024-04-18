<template>
    <div>
        <el-button type="primary" @click="addScriptBlock">Add Script Block</el-button>
    </div>
    <div class="demo-collapse" v-if="scriptBlocks.length > 0">
        <el-collapse v-model="activeNames" @change="handleChange">
            <el-collapse-item :name="scriptBlock.name" v-for="scriptBlock in scriptBlocks" :title="scriptBlock.title">
            </el-collapse-item>
        </el-collapse>
    </div>
    <div v-else>
        <el-empty :image-size="200" />
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
interface ScriptBlock {
    name: string
    title: string
    env: string
    content: string
}
const activeNames = ref<string[]>([])
const scriptBlocks = ref<ScriptBlock[]>([])
const handleChange = (val: string[]) => {
    console.log(val)
}
const addScriptBlock = () => {
    scriptBlocks.value.push({
        name: `${scriptBlocks.value.length + 1}`,
        title: `Script Snippet ${scriptBlocks.value.length + 1}`,
        env: 'bash',
        content: 'echo "Hello, World!"'
    })
}
</script>