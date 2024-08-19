<script lang="ts">
  import { Textarea } from "$lib/components/ui/textarea";
  import { requestBody } from "$lib/stores/request.svelte";

  function updateBody(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    requestBody.body = target.value;
  }
</script>

<div class="space-y-2">
  <Textarea
    placeholder="Enter request body"
    value={requestBody.body}
    on:input={updateBody}
  />
</div>
