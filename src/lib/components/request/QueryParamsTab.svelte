<script lang="ts">
    import KeyValueInput from "$lib/components/shared/KeyValueInput.svelte";
    import { request } from "$lib/stores/request.svelte";

    function addParam() {
        request.addParams();
    }

    function removeParam(index: number) {
        request.removeParams(index);
    }
</script>

<KeyValueInput
    items={request.params}
    onAdd={addParam}
    onRemove={removeParam}
    keyPlaceholder="Parameter"
/>
