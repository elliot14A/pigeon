<script lang="ts">
  import { Textarea } from "$lib/components/ui/textarea";

  export let responseBody = "";
</script>

<div class="flex flex-col h-full">
  <h2 class="text-lg font-semibold mb-2">Response Body</h2>
  <Textarea class="flex-grow resize-none" readonly value={responseBody} />
</div>
