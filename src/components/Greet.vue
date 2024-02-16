<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

class KvPair {
  k: string;
  v: string;
  constructor(k: string, v: string) {
    this.k = k;
    this.v = v;
  }
}

// const greetMsg = ref("");
// const name = ref("");
const vars = ref<KvPair[]>([]);
// const path = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

async function listVars() {
  let json_str = await invoke<string>("list_vars", {});
  vars.value = JSON.parse(json_str);
}


// async function listPath() {
//   path.value = await invoke("list_path", { name: name.value });
// }

onMounted(()=>{
  listVars()
}) 
</script>

<template>
  <div>
    所有环境变量按字母排序：
    <div v-for="(item, idx) in vars" :key="idx" style="text-align: left;">
      {{item.k}}={{item.v}}
    </div>
  </div>
</template>
