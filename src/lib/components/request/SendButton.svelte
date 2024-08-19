<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import {
    requestBody,
    requestParams,
    requestHeaders,
    requestUrl,
    requestMethod,
  } from "$lib/stores/request.svelte";

  $inspect(requestMethod.method);

  async function sendRequest() {
    // This function should be implemented in Tauri
    console.log("Sending request:", {
      method: requestMethod.method,
      url: requestUrl.url,
      queryParams: requestParams.params,
      headers: requestHeaders.headers,
      body: requestBody.body,
    });
    // TODO: Implement the actual request sending logic in Tauri
  }
</script>

<Button on:click={sendRequest}>Send</Button>
