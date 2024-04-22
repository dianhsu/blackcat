<template>
  <div class="custom-tree-container">
    <p>Using render-content</p>
    <el-tree style="max-width: 600px" :data="dataSource" show-checkbox node-key="id" default-expand-all
      :expand-on-click-node="false" :render-content="renderContent" />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import type Node from 'element-plus/es/components/tree/src/model/node'

interface Tree {
  id: number
  label: string
  isFile: boolean
  children?: Tree[]
}
let id = 1000

const append = (data: Tree) => {
  const newChild = { id: id++, label: 'testtest', children: [], isFile: true }
  if (!data.children) {
    data.children = []
  }
  data.children.push(newChild)
  dataSource.value = [...dataSource.value]
}

const remove = (node: Node, data: Tree) => {
  const parent = node.parent
  const children: Tree[] = parent.data.children || parent.data
  const index = children.findIndex((d) => d.id === data.id)
  children.splice(index, 1)
  dataSource.value = [...dataSource.value]
}

const renderContent = (
  h: any,
  {
    node,
    data,
    // @ts-ignore
    store,
  }: {
    node: Node
    data: Tree
    store: Node['store']
  }
) => {
  if (node.data.isFile) {
    return h('span', null, node.label)
  } else {
    return h(
      'span',
      {
        class: 'custom-tree-node',
      },
      h('span', null, node.label),
      h(
        'span',
        null,
        h(
          'a',
          {
            onClick: () => append(data),
          },
          'Append'
        ),
        h(
          'a',
          {
            style: 'margin-left: 8px',
            onClick: () => remove(node, data),
          },
          'Delete'
        )
      )
    )
  }
}

const dataSource = ref<Tree[]>([
  {
    id: 1,
    label: 'Level one 1',
    isFile: false,
    children: [
    ],
  },
  {
    id: 2,
    label: 'Level one 2',
    isFile: false,
    children: [
    ],
  },
  {
    id: 3,
    label: 'Level one 3',
    isFile: false,
    children: [],
  },
])
</script>

<style>
.custom-tree-node {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  font-size: 14px;
  padding-right: 8px;
}
</style>
