<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { request } from "$lib/stores/request.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  $inspect(request);

  async function logToBackend(message: string) {
    await invoke("log_message", { message });
  }

  async function sendRequest() {
    const req = {
      method: request.method,
      url: request.url,
      params: request.params,
      headers: request.headers,
      body: request.body,
    };

    try {
      const response = await invoke("send_request", { request: req });
      console.log("Response:", response);
      await logToBackend("Request logged successfully");
    } catch (err) {
      console.error("Error logging request:", err);
      await logToBackend(`Failed to log request: ${err}`);
    }
  }
</script>

<Button on:click={sendRequest}>Send</Button>
