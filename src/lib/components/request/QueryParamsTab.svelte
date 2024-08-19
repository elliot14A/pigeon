<script lang="ts">
    import KeyValueInput from "$lib/components/shared/KeyValueInput.svelte";
    import { requestParams } from "$lib/stores/request.svelte";

    function addParam() {
        requestParams.addParams();
    }

    function removeParam(index: number) {
        requestParams.removeParams(index);
    }
</script>

<KeyValueInput
    items={requestParams.params}
    onAdd={addParam}
    onRemove={removeParam}
    keyPlaceholder="Parameter"
/>
