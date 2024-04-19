<template>
  <el-menu class="el-menu-demo" mode="horizontal" :ellipsis="false" :router="true" :default-active="activeName">
    <el-menu-item index="/">
      <el-icon>
        <img style="width: 50px" src="../assets/cat.svg" alt="BlackCat logo" />
      </el-icon>
    </el-menu-item>
    <div class="flex-grow" />
    <el-menu-item index="/file">File</el-menu-item>
    <el-menu-item index="/process">Process</el-menu-item>
    <el-menu-item index="/terminal">Terminal</el-menu-item>
    <el-menu-item index="/script">Script</el-menu-item>
    <el-menu-item index="/utility">Utility</el-menu-item>
    <el-menu-item index="/about">About</el-menu-item>
  </el-menu>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
const activeName = ref("/");
</script>

<style>
.flex-grow {
  flex-grow: 1;
}
</style>