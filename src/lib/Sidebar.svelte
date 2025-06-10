<script lang="ts">
  import { LayoutDashboard, Undo2 } from '@lucide/svelte';
  export let isExpanded = false;
  export let CreateWorkspacefunc;
</script>

<div
  class="bg-neutral-950 flex flex-col items-center p-5 transition-all duration-300 ease-in-out {isExpanded
    ? 'w-72'
    : 'w-18'} h-full"
>
  {#each ['Swedish Layout', 'English Layout', 'Custom Layout'] as layout}
    <div class="flex w-full items-center h-10">
      <LayoutDashboard size="24" class="text-white mx-4" />
      <div class="overflow-hidden {isExpanded ? 'w-auto' : 'w-auto'}">
        <button class="text-white text-[1rem] whitespace-nowrap" on:click={CreateWorkspacefunc}
          >{layout}</button
        >
      </div>
    </div>
  {/each}
  <div class="mt-auto w-full flex flex-col items-center">
    <div class="bg-white w-full h-[0.2rem]"></div>
    <div class="flex w-full h-4">
      <!--Dont need this button when using on hover 
      <button class="flex items-center mt-4">
        <div class="text-white"></div>
        <Undo2 size="1.5rem" class="text-white ml-6" />
      </button> -->
    </div>
  </div>
</div>
