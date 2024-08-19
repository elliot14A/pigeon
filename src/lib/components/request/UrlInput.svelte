<script lang="ts">
    import { Input } from "$lib/components/ui/input";
    import { requestUrl } from "@/lib/stores/request.svelte";
</script>

<Input bind:value={requestUrl.url} type="text" placeholder="Enter URL" />
