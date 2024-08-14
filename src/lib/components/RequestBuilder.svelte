<!-- src/lib/components/RequestBuilder.svelte -->
<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import QueryParamsTab from "./QueryParamsTab.svelte";
  import HeadersTab from "./HeadersTab.svelte";
  import RequestBodyTab from "./RequestBodyTab.svelte";

  let activeTab = "params";

  const tabs = [
    { id: "params", label: "Params" },
    { id: "headers", label: "Headers" },
    { id: "body", label: "Body" },
  ];
</script>

<div class="border rounded-md">
  <div class="flex border-b">
    {#each tabs as tab}
      <Button
        variant={activeTab === tab.id ? "default" : "ghost"}
        class="rounded"
        on:click={() => (activeTab = tab.id)}
      >
        {tab.label}
      </Button>
    {/each}
  </div>

  <div class="p-4">
    {#if activeTab === "params"}
      <QueryParamsTab />
    {:else if activeTab === "headers"}
      <HeadersTab />
    {:else if activeTab === "body"}
      <RequestBodyTab />
    {/if}
  </div>
</div>
