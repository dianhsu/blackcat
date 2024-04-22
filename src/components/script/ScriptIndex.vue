<template>
  <el-tabs v-model="editableTabsValue" type="card" class="demo-tabs" editable @edit="handleTabsEdit">
    <el-tab-pane v-for="item in editableTabs" :key="item.name" :label="item.title" :name="item.name">
      <div>
        <el-input v-model="item.title" />
      </div>
      <el-divider />
      <ScriptsContainer />
    </el-tab-pane>
  </el-tabs>
</template>
<script lang="ts" setup>
import { ref } from 'vue'
import type { TabPaneName } from 'element-plus'
import ScriptsContainer from './ScriptsContainer.vue';

interface Tab {
  title: string;
  name: string;
}
let tabIndex = 0
const editableTabsValue = ref('2')
const editableTabs = ref<Tab[]>([])

const handleTabsEdit = (
  targetName: TabPaneName | undefined,
  action: 'remove' | 'add'
) => {
  if (action === 'add') {
    const newTabName = `${++tabIndex}`
    editableTabs.value.push({
      title: 'New Tab',
      name: newTabName,
    })
    editableTabsValue.value = newTabName
  } else if (action === 'remove') {
    const tabs = editableTabs.value
    let activeName = editableTabsValue.value
    if (activeName === targetName) {
      tabs.forEach((tab, index) => {
        if (tab.name === targetName) {
          const nextTab = tabs[index + 1] || tabs[index - 1]
          if (nextTab) {
            activeName = nextTab.name
          }
        }
      })
    }

    editableTabsValue.value = activeName
    editableTabs.value = tabs.filter((tab) => tab.name !== targetName)
  }
}
</script>
<style>
.demo-tabs>.el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}
</style>