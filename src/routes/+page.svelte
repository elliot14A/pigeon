<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Select from "$lib/components/ui/select";
  import { Textarea } from "$lib/components/ui/textarea";
</script>

<div class="flex flex-col h-screen bg-background text-foreground">
  <header class="p-4">
    <h1 class="text-2xl font-bold text-foreground">Pigeon</h1>
  </header>

  <main class="flex-1 p-4 overflow-auto">
    <div class="flex space-x-2 mb-4">
      <Select.Root portal={null}>
        <Select.Trigger class="w-[100px]">
          <Select.Value placeholder="Method" />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="GET">GET</Select.Item>
          <Select.Item value="POST">POST</Select.Item>
          <Select.Item value="PUT">PUT</Select.Item>
          <Select.Item value="DELETE">DELETE</Select.Item>
        </Select.Content>
      </Select.Root>
      <Input type="text" placeholder="Enter URL" class="flex-1" />
      <Button>Send</Button>
    </div>

    <div class="grid grid-cols-2 gap-4">
      <div class="space-y-4">
        <h2 class="text-lg font-semibold">Request</h2>
        <Textarea placeholder="Enter request body" />
      </div>
      <div class="space-y-4">
        <h2 class="text-lg font-semibold">Response</h2>
        <Textarea placeholder="Response will appear here" readonly />
      </div>
    </div>
  </main>
</div>
