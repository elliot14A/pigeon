<script lang="ts">
  import * as Select from "$lib/components/ui/select";
  const methods = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
</script>

<Select.Root portal={null}>
  <Select.Trigger class="w-[100px]">
    <Select.Value placeholder="Method" />
  </Select.Trigger>
  <Select.Content>
    {#each methods as method}
      <Select.Item value={method}>{method}</Select.Item>
    {/each}
  </Select.Content>
</Select.Root>
