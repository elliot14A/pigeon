<script lang="ts">
  import * as Select from "$lib/components/ui/select";
  import MonacoEditor from "$lib/components/shared/MonacoEditor.svelte";
  import { request } from "$lib/stores/request.svelte";
  import type { Selected } from "bits-ui";

  const bodyTypes = [
    { value: "json", label: "JSON" },
    { value: "text", label: "Text" },
    { value: "xml", label: "XML" },
    { value: "html", label: "HTML" },
  ];

  let selectedBodyType = $state(bodyTypes[0]);

  function updateBody(newValue: string) {
    request.body = newValue;
  }

  function updateBodyType(event: Selected<String> | undefined) {
    if (event === undefined) return;
    selectedBodyType =
      bodyTypes.find((type) => type.value === event.value) || bodyTypes[0];
  }
</script>

<div class="space-y-2 flex flex-col h-full">
  <div class="mb-2">
    <Select.Root onSelectedChange={updateBodyType}>
      <Select.Trigger class="w-[200px]">
        <Select.Value placeholder="Select body type" />
      </Select.Trigger>
      <Select.Content>
        {#each bodyTypes as type}
          <Select.Item value={type.value}>{type.label}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
  </div>
  <div class="flex-grow">
    <MonacoEditor
      value={request.body}
      language={selectedBodyType.value}
      onChange={updateBody}
      readOnly={false}
    />
  </div>
</div>

