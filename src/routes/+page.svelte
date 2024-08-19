<script lang="ts">
  import Header from "$lib/components/layout/Header.svelte";
  import MethodSelector from "$lib/components/request/MethodSelector.svelte";
  import UrlInput from "$lib/components/request/UrlInput.svelte";
  import SendButton from "$lib/components/request/SendButton.svelte";
  import ResponseViewer from "$lib/components/response/ResponseViewer.svelte";
  import RequestBuilder from "$lib/components/request/RequestBuilder.svelte";

  let responseBody = ""; // This should be updated when a response is received
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
    <div class="h-1/3 min-h-[400px] p-4 border-t">
      <ResponseViewer {responseBody} />
    </div>
  </main>
</div>
