<!-- src/routes/+page.svelte -->
<script lang="ts">
  import Header from "$lib/components/Header.svelte";
  import MethodSelector from "$lib/components/MethodSelector.svelte";
  import UrlInput from "$lib/components/UrlInput.svelte";
  import SendButton from "$lib/components/SendButton.svelte";

  import ResponseBody from "$lib/components/ResponseViewer.svelte";
  import RequestBuilder from "@/lib/components/RequestBuilder.svelte";
</script>

<div class="flex flex-col h-screen bg-background text-foreground">
  <Header />
  <main class="flex flex-col h-full">
    <div class="flex-grow overflow-auto p-4">
      <div class="flex space-x-2 mb-4">
        <MethodSelector />
        <UrlInput />
        <SendButton />
      </div>
      <div class="flex-grow">
        <RequestBuilder />
      </div>
    </div>
    <div class="h-1/3 min-h-[200px] p-4 border-t">
      <ResponseBody />
    </div>
  </main>
</div>
