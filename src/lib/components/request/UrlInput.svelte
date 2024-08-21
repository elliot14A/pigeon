<script lang="ts">
    import { Input } from "$lib/components/ui/input";
    import { request } from "@/lib/stores/request.svelte";
</script>

<Input bind:value={request.url} type="text" placeholder="Enter URL" />
