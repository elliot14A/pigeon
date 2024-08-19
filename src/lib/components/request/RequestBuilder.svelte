<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import QueryParamsTab from "$lib/components/request/QueryParamsTab.svelte";
  import HeadersTab from "$lib/components/request/HeadersTab.svelte";
  import RequestBodyTab from "$lib/components/request/RequestBodyTab.svelte";

  let activeTab = "params";

  const tabs = [
    { id: "params", label: "Params" },
    { id: "headers", label: "Headers" },
    { id: "body", label: "Body" },
  ];
</script>

<div class="border rounded-md flex flex-col h-full">
  <div class="flex border-b">
    {#each tabs as tab}
      <Button
        variant={activeTab === tab.id ? "default" : "ghost"}
        class="rounded-none"
        on:click={() => (activeTab = tab.id)}
      >
        {tab.label}
      </Button>
    {/each}
  </div>

  <div class="p-4 flex-grow h-[300px]">
    {#if activeTab === "params"}
      <QueryParamsTab />
    {:else if activeTab === "headers"}
      <HeadersTab />
    {:else if activeTab === "body"}
      <RequestBodyTab />
    {/if}
  </div>
</div>
