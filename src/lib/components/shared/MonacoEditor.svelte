<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import * as monaco from "monaco-editor";
  import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
  import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
  import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
  import type { editor } from "monaco-editor";

  export let value: string;
  export let language = "json";
  export let readOnly = false;
  export let onChange: (newValue: string) => void;

  let editorInstance: monaco.editor.IStandaloneCodeEditor;
  let editorElement: HTMLElement;

  // Define One Dark theme
  const oneDarkTheme: editor.IStandaloneThemeData = {
    base: "vs-dark",
    inherit: true,
    rules: [
      { token: "comment", foreground: "5C6370", fontStyle: "italic" },
      { token: "keyword", foreground: "C678DD" },
      { token: "string", foreground: "98C379" },
      { token: "number", foreground: "D19A66" },
      { token: "operator", foreground: "56B6C2" },
    ],
    colors: {
      "editor.background": "#282C34",
      "editor.foreground": "#ABB2BF",
      "editor.lineHighlightBackground": "#2C313C",
      "editorCursor.foreground": "#528BFF",
      "editorWhitespace.foreground": "#3B4048",
    },
  };

  // Set up Monaco environment
  self.MonacoEnvironment = {
    getWorker(_, label) {
      if (label === "json") {
        return new jsonWorker();
      }
      if (label === "html" || label === "xml") {
        return new htmlWorker();
      }
      return new editorWorker();
    },
  };

  onMount(() => {
    if (!editorElement) return;

    // Register One Dark theme
    monaco.editor.defineTheme("oneDark", oneDarkTheme);

    // Set JSON language options
    monaco.languages.json.jsonDefaults.setDiagnosticsOptions({
      validate: true,
      allowComments: false,
      schemas: [],
      enableSchemaRequest: false,
      schemaRequest: "error",
      schemaValidation: "error",
      trailingCommas: "error",
    });

    // Create editor
    editorInstance = monaco.editor.create(editorElement, {
      value,
      language,
      theme: "oneDark",
      minimap: { enabled: false },
      readOnly,
      automaticLayout: true,
      scrollBeyondLastLine: false,
      lineNumbers: "on",
      roundedSelection: false,
      selectionClipboard: true,
      scrollbar: {
        useShadows: false,
        verticalHasArrows: true,
        horizontalHasArrows: true,
        vertical: "visible",
        horizontal: "visible",
      },
    });

    // Add change listener
    editorInstance.onDidChangeModelContent(() => {
      const newValue = editorInstance.getValue();
      if (newValue !== value) {
        onChange(newValue);
      }
    });

    // Focus the editor
    editorInstance.focus();
  });

  onDestroy(() => {
    if (editorInstance) {
      editorInstance.dispose();
    }
  });

  $: {
    if (editorInstance) {
      // Update value when it changes externally
      if (value !== editorInstance.getValue()) {
        editorInstance.setValue(value);
      }

      // Update language when it changes
      if (language) {
        monaco.editor.setModelLanguage(editorInstance.getModel()!, language);
      }

      // Update readOnly state
      editorInstance.updateOptions({ readOnly });
    }
  }
</script>

<div bind:this={editorElement} class="h-full w-full min-h-[200px]"></div>
