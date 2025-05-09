<template>
  <div>
    <div>
      <v-file-input label="load" variant="outlined"></v-file-input>
      <p v-if="loadError" style="color: red;">{{ loadError }}</p>
    </div>

    <div class="d-flex flex-row">
      <p style="width: 120px; text-align: right; margin-right: 10px;">Name:</p>
      <div>{{ count }}</div>
    </div>
    <div class="d-flex flex-row">
      <p style="width: 120px; text-align: right; margin-right: 10px;">use postman:</p>
      <div>{{ count }}</div>
    </div>
    <div class="d-flex flex-row">
      <p style="width: 120px; text-align: right; margin-right: 10px;">use postman env:</p>
      <div>{{ count }}</div>
    </div>

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

<script>
import { defineComponent, ref } from 'vue';
import InputField from '../components/InputField.vue';
import ResultTable from '../components/ResultTable.vue';
import { VFileUpload } from 'vuetify/labs/VFileUpload'

export default defineComponent({
  name: 'HomeView',
  components: {
    InputField,
    ResultTable,
    VFileUpload,
  },
  setup() {
    const filePath = ref('');
    const testName = ref('');
    const postmanName = ref('');
    const environmentName = ref('');
    const status = ref('Wait');
    const testResults = ref([]);
    const loadError = ref('');

    const loadFile = () => {
      // TauriのAPIを呼び出してファイル選択ダイアログを表示し、
      // 選択されたファイルのパスをfilePathに設定する処理を記述します。
      // また、選択されたymlファイルの内容を読み込み、
      // testName, postmanName, environmentName に値を設定する処理もここで行います。
      // エラーが発生した場合は、loadErrorにエラーメッセージを設定します。
      console.log('Load button clicked');
      // 例:
      // window.__TAURI__.dialog.open({
      //   filters: [{
      //     name: 'YAML',
      //     extensions: ['yml', 'yaml']
      //   }]
      // }).then(selectedPath => {
      //   if (selectedPath) {
      //     filePath.value = selectedPath;
      //     // ここでymlファイルを読み込む処理 (Tauriのfs APIなどを使用)
      //     // ...
      //     // 例として固定値を設定
      //     testName.value = 'Sample Test';
      //     postmanName.value = 'collection.json';
      //     environmentName.value = 'environment.json';
      //     loadError.value = '';
      //   }
      // }).catch(error => {
      //   console.error('ファイル選択エラー:', error);
      //   loadError.value = 'ファイルの読み込みに失敗しました。';
      // });
    };

    const runTest = () => {
      if (filePath.value) {
        status.value = 'Running';
        // Tauriのコマンドを呼び出してバックエンドのテスト実行処理を開始します。
        // バックエンドからテスト結果が送信される仕組みを実装する必要があります。
        console.log('Run button clicked');
        // 例:
        // window.__TAURI__.invoke('run_tests', { filePath: filePath.value })
        //   .then(response => {
        //     console.log('テスト開始:', response);
        //     // テスト結果の更新は、バックエンドからのイベントリスナーで行う想定
        //   })
        //   .catch(error => {
        //     console.error('テスト実行エラー:', error);
        //     status.value = 'NG'; // エラー時のステータス
        //   });
        // // モックのテスト結果を定期的に追加 (実際はバックエンドからのイベント駆動)
        setTimeout(() => {
          testResults.value.push({ No: 1, Item: 'Test Case 1', Result: 'OK' });
          setTimeout(() => {
            testResults.value.push({ No: 2, Item: 'Test Case 2', Result: 'NG' });
            status.value = 'OK'; // モックの完了
          }, 2000);
          status.value = 'Running';
        }, 1000);
      } else {
        alert('先にファイルを読み込んでください。');
      }
    };

    const stopTest = () => {
      if (status.value === 'Running') {
        status.value = 'Wait';
        // Tauriのコマンドを呼び出してバックエンドのテスト実行処理を停止します。
        console.log('Stop button clicked');
        // 例:
        // window.__TAURI__.invoke('stop_tests')
        //   .then(response => {
        //     console.log('テスト停止:', response);
        //   })
        //   .catch(error => {
        //     console.error('テスト停止エラー:', error);
        //   });
      }
    };

    return {
      filePath,
      testName,
      postmanName,
      environmentName,
      status,
      testResults,
      loadError,
      loadFile,
      runTest,
      stopTest,
    };
  },
});
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