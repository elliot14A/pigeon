<!-- src/lib/components/RequestBodyTab.svelte -->
<script lang="ts">
  import { Textarea } from "$lib/components/ui/textarea";

  let requestBody = "";
</script>

<div class="space-y-2">
  <Textarea placeholder="Enter request body" bind:value={requestBody} />
</div>
