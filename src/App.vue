<template>
  <div class="container">

    <div class="container-top">
      <a-space fill>
        <span>
          数量: {{ vars?.length }}
        </span>
        <a-button size="small">
          <template #icon>
          </template>
          <template #default>变量黑名单</template>
        </a-button>
        <a-button size="small" status="success">
          <template #icon>
            <icon-plus/>
          </template>
          <template #default>新增</template>
        </a-button>
      </a-space>
    </div>
    <div class="container-middle">
      <div style="margin: 10px 0;">
        <a-input-tag :default-value="['test']" :style="{width:'320px'}" placeholder="Please Enter" allow-clear/>
      </div>
      <div>
        <a-collapse :default-active-key="['2']" accordion>
          <a-collapse-item header="PATH" key="1">
            <a-space direction="vertical" fill>
              <div v-for="(row, idx) in path" :key="idx">
                <a-tag color="#00b42a">{{ row }}</a-tag>
              </div>
            </a-space>
          </a-collapse-item>
          <a-collapse-item header="ALL" key="2">
            <a-space direction="vertical" fill>
              <div v-for="(row, idx) in vars" :key="idx">
                <a-space>
                  <a-tag color="#00b42a">{{ row.k }}</a-tag>
                  <span>=</span>
                  <a-tag color="#00b42a">{{ row.v }}</a-tag>
                  <a-button size="mini">
                    <template #icon>
                      <icon-edit/>
                    </template>
                  </a-button>
                  <a-button size="mini">
                    <template #icon>
                      <icon-close/>
                    </template>
                  </a-button>
                </a-space>
              </div>
            </a-space>
          </a-collapse-item>
          <a-collapse-item header="BLACK LIST" key="3">
          </a-collapse-item>
        </a-collapse>
      </div>


      <div style="height: 10px;"></div>
    </div>
    <div class="container-bottom"></div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from "vue"
import {invoke} from "@tauri-apps/api/tauri"

class KvPair {
  k: string;
  v: string;

  constructor(k: string, v: string) {
    this.k = k;
    this.v = v;
  }
}

const vars = ref<KvPair[]>([]);
const path = ref<string[]>([]);

async function listVars() {
  let json_str = await invoke<string>("list_vars", {});
  vars.value = JSON.parse(json_str);

  path.value = extractPath(vars.value)
}


// async function listPath() {
//   path.value = await invoke("list_path", { name: name.value });
// }

function extractPath(list: KvPair[]): string[] {
  for (let row of list) {
    if ('PATH' !== row?.k.toUpperCase()) continue
    return row.v.split(';').filter(Boolean)
  }
  return []
}

onMounted(() => {
  listVars()
})
</script>

<style scoped>
.container-top {
  padding: 10px 20px;
}

.container-middle {
  padding: 10px 20px;
}

.container-bottom {
  padding: 10px 20px;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
