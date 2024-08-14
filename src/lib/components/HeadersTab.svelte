<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  let headers = [{ key: "", value: "" }];

  function addHeader() {
    headers = [...headers, { key: "", value: "" }];
  }

  function removeHeader(index: number) {
    headers = headers.filter((_, i) => i !== index);
  }
</script>

<div class="space-y-2">
  {#each headers as header, i}
    <div class="flex space-x-2">
      <Input placeholder="Header" bind:value={header.key} />
      <Input placeholder="Value" bind:value={header.value} />
      <Button variant="destructive" on:click={() => removeHeader(i)}
        >Remove</Button
      >
    </div>
  {/each}
  <Button on:click={addHeader}>Add Header</Button>
</div>
