<script lang="ts">
  import { LayoutDashboard, Undo2 } from '@lucide/svelte';
  export let isExpanded = false;
  export let CreateWorkspacefunc;
</script>

<div
  class="flex flex-col items-center bg-neutral-50 p-5 shadow-xl transition-all duration-300 ease-in-out {isExpanded
    ? 'w-72'
    : 'w-16'} h-full"
>
  {#each ['Swedish Layout', 'Math Layout', 'Programming Layout'] as layout}
    <button class="flex h-10 w-full items-center" on:click={() => CreateWorkspacefunc(layout)}>
      <LayoutDashboard size="24" class="mx-4" />
      <div class="overflow-hidden {isExpanded ? 'w-auto' : 'w-auto'}">
        <div class="whitespace-nowrap text-[1rem]">{layout}</div>
      </div>
    </button>
  {/each}
  <div class="mt-auto flex w-full flex-col items-center">
    <div class="h-[0.15rem] w-full bg-black"></div>
  </div>
</div>
