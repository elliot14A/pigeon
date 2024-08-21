<!-- src/lib/components/request/HeadersTab.svelte -->
<script lang="ts">
  import KeyValueInput from "$lib/components/shared/KeyValueInput.svelte";
  import { request } from "$lib/stores/request.svelte";

  function addHeader() {
    request.addHeaders();
  }

  function removeHeader(index: number) {
    request.removeHeader(index);
  }
</script>

<KeyValueInput
  items={request.headers}
  onAdd={addHeader}
  onRemove={removeHeader}
  keyPlaceholder="Header"
  valuePlaceholder="Value"
/>
