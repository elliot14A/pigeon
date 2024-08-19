<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Settings } from "lucide-svelte";
</script>

<header class="flex items-center justify-between p-4 bg-background">
  <h1 class="text-2xl font-bold text-foreground">Pigeon - AI Http Client</h1>
  <Button variant="ghost"><Settings /></Button>
</header>
