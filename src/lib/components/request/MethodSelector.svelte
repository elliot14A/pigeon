<script lang="ts">
  import * as Select from "$lib/components/ui/select";
  import { HTTP_METHODS } from "$lib/constants";
  import { request} from "$lib/stores/request.svelte";

  $inspect(request)

</script>

<Select.Root
  onSelectedChange={(v) => request.method = v?.value as string }
  portal={null}
>
  <Select.Trigger class="w-[100px]">
    <Select.Value placeholder="Method" />
  </Select.Trigger>
  <Select.Content>
    {#each HTTP_METHODS as httpMethod}
      <Select.Item value={httpMethod}>{httpMethod}</Select.Item>
    {/each}
  </Select.Content>
</Select.Root>
