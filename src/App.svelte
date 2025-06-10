<script>
  import Sidebar from './lib/Sidebar.svelte';
  import { writable } from 'svelte/store';
  const isExpanded = writable(false);
  let isWorkspace = true;
  function openWorkspace() {
    isWorkspace = !isWorkspace;
  }
  const urls = [
    'https://web.skola24.se/timetable/timetable-viewer/industritekniska.skola24.se/Hitachigymnasiet%20i%20V%C3%A4ster%C3%A5s/',
    'https://haldor.se',
    'https://evolando.se/',
  ];
</script>

<div class="flex h-full">
  <div
    class="fixed left-0 top-0 h-full"
    role="navigation"
    on:mouseenter={() => ($isExpanded = true)}
    on:mouseleave={() => ($isExpanded = false)}
  >
    <Sidebar isExpanded={$isExpanded} CreateWorkspacefunc={openWorkspace} />
  </div>
  <div
    class="flex w-full h-screen transition-all duration-300 ease-in-out"
    class:ml-72={$isExpanded}
    class:ml-18={!$isExpanded}
  >
    {#if isWorkspace}
      <div class="grid grid-cols-2 grid-rows-2 gap-1 w-full h-full">
        <iframe
          src={urls[0]}
          class="w-full h-full border-none col-span-1 row-span-2"
          title="Workspace Content 1"
        ></iframe>
        <iframe
          src={urls[1]}
          class="w-full h-full border-none col-span-1 row-span-1"
          title="Workspace Content 2"
        ></iframe>
        <iframe
          src={urls[2]}
          class="w-full h-full border-none col-span-1 row-span-1"
          title="Workspace Content 3"
        ></iframe>
      </div>
    {:else}
      <div class="flex items-center w-3/4 mx-auto flex-col text-center h-full justify-center">
        <h1 class="text-5xl font-bold text-white">Workspaces goes here.</h1>
        <h2 class="text-xl text-white mt-3">Select an environment from the menu to begin</h2>
      </div>
    {/if}
  </div>
</div>
