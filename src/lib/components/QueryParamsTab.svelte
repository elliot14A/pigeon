<!-- src/lib/components/QueryParamsTab.svelte -->
<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";

  let params = $state([{ key: "", value: "" }]);

  function addParam() {
    params = [...params, { key: "", value: "" }];
  }

  function removeParam(index: number) {
    params = params.filter((_, i) => i !== index);
  }
</script>

<div class="space-y-2">
  {#each params as param, i}
    <div class="flex space-x-2">
      <Input placeholder="Key" bind:value={param.key} />
      <Input placeholder="Value" bind:value={param.value} />
      <Button variant="destructive" on:click={() => removeParam(i)}
        >Remove</Button
      >
    </div>
  {/each}
  <Button on:click={addParam}>Add Parameter</Button>
</div>
