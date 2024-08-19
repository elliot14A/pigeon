<!-- src/lib/components/request/HeadersTab.svelte -->
<script lang="ts">
  import KeyValueInput from "$lib/components/shared/KeyValueInput.svelte";
  import { requestHeaders } from "$lib/stores/request.svelte";

  function addHeader() {
    requestHeaders.addHeaders();
  }

  function removeHeader(index: number) {
    requestHeaders.removeHeader(index);
  }
</script>

<KeyValueInput
  items={requestHeaders.headers}
  onAdd={addHeader}
  onRemove={removeHeader}
  keyPlaceholder="Header"
  valuePlaceholder="Value"
/>
