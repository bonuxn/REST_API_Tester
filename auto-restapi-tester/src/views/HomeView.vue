<template>
  <div>
    <div>
        <v-file-input label="load" variant="outlined" @change="getFileName" ></v-file-input>
      <p>選択されたファイル: </p><p v-if="testName">{{ testName }}</p>
      <p v-if="loadError" style="color: red;">{{ loadError }}</p>
    </div>

<!--    <div class="d-flex flex-row">-->
<!--      <p style="width: 120px; text-align: right; margin-right: 10px;">Name:</p>-->
<!--      <div>{{ files ? files[0].name : 'なし' }}</div>-->
<!--    </div>-->
<!--    <div class="d-flex flex-row">-->
<!--      <p style="width: 120px; text-align: right; margin-right: 10px;">use postman:</p>-->
<!--      <div>{{ files ? (files[0].size / 1024 / 1024).toFixed(2) : 'なし' }}MB</div>-->
<!--    </div>-->
<!--    <div class="d-flex flex-row">-->
<!--      <p style="width: 120px; text-align: right; margin-right: 10px;">use postman env:</p>-->
<!--      <div>{{ files ? files[0].type : 'なし' }}</div>-->
<!--    </div>-->

    <div style="margin-top: 20px;" class="d-flex flex-row">
      <label>ステータス</label>
      <div style="border: 1px solid #ccc; padding: 8px; width: 100px; text-align: center;">{{ status }}</div>
    </div>

    <div style="margin-top: 20px;">
      <button @click="runTest" :disabled="status === 'Running'">Run</button>
      <button @click="stopTest" :disabled="status !== 'Running'">Stop</button>
    </div>

    <result-table :results="testResults" />
  </div>
</template>

<script setup>
  import { ref } from 'vue';
  import ResultTable from '../components/ResultTable.vue';
  import { VFileUpload } from 'vuetify/labs/VFileUpload';
  // import * as jsyaml from 'js-yaml'; // 必要に応じてインポート

  const filePath = ref('');
  const testName = ref('ddd');
  const postmanName = ref('');
  const environmentName = ref('');
  const status = ref('Wait');
  const testResults = ref([]);
  const loadError = ref('');
  const selectedFiles = ref([]);

  const loadFile = async () => {
    // console.log('Load button clicked');
    // try {
    //   const selectedPath = await window.__TAURI__.dialog.open({
    //     filters: [{
    //       name: 'YAML',
    //       extensions: ['yml', 'yaml']
    //     }]
    //   });
    //
    //   if (selectedPath) {
    //     filePath.value = selectedPath;
    //     try {
    //       const contents = await window.__TAURI__.fs.readTextFile(selectedPath);
    //       const parsedYAML = jsyaml.load(contents);
    //       testName.value = parsedYAML.testName || '';
    //       postmanName.value = parsedYAML.postman || '';
    //       environmentName.value = parsedYAML.environment || '';
    //       loadError.value = '';
    //     } catch (error) {
    //       console.error('YAMLファイルの解析エラー:', error);
    //       loadError.value = 'YAMLファイルの解析に失敗しました。';
    //     }
    //   }
    // } catch (error) {
    //   console.error('ファイル選択エラー:', error);
    //   loadError.value = 'ファイルの読み込みに失敗しました。';
    // }
  };

  // const runTest = async () => {
  //   if (filePath.value) {
  //     status.value = 'Running';
  //     console.log('Run button clicked');
  //     try {
  //       const response = await window.__TAURI__.invoke('run_tests', { filePath: filePath.value });
  //       console.log('テスト開始:', response);
  //       // テスト結果の更新は、バックエンドからのイベントリスナーで行う想定
  //       // モックのテスト結果を定期的に追加 (実際はバックエンドからのイベント駆動)
  //       setTimeout(() => {
  //         testResults.value.push({ No: 1, Item: 'Test Case 1', Result: 'OK' });
  //         setTimeout(() => {
  //           testResults.value.push({ No: 2, Item: 'Test Case 2', Result: 'NG' });
  //           status.value = 'OK'; // モックの完了
  //         }, 2000);
  //         status.value = 'Running';
  //       }, 1000);
  //     } catch (error) {
  //       console.error('テスト実行エラー:', error);
  //       status.value = 'NG'; // エラー時のステータス
  //     }
  //   } else {
  //     alert('先にファイルを読み込んでください。');
  //   }
  // };
  //
  // const stopTest = async () => {
  //   if (status.value === 'Running') {
  //     status.value = 'Wait';
  //     console.log('Stop button clicked');
  //     try {
  //       const response = await window.__TAURI__.invoke('stop_tests');
  //       console.log('テスト停止:', response);
  //     } catch (error) {
  //       console.error('テスト停止エラー:', error);
  //     }
  //   }
  // };

  const getFileName = (files) => {
    if (files && files.length > 0) {
      console.log("fileが選択されました。");
      console.log(files[0].name);
      testName.value = files[0].name;
      // filePath の更新は loadFile で行う想定なので、ここでは filePath は更新しません。
    } else {
      console.log("fileが選択されてない。");
      console.log(files[0].name);
      testName.value = ''; // ファイルが選択されなかった場合はクリア
    }
  };
</script>

<style scoped>
/* 必要に応じてスタイルを記述 */
button {
  padding: 10px 15px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  margin-right: 10px;
}

button:enabled {
  background-color: #007bff;
  color: white;
}

button:disabled {
  background-color: #ccc;
  color: #666;
  cursor: not-allowed;
}
</style>