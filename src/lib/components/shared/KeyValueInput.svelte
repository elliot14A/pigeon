<script lang="ts">
    import type { KeyValuePair } from "$lib/types";
    import Button from "../ui/button/button.svelte";
    import Input from "../ui/input/input.svelte";

    let {
        items,
        onAdd,
        onRemove,
        keyPlaceholder = "Key",
        valuePlaceholder = "Value",
    } = $props<{
        items: KeyValuePair[];
        onAdd: () => void;
        onRemove: (i: number) => void;
        keyPlaceholder?: string;
        valuePlaceholder?: string;
    }>();

    let size = $derived(items.length);
</script>

<div class="flex flex-col h-full">
    <div class="flex-grow overflow-y-auto mb-4">
        {#each items as item, i}
            <div class="flex space-x-2 mb-2">
                <Input placeholder={keyPlaceholder} bind:value={item.key} />
                <Input placeholder={valuePlaceholder} bind:value={item.value} />
                <Button
                    variant="destructive"
                    disabled={size == 1}
                    onclick={() => onRemove(i)}>Remove</Button
                >
            </div>
        {/each}
    </div>
    <div class="mt-auto">
        <Button onclick={onAdd}>Add</Button>
    </div>
</div>
