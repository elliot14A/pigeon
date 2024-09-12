<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { request } from "$lib/stores/request.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { info } from "tauri-plugin-log-api";

  $inspect(request);

  async function sendRequest() {
    const req = {
      method: request.method,
      url: request.url,
      headers: {},
      params: [],
      body: null,
    };

    try {
      const response: any = await invoke("send_request", {
        request: req,
      });
      info(`response ${response.status.code}`);
    } catch (err) {
      info(`error: ${err}`);
    }
  }
</script>

<Button on:click={sendRequest}>Send</Button>
